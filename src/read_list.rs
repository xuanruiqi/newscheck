use std::fs::{read, File};
use std::io::ErrorKind;
use crate::feed::Entry;

const HASH_SIZE: usize = 16;

pub fn load_or_create(path: &str) -> std::io::Result<Vec<u8>> {
    match read(path) {
        Ok(data) => Ok(data),
        Err(e) => {
            match e.kind() {
                ErrorKind::NotFound => {
                    File::create(path)?;
                    Ok(Vec::new())
                },
                _ => Err(e), // Propagate other errors
            }
        }
    }
}

fn check_read(read_list: &Vec<u8>, entry: &Entry) -> bool {
    let entry_digest = entry.digest();
    let chunked = read_list.chunks(HASH_SIZE);
    for chunk in chunked {
        if chunk.len() == HASH_SIZE && chunk == entry_digest {
            return true;
        }
    }
    false
}

pub fn write_read_list(path: &str, read_list: Vec<u8>) -> std::io::Result<()> {
    std::fs::write(path, read_list)
}

/* fn read_entry(entry: &mut Entry, read_list: &mut Vec<u8>) -> () {
    if !entry.unread() {
        entry.mark_as_read();
        read_list.extend_from_slice(&entry.digest());
    }
} */

pub fn get_unread_entries(entries: &Vec<Entry>, read_list: &Vec<u8>) -> Vec<Entry> {
    entries.iter()
        .filter(|entry| !check_read(read_list, entry))
        .cloned()
        .collect()
}