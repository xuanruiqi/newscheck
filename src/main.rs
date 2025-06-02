mod feed;
use feed::{entries, print_entries};
use sysinfo::{System, get_current_pid};
use clap::Command;


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
        )
}

fn is_under_pacman() -> bool {
    let s = System::new_all();
    let curr = get_current_pid().unwrap();
    let parent = s.process(curr).unwrap().parent().unwrap();
    s.process(parent).unwrap().name() == "pacman"
}

fn print_feed() -> () {
    let entries = entries();
    match entries {
        Ok(entries) => print_entries(entries),
        Err(e) => eprintln!("Error fetching entries: {}", e),
    }
}

fn main() {
    let matches = cli().get_matches();
    match matches.subcommand() {
        Some(("list", _)) => {
            print_feed();
        },
        Some(_) => {
            println!("Subcommand not implemented yet.");
        },
        None => {
            println!("No subcommand");
        }
    }

    if !is_under_pacman() {
        println!("This program is not running under pacman.");
    }
    
}