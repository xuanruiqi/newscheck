use html2text::from_read;
use terminal_size::{terminal_size, Width};
use chrono::Local;
use crate::feed::Entry;

const DEFAULT_WIDTH: u16 = 80;

fn get_width() -> u16 {
    if let Some((Width(w), _)) = terminal_size() {
        std::cmp::min(w, DEFAULT_WIDTH)
    } else {
        DEFAULT_WIDTH // Default width if terminal size cannot be determined
    }
}

pub fn pretty_print_item(entry: &Entry) {
    let term_width = get_width();
    if let Some(t) = term::stdout().as_mut() {
        t.attr(term::Attr::Bold).unwrap();
        writeln!(t, "{}", entry.title).unwrap();
        t.reset().unwrap();
        writeln!(t, "{}", entry.timestamp.with_timezone(&Local)).unwrap();
        let formatted_body = from_read(entry.body.as_bytes(), term_width as usize).unwrap_or(entry.body.clone());
        writeln!(t, "{}", formatted_body).unwrap();        
    } else {
        eprintln!("Failed to get terminal output.");
    } 
}