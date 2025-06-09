mod feed;
mod read_list;
mod term;

use feed::{Entry, entries};
use sysinfo::{System, get_current_pid};
use clap::{crate_version, Parser, Subcommand, Args};
use read_list::{get_unread_entries, load_or_create};
use term::pretty_print_item;

const READ_LIST_PATH: &str = "readlist";

struct Config<'a> {
    raw: bool, // whether to print raw HTML
    read_list_path: &'a str,
    overwrite: bool, // whether to overwrite the read list
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
}

#[derive(Debug, Subcommand)]
enum SubCommand {
    #[command(about = "List the most recent news entries, including all read and unread items.")]
    List,
    #[command(about = "Check for unread news items.")]
    Check,
    #[command(about = "Read a specific news item.")]
    Read {
        num_item: Option<usize>
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

fn list_entries(entries: &Vec<Entry>, _conf: &Config) -> () {
    for (i, entry) in entries.iter().enumerate() {
        println!("{}: {} {}", i, entry.title, entry.timestamp);
    }
}

fn check_entries(entries: &Vec<Entry>, conf: &Config) -> () {
    match load_or_create(conf.read_list_path, conf.overwrite) {
        Ok(read_list) => {
            let unread_entries = get_unread_entries(entries, &read_list);
            if unread_entries.is_empty() {
                println!("There are no unread news items.");
            } else if unread_entries.len() == 1 {
                println!("There is 1 unread news item. Use \"newscheck read [# of news item]\" to read it."); 
            } else {
                println!("There are {} unread news items. Use \"newscheck read [# of news item]\" to read them.", unread_entries.len());
            }
        },
        Err(e) => {
            eprintln!("Error loading read list: {}", e);
        }
    }
}

fn read_entries(entries: &Vec<Entry>, read_item: usize, conf: &Config) -> () {
    match load_or_create(conf.read_list_path, conf.overwrite) {
        Ok(mut read_list) => {
            if let Some(entry) = entries.get(read_item) {
                if let Err(e) = pretty_print_item(entry, conf.raw) {
                    eprintln!("Error printing item: {}", e);
                }
                read_list.extend_from_slice(&entry.digest());
                if let Err(e) = read_list::write_read_list(conf.read_list_path, read_list) {
                    eprintln!("Error writing to read list: {}", e);
                }
            } else {
                eprintln!("No news found for index {}", read_item);
            }
        },
        Err(e) => {
            eprintln!("Error loading read list: {}", e);
        }
    }
}
fn main() {
    let cli = Cli::parse();
    let entries = entries();
    // let matches = cli().get_matches();
    let conf = Config {
        raw: cli.flags.raw,
        read_list_path: cli.flags.readlist_path.as_str(),
        overwrite: cli.flags.clear_readlist,
    };
    match entries {
        Ok(entries) => {
            match &cli.subcommand {
                SubCommand::List => {
                    list_entries(&entries, &conf);
                },
                SubCommand::Check => {
                    check_entries(&entries, &conf);
                },
                SubCommand::Read { num_item } => {
                    let read_item: usize = num_item.unwrap_or(0);
                    read_entries(&entries, read_item, &conf);
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