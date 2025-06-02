mod feed;
mod read_list;
use std::fs::read;

use feed::{Entry, entries};
use sysinfo::{System, get_current_pid};
use clap::{Command, arg};
use read_list::{get_unread_entries, load_or_create};

const READ_LIST_PATH: &str = "readlist";

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

fn list_entries(entries: &Vec<Entry>) -> () {
    for (i, entry) in entries.iter().enumerate() {
        println!("{}: {} {}", i, entry.title, entry.timestamp);
    }
}

fn check_entries(entries: &Vec<Entry>) -> () {
    match load_or_create(READ_LIST_PATH) {
        Ok(read_list) => {
            let unread_entries = get_unread_entries(entries, &read_list);
            if unread_entries.is_empty() {
                println!("There are no unread news items.");
            } else {
                println!("There are {} unread news items. Use \"newscheck read [# of news item]\" to read them.", unread_entries.len());
            }
        },
        Err(e) => {
            eprintln!("Error loading read list: {}", e);
        }
    }
}

fn read_entries(entries: &Vec<Entry>, read_item: usize) -> () {
    match load_or_create(READ_LIST_PATH) {
        Ok(mut read_list) => {
            if let Some(entry) = entries.get(read_item) {
                println!("{}", entry);
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
    match entries {
        Ok(entries) => {
            match matches.subcommand() {
                Some(("list", _)) => {
                    list_entries(&entries);
                },
                Some(("check", _)) => {
                    check_entries(&entries);
                },
                Some(("read", sub_matches)) => {
                    let read_item: usize = sub_matches.get_one::<String>("news_item").map_or("", |v| v).parse().unwrap_or(0);
                    read_entries(&entries, read_item);
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