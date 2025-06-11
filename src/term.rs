use html2text::from_read;
use terminal_size::{terminal_size, Width};
use chrono::Local;
use crate::feed::Entry;
use which::which;
use std::{io::Write, process::{Command, Stdio}};

const DEFAULT_WIDTH: usize = 80;

macro_rules! format_time {
    ($t:expr, $local:expr) => {
        format!(
            "{}",
            $t.with_timezone($local).format("%a, %d %b %Y %H:%M:%S %z")
        )
    };
}

pub fn print_error(msg: &str) {
    if let Some(mut t_ref) = term::stderr() {
        let t = t_ref.as_mut();
        t.fg(term::color::RED).unwrap_or(());
        write!(t, "ERROR: ").unwrap_or(());
        t.reset().unwrap_or(());
        writeln!(t, "{}", msg).unwrap_or(());
    } else {
        eprintln!("ERROR: {}", msg);
    }
}

pub fn print_warning(msg: &str) {
    if let Some(mut t_ref) = term::stderr() {
        let t = t_ref.as_mut();
        t.fg(term::color::YELLOW).unwrap_or(());
        write!(t, "WARN: ").unwrap_or(());
        t.reset().unwrap_or(());
        writeln!(t, "{}", msg).unwrap_or(());
    } else {
        eprintln!("WARN: {}", msg);
    }
}

pub fn print_pacman(msg: &str) {
    if let Some(mut t_ref) = term::stderr() {
        let t = t_ref.as_mut();
        t.fg(term::color::YELLOW).unwrap_or(());
        write!(t, ":: newsread: ").unwrap_or(());
        t.reset().unwrap_or(());
        writeln!(t, "{}", msg).unwrap_or(());
    } else {
        eprintln!(":: newsread: {}", msg);
    }
}

fn get_width() -> usize {
    if let Some((Width(w), _)) = terminal_size() {
        w as usize
    } else {
        DEFAULT_WIDTH // Default width if terminal size cannot be determined
    }
}

pub fn pretty_print_title(id: usize, entry: &Entry) {
    let term_width = get_width();
    let pad = term_width.saturating_sub(entry.title.len() + 4);
    let ts = format_time!(entry.timestamp, &Local);
    println!("{}: {} {:>width$}", id, entry.title, ts, width = pad);
}

pub fn page_item(entry: &Entry, html_raw: bool, pager: &str) {
    if let Err(_) = which(pager) {
        pretty_print_item(entry, html_raw); // Fallback to standard pretty printing
    }
    let mut pager = Command::new(pager);
    if let Ok(mut proc) = pager.stdin(Stdio::piped()).spawn() {
        let ts = format_time!(entry.timestamp, &Local);
        let body = if html_raw { entry.body.clone() } else { from_read(entry.body.as_bytes(), DEFAULT_WIDTH).unwrap_or(entry.body.clone()) };
        proc.stdin.take().expect("Bad pipe to pager").write_fmt(format_args!("# {}\n{}\n{}\n", entry.title, ts, body))
            .expect("Broken pipe to pager");
        proc.wait().expect("Pager process did not exit cleanly");
    } else {
        pretty_print_item(entry, html_raw);
    }
}

pub fn pretty_print_item(entry: &Entry, html_raw: bool) {
    let term_width = get_width();
    let body = if html_raw { entry.body.clone() } else { from_read(entry.body.as_bytes(), term_width).unwrap_or(entry.body.clone()) };
    let ts = format_time!(entry.timestamp, &Local);
    if let Some(mut t_ref) = term::stdout() {
        let t = t_ref.as_mut();
        t.attr(term::Attr::Bold).unwrap_or(());
        writeln!(t, "{}", entry.title).unwrap_or(());
        t.reset().unwrap_or(());
        writeln!(t, "{}", ts).unwrap_or(());
        writeln!(t, "{}", body).unwrap_or(());
    } else {
        println!("{}", entry.title);
        println!("{}", ts);
        println!("{}", body);
    }
}

pub fn prompt(prompt_str: &str) -> Result<bool, std::io::Error> {
    let mut input = String::new();
    print!("{}", prompt_str);
    std::io::stdout().flush()?;
    std::io::stdin().read_line(&mut input)?;
    match input.trim().to_lowercase().as_str() {
        "y" | "yes" | "" => Ok(true),
        "n" | "no" => Ok(false),
        _ => {
            print_error("Invalid input. Please enter 'y' or 'n'.");
            prompt(prompt_str) // Recursive call for valid input
        }
    }
}