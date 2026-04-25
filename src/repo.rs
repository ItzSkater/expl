use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use crate::config::{INDEX_CACHE, REPO_INDEX_URL};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Package {
    pub version: String,
    pub description: String,
    pub url: String,
    pub arch: Vec<String>,
}

pub type Index = HashMap<String, Package>;

fn cache_path() -> PathBuf {
    PathBuf::from(INDEX_CACHE.replace("~", &std::env::var("HOME").unwrap_or_default()))
}

pub async fn sync_index() {
    println!(":: Syncing package index...");
    let resp = reqwest::get(REPO_INDEX_URL).await;
    match resp {
        Ok(r) => {
            let text = r.text().await.unwrap_or_default();
            let path = cache_path();
            fs::create_dir_all(path.parent().unwrap()).ok();
            fs::write(&path, &text).ok();
            println!(":: Index updated successfully.");
        }
        Err(e) => eprintln!("error: failed to fetch index: {}", e),
    }
}

pub fn load_index() -> Option<Index> {
    let path = cache_path();
    let content = fs::read_to_string(path).ok()?;
    serde_json::from_str(&content).ok()
}

pub fn find_package(name: &str) -> Option<Package> {
    let index = load_index()?;
    index.get(name).cloned()
}
