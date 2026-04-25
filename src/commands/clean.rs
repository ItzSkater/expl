use std::fs;
use std::path::PathBuf;
use crate::config::CACHE_DIR;

pub fn run() {
    let cache = PathBuf::from(
        CACHE_DIR.replace("~", &std::env::var("HOME").unwrap_or_default()),
    );
    if cache.exists() {
        fs::remove_dir_all(&cache).unwrap();
        println!(":: Cache cleared");
    } else {
        println!(":: Cache is already empty");
    }
}
