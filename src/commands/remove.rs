use std::fs;
use std::path::PathBuf;
use crate::config::INSTALL_DIR;

pub fn run(pkg: &str) {
    let install_dir = PathBuf::from(
        INSTALL_DIR.replace("~", &std::env::var("HOME").unwrap_or_default()),
    );
    let filepath = install_dir.join(format!("{}.AppImage", pkg));

    if filepath.exists() {
        fs::remove_file(&filepath).unwrap();
        println!(":: {} removed", pkg);
    } else {
        eprintln!("error: package '{}' is not installed", pkg);
    }
}
