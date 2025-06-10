use html2text::from_read;
use terminal_size::{terminal_size, Width};
use chrono::Local;
use crate::feed::Entry;

const DEFAULT_WIDTH: usize = 80;

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
    // eprintln!("Pad length: {}", pad);
    let ts = format!(
        "{}",
        entry.timestamp.with_timezone(&Local).format("%a, %d %b %Y %H:%M:%S %z")
    );
    println!("{}: {} {:>width$}", id, entry.title, ts, width = pad);
}

pub fn pretty_print_item(entry: &Entry, html_raw: bool) -> Result<(), std::io::Error> {
    let term_width = get_width();
    let mut t_ref = term::stdout();
    let t = t_ref.as_mut()
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::Other, "Failed to get terminal output"))?;
    t.attr(term::Attr::Bold)?;
    writeln!(t, "{}", entry.title)?;
    t.reset()?;
    writeln!(t, "{}", entry.timestamp.with_timezone(&Local))?;
    if html_raw {
        writeln!(t, "{}", entry.body)?;
    } else {
        let formatted_body = from_read(entry.body.as_bytes(), term_width).unwrap_or(entry.body.clone());
        writeln!(t, "{}", formatted_body)?;
    }
    Ok(())
}