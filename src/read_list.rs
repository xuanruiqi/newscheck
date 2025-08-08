use std::fs::{read, File};
use std::io::ErrorKind;
use crate::feed::Entry;
use crate::term::print_warning;

const HASH_SIZE: usize = 16;

pub fn load_or_create(path: &str, overwrite: bool) -> std::io::Result<Vec<u8>> {
    if overwrite {
        File::create(path)?;
        return Ok(Vec::new());
    }
    match read(path) {
        Ok(data) => Ok(data),
        Err(e) => {
            if let ErrorKind::NotFound = e.kind() {
                File::create(path)?;
                Ok(Vec::new())
            } else {
                Err(e)
            }
        }
    }
}

fn check_read(read_list: &Vec<u8>, entry: &Entry) -> bool {
    let entry_digest = entry.digest();
    read_list.chunks(HASH_SIZE).any(|chunk| {
        chunk.len() == HASH_SIZE && chunk == &entry_digest
    })
}

pub fn write_read_list(path: &str, read_list: Vec<u8>) -> std::io::Result<()> {
    std::fs::write(path, read_list)
        .inspect_err(|e| {
            if e.kind() == ErrorKind::PermissionDenied {
                print_warning("Could not write to the read list. You must be in the \"newscheck\" group to do this.");
            }
        })
}

pub fn add_to_read_list(read_list: &mut Vec<u8>, entry: &Entry) {
    if !read_list.chunks(16).any(|d| d == &entry.digest()) {
        read_list.extend_from_slice(&entry.digest());
    }
}

pub fn get_unread_entries(entries: &Vec<Entry>, read_list: &Vec<u8>) -> Vec<Entry> {
    entries.iter()
        .filter(|entry| !check_read(read_list, entry))
        .cloned()
        .collect()
}