mod feed;
mod read_list;
mod term;

use core::fmt;
use std::process::ExitCode;

use feed::{Entry, entries};
use sysinfo::{System, get_current_pid};
use clap::{crate_version, Parser, Subcommand, Args, CommandFactory};
use clap_complete::{generate, Shell};
use read_list::{get_unread_entries, load_or_create};
use term::{pretty_print_item, print_error, print_warning, print_pacman, prompt};

const READ_LIST_PATH: &str = "readlist";
const FEED_ENDPOINT: &str = "https://archlinux.org/feeds/news/";

macro_rules! page_or_pp {
    ($entry:expr, $conf:expr) => {
        if let Some(pager) = $conf.pager {
            term::page_item($entry, $conf.raw, pager);
        } else {
            term::pretty_print_item($entry, $conf.raw);
        }
    };
}

#[derive(Debug, Clone, Copy)]
struct Blocked;

impl fmt::Display for Blocked {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "pacman update blocked due to unread news items")
    }
}

impl std::error::Error for Blocked {}

struct Config<'a> {
    endpoint: &'a str,
    raw: bool, // whether to print raw HTML
    read_list_path: &'a str,
    overwrite: bool, // whether to overwrite the read list
    pager: Option<&'a str>, // optional pager command
    hook: bool, // whether running as pacman hook
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
    pager: Option<String>,
    #[clap(long, global = true, help = "Run as if this is a pacman hook (for debugging purposes)", hide = true)]
    debug_pacman: bool
}

#[derive(Debug, Subcommand)]
enum SubCommand {
    #[command(about = "List the most recent news entries, including all read and unread items")]
    List {
        #[clap(long, global = false, help = "List news items in reverse order")]
        reverse: bool,
        #[clap(long = "unread", global = false, help = "Print raw HTML instead of formatted text")]
        only_unread: bool
    },
    #[command(about = "Check for unread news items")]
    Check,
    #[command(about = "Read a specific news item")]
    Read {
        num_item: Option<usize>,
        #[clap(long = "all", global = false, help = "Mark all news items as read without printing")]
        all: bool
    },
    #[command(about = "Generate shell completions for newscheck (see `newscheck completions --help` for more info)",
              after_help = r#"You should not need to run this command manually. During package installation,
the completions will be generated automatically and installed to the appropriate location
for your shell. Shell completion should work out of the box for bash, zsh, and fish.

If for any reason you need to do so manually, you can run this command to
generate completions for your shell of choice. For example, to generate
completions for bash, you can run:

$ newscheck completions bash > ~/.bash_completion.d/newscheck.bash

or for zsh:

$ newscheck completions zsh > ~/.zsh/site_functions/_newscheck"#)]
    Completions {
        #[clap(value_enum, help = "Shell to generate completions for")]
        shell: Shell
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

fn list_entries(entries: &Vec<Entry>, conf: &Config, reverse: bool, unread: bool) -> Result<(), Box<dyn std::error::Error>> {
    let entries_to_list: &Vec<Entry> = if unread {
        let read_list = load_or_create(conf.read_list_path, conf.overwrite)?;
        &get_unread_entries(entries, &read_list)
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
    Ok(())
}

fn check_entries(entries: &Vec<Entry>, conf: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let mut read_list = load_or_create(conf.read_list_path, conf.overwrite)?;
    let unread_entries = get_unread_entries(entries, &read_list);
    if unread_entries.is_empty() {
        println!("There are no unread news items.");
        Ok(())
    } else if unread_entries.len() == 1 {
        if conf.hook {
            print_pacman(&format!("stopping upgrade to print news"));
        }
        pretty_print_item(&unread_entries[0], conf.raw);
        read_list::add_to_read_list(&mut read_list, &unread_entries[0]);
        read_list::write_read_list(conf.read_list_path, read_list)?;
        if conf.hook {
            print_pacman("you can re-run your upgrade command to complete the upgrade.");
            Err(Box::<dyn std::error::Error>::from(Blocked))?;
        }
        Ok(())
    } else {
        println!(
            "There are {} unread news items. Use \"newscheck read [# of news item]\" to read them.",
            unread_entries.len()
        );
        if conf.hook {
            print_pacman("run `newscheck read` to read the news before proceeding with the upgrade.");
            Err(Box::<dyn std::error::Error>::from(Blocked))?;
        }
        Ok(())
    }
}

fn read_entries(entries: &Vec<Entry>, read_item: usize, conf: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let mut read_list = load_or_create(conf.read_list_path, conf.overwrite)?;
    let entry = entries.get(read_item).ok_or(format!("there is no news item with index {}", read_item))?;
    page_or_pp!(entry, conf);
    read_list::add_to_read_list(&mut read_list, entry);
    read_list::write_read_list(conf.read_list_path, read_list)?;
    Ok(())
}

fn mark_all_read(entries: &Vec<Entry>, conf: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let mut buf: Vec<u8> = Vec::new();
    for entry in entries {
        buf.extend_from_slice(&entry.digest());
    }
    read_list::write_read_list(conf.read_list_path, buf)?;
    Ok(())
}

fn read_all_unread(entries: &Vec<Entry>, conf: &Config) -> Result<(), Box<dyn std::error::Error>> {
    if atty::isnt(atty::Stream::Stdout) {
        print_warning("interactive mode is not available. `newscheck read` without arguments is meant to be used interactively only.");
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "not a TTY"
        )));
    }
    let mut read_list = load_or_create(conf.read_list_path, conf.overwrite)?;
    let unread = get_unread_entries(entries, &read_list);
    let mut it = unread.iter().peekable();
    while let Some(entry) = it.next() {
        page_or_pp!(entry, conf);
        read_list::add_to_read_list(&mut read_list, entry);
        if it.peek().is_some() {
            if !prompt("Read next news item? (y/n) ")? { break };
        } else {
            println!("All unread news items have been read.");
        }
    }
    read_list::write_read_list(conf.read_list_path, read_list)?;
    Ok(())
}

fn app() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let pager = cli.flags.pager.clone().or_else(|| {
        std::env::var("NEWSCHECK_PAGER").ok()
    });
    let conf = Config {
        endpoint: cli.flags.url.as_str(),
        raw: cli.flags.raw,
        read_list_path: cli.flags.readlist_path.as_str(),
        overwrite: cli.flags.clear_readlist,
        pager: pager.as_deref(),
        hook: is_under_pacman() || cli.flags.debug_pacman
    };
    let entries = entries(conf.endpoint)?;
    if entries.is_empty() {
        print_warning("doing nothing because there are no news items found");
        return Ok(());
    }
    match &cli.subcommand {
        SubCommand::List { reverse, only_unread  } => {
            list_entries(&entries, &conf, *reverse, *only_unread)?;
        },
        SubCommand::Check => {
            check_entries(&entries, &conf)?;
        },
        SubCommand::Read { num_item, all } => {
            if *all {
                mark_all_read(&entries, &conf)?;
            } else if let Some(num) = num_item {
                read_entries(&entries, *num, &conf)?;
            } else {
                read_all_unread(&entries, &conf)?;
            }
        },
        SubCommand::Completions { shell } => {
            let mut cmd = Cli::command();
            generate(*shell, &mut cmd, "newscheck", &mut std::io::stdout());
            return Ok(());
        }
    }
    Ok(())
}

fn main() -> std::process::ExitCode {
    if let Err(e) = app() {
        e.as_ref()
            .downcast_ref::<Blocked>()
            .map_or_else(|| {
                print_error(&format!("{}", e));
            }, |_blocked| {
                ()
            });
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}