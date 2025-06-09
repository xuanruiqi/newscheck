mod feed;
mod read_list;
mod term;

use feed::{Entry, entries};
use sysinfo::{System, get_current_pid};
use clap::{Command, arg};
use read_list::{get_unread_entries, load_or_create};
use term::pretty_print_item;

const READ_LIST_PATH: &str = "readlist";

struct CliArgs {
    raw: bool,
}

fn cli() -> Command {
    Command::new("newscheck")
        .about("Another Arch Linux news reader")
        .subcommand(
            Command::new("list")
                .about("Print the recent news items")
        )
        .subcommand(
            Command::new("check")
                .about("Check for unread news items")
        )
        .subcommand(
            Command::new("read")
                .about("Read a specific news item")
                .arg(arg!([news_item] "The number of the news item to read"))
        )
        .arg(arg!(-r --raw  "Do not format HTML in news items"))
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

fn list_entries(entries: &Vec<Entry>, _args: &CliArgs) -> () {
    for (i, entry) in entries.iter().enumerate() {
        println!("{}: {} {}", i, entry.title, entry.timestamp);
    }
}

fn check_entries(entries: &Vec<Entry>, _args: &CliArgs) -> () {
    match load_or_create(READ_LIST_PATH) {
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

fn read_entries(entries: &Vec<Entry>, read_item: usize, args: &CliArgs) -> () {
    match load_or_create(READ_LIST_PATH) {
        Ok(mut read_list) => {
            if let Some(entry) = entries.get(read_item) {
                if let Err(e) = pretty_print_item(entry, args.raw) {
                    eprintln!("Error printing item: {}", e);
                }
                read_list.extend_from_slice(&entry.digest());
                if let Err(e) = read_list::write_read_list(READ_LIST_PATH, read_list) {
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
    let entries = entries();
    let matches = cli().get_matches();
    let args = CliArgs {
        raw: matches.get_flag("raw"),
    };
    match entries {
        Ok(entries) => {
            match matches.subcommand() {
                Some(("list", _)) => {
                    list_entries(&entries, &args);
                },
                Some(("check", _)) => {
                    check_entries(&entries, &args);
                },
                Some(("read", sub_matches)) => {
                    let read_item: usize = sub_matches.get_one::<String>("news_item").map_or("", |v| v).parse().unwrap_or(0);
                    read_entries(&entries, read_item, &args);
                },
                _ => println!("Subcommand not implemented yet."),
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