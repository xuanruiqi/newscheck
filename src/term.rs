use html2text::from_read;
use terminal_size::{terminal_size, Width};
use chrono::Local;
use crate::feed::Entry;

const DEFAULT_WIDTH: usize = 80;

macro_rules! format_time {
    ($t:expr, $local:expr) => {
        format!(
            "{}",
            $t.with_timezone($local).format("%a, %d %b %Y %H:%M:%S %z")
        )
    };
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