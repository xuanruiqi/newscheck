use std::error::Error;
use rss::{Channel, Item};
use chrono::{DateTime, Utc};
use std::fmt;
use std::fmt::{Display, Formatter};
use md5::{Md5, Digest};

const ENDPOINT: &str = "https://archlinux.org/feeds/news/";

#[derive(Debug, Hash, Clone)]
pub struct Entry {
    pub title: String,
    pub body: String,
    pub timestamp: DateTime<Utc>,
    is_read: bool,
}

impl Display for Entry {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}\n{}\n{}", self.title, self.timestamp, self.body)
    }
}

impl Entry {
    fn new(title: String, body: String, timestamp: DateTime<Utc>) -> Self {
        Entry {
            title,
            body,
            timestamp,
            is_read: false,
        }
    }

/*     pub fn mark_as_read(&mut self) {
        self.is_read = true;
    }

    pub fn unread(&self) -> bool {
        !self.is_read
    } */

    fn from_rss_item(item: &Item) -> Result<Self, Box<dyn Error>> {
        let title = item.title().ok_or("Cannot get title")?;
        let body = item.description().ok_or("Cannot get body")?;
        let timestamp = DateTime::parse_from_rfc2822(
            item.pub_date().ok_or("Cannot get date")?)?;
        Ok(Entry::new(
            title.to_string(),
            body.to_string(),
            timestamp.with_timezone(&Utc),
        ))
    }

    pub fn digest(&self) -> [u8; 16] {
        let mut hasher = Md5::new();
        hasher.update(self.title.as_bytes());
        hasher.update(self.timestamp.to_rfc3339().as_bytes());
        hasher.finalize().into()
    }
}

pub fn entries() -> Result<Vec<Entry>, Box<dyn Error>> {
    let body = reqwest::blocking::get(ENDPOINT)?.bytes()?;
    let ch = Channel::read_from(&body[..])?;
    let entries = ch.items().iter().map(|item|
        Entry::from_rss_item(item)).collect::<Result<Vec<Entry>, _>>()?;
    Ok(entries)
}