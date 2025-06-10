mod feed;
mod read_list;
mod term;

use feed::{Entry, entries};
use sysinfo::{System, get_current_pid};
use clap::{crate_version, Parser, Subcommand, Args};
use read_list::{get_unread_entries, load_or_create};
use term::{pretty_print_item, page_item};

const READ_LIST_PATH: &str = "readlist";
const FEED_ENDPOINT: &str = "https://archlinux.org/feeds/news/";

macro_rules! handle_error {
    ($e:expr, $msg:literal) => {
        if let Err(e) = $e {
            eprintln!("{}: {}", $msg, e);
        }
    };
}

macro_rules! with_read_list {
    ($conf:expr, $action:expr) => {
        match load_or_create($conf.read_list_path, $conf.overwrite) {
            Ok(read_list) => {
                $action(read_list);
            },
            Err(e) => {
                eprintln!("Error loading read list: {}", e);
            }
        }
    };
}

struct Config<'a> {
    endpoint: &'a str,
    raw: bool, // whether to print raw HTML
    read_list_path: &'a str,
    overwrite: bool, // whether to overwrite the read list
    pager: Option<&'a str>, // optional pager command
}

#[derive(Debug, Parser)]
#[command(version = crate_version!(), about = "Another Arch Linux news reader")]
#[command(propagate_version = true)]
struct Cli {
    #[clap(flatten)]
    flags: Flags,
    #[clap(subcommand)]
    subcommand: SubCommand
}

#[derive(Debug, Args)]
struct Flags {
    #[clap(short, long, global = true, help="Print raw HTML instead of formatted text")]
    raw: bool, // whether to print raw HTML
    #[clap(long, global = true, help="Clear the read list file")]
    clear_readlist: bool, // whether to clear the read list
    #[clap(long = "file", short = 'f', global = true, help="Path to the read list file", default_value = READ_LIST_PATH)]
    readlist_path: String, // path to the read list file
    #[clap(long, global = true, help="Endpoint for the news feed", default_value = FEED_ENDPOINT)]
    url: String,
    #[clap(short, long, global = true, help = "Use a pager to display news items")]
    pager: Option<String>
}

#[derive(Debug, Subcommand)]
enum SubCommand {
    #[command(about = "List the most recent news entries, including all read and unread items.")]
    List {
        #[clap(long, global = false, help = "List news items in reverse order")]
        reverse: bool,
        #[clap(long = "unread", global = false, help = "Print raw HTML instead of formatted text")]
        only_unread: bool
    },
    #[command(about = "Check for unread news items.")]
    Check,
    #[command(about = "Read a specific news item.")]
    Read {
        num_item: Option<usize>,
        #[clap(long = "all", global = false, help = "Mark all news items as read without printing")]
        all: bool
    }
}

fn is_under_pacman() -> bool {
    let s = System::new_all();
    let curr_pid = get_current_pid().unwrap_or(0.into());
    let parent = s.process(curr_pid);
    match parent {
        Some(proc) => proc.name() == "pacman",
        None => false,
    }
}

fn list_entries(entries: &Vec<Entry>, conf: &Config, reverse: bool, unread: bool) -> () {
    let entries_to_list = if unread {
        match load_or_create(conf.read_list_path, conf.overwrite) {
            Ok(read_list) => &get_unread_entries(entries, &read_list),
            Err(e) => {
                eprintln!("Error loading read list: {}", e);
                entries}
        }
    } else {
        entries
    };
    if reverse {
        for (i, entry) in entries_to_list.iter().enumerate().rev() {
            term::pretty_print_title(i, entry);
        }
    } else {
        for (i, entry) in entries_to_list.iter().enumerate() {
            term::pretty_print_title(i, entry);
        }
    }

}

fn check_entries(entries: &Vec<Entry>, conf: &Config) -> () {
    with_read_list!(conf, |read_list| {
        let unread_entries = get_unread_entries(entries, &read_list);
        if unread_entries.is_empty() {
            println!("There are no unread news items.");
        } else if unread_entries.len() == 1 {
            pretty_print_item(&unread_entries[0], conf.raw);
        } else {
            println!(
                "There are {} unread news items. Use \"newscheck read [# of news item]\" to read them.",
                unread_entries.len()
            );
        }
    });
}

fn read_entries(entries: &Vec<Entry>, read_item: usize, conf: &Config) -> () {
    with_read_list!(conf, |read_list| {
        if let Some(entry) = entries.get(read_item) {
            if let Some(pager) = conf.pager { 
                page_item(entry, conf.raw, pager);
            } else { pretty_print_item(entry, conf.raw) };
            handle_error!(
                read_list::add_and_save(conf.read_list_path, read_list, entry),
                "Error writing to read list"
            );
        } else {
            eprintln!("No news found for index {}", read_item);
        }
    });
}

fn mark_all_read(entries: &Vec<Entry>, conf: &Config) -> () {
    let mut buf: Vec<u8> = Vec::new();
    for entry in entries {
        buf.extend_from_slice(&entry.digest());
    }
    handle_error!(
        read_list::write_read_list(conf.read_list_path, buf),
        "Error writing to read list"
    );
}

fn main() {
    let cli = Cli::parse();
    let conf = Config {
        endpoint: cli.flags.url.as_str(),
        raw: cli.flags.raw,
        read_list_path: cli.flags.readlist_path.as_str(),
        overwrite: cli.flags.clear_readlist,
        pager: cli.flags.pager.as_deref(),
    };
    let entries = entries(conf.endpoint);
    match entries {
        Ok(entries) => {
            match &cli.subcommand {
                SubCommand::List { reverse, only_unread  } => {
                    list_entries(&entries, &conf, *reverse, *only_unread);
                },
                SubCommand::Check => {
                    check_entries(&entries, &conf);
                },
                SubCommand::Read { num_item, all } => {
                    if *all {
                        mark_all_read(&entries, &conf);
                    } else if let Some(num) = num_item {
                        read_entries(&entries, *num, &conf);
                    } else {
                        eprintln!("Not implemented yet.");
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Error fetching news entries: {}", e);
            return;
        }
    }

    if !is_under_pacman() {
        println!("This program is not running under pacman.");
    }
}