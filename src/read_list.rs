use std::fs::{read, File};
use std::io::ErrorKind;
use crate::feed::Entry;

const HASH_SIZE: usize = 16;

pub fn load_or_create(path: &str, overwrite: bool) -> std::io::Result<Vec<u8>> {
    if overwrite {
        File::create(path)?;
        return Ok(Vec::new());
    }
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

pub fn add_and_save(path: &str, read_list: Vec<u8>, entry: &Entry) -> std::io::Result<()> {
    if !read_list.chunks(16).any(|d| d == &entry.digest()) {
        let mut new_read_list = read_list.clone();
        new_read_list.extend_from_slice(&entry.digest());
        std::fs::write(path, read_list)
    } else {
        Ok(())
    }
}

pub fn get_unread_entries(entries: &Vec<Entry>, read_list: &Vec<u8>) -> Vec<Entry> {
    entries.iter()
        .filter(|entry| !check_read(read_list, entry))
        .cloned()
        .collect()
}