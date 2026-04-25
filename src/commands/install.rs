use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::time::Duration;
use tokio::io::AsyncWriteExt;

use crate::config::INSTALL_DIR;
use crate::repo;
use crate::tips::random_tip;

pub async fn run(pkg: &str) {
    println!(":: Looking for {}...", pkg);

    // Синкаем индекс если нужно
    let package = match repo::find_package(pkg) {
        Some(p) => p,
        None => {
            // Пробуем обновить индекс и поискать снова
            repo::sync_index().await;
            match repo::find_package(pkg) {
                Some(p) => p,
                None => {
                    eprintln!("error: package '{}' not found", pkg);
                    return;
                }
            }
        }
    };

    println!(":: Found {} v{}", pkg, package.version);
    println!(":: {}", package.description);
    println!();

    // Качаем с прогресс баром
    let client = reqwest::Client::new();
    let resp = match client.get(&package.url).send().await {
        Ok(r) => r,
        Err(e) => {
            eprintln!("error: download failed: {}", e);
            return;
        }
    };

    let total = resp.content_length().unwrap_or(0);
    let pb = ProgressBar::new(total);
    pb.set_style(
        ProgressStyle::with_template(
            ":: Downloading [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})",
        )
        .unwrap()
        .progress_chars("=>-"),
    );

    // Показываем советы каждые 3 секунды в отдельном потоке
    let pb_clone = pb.clone();
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(Duration::from_secs(3)).await;
            if pb_clone.is_finished() {
                break;
            }
            pb_clone.println(format!("   \x1b[90m{}\x1b[0m", random_tip()));
        }
    });

    // Сохраняем файл
    let install_dir = PathBuf::from(
        INSTALL_DIR.replace("~", &std::env::var("HOME").unwrap_or_default()),
    );
    fs::create_dir_all(&install_dir).ok();

    let filename = format!("{}.AppImage", pkg);
    let filepath = install_dir.join(&filename);
    let mut file = tokio::fs::File::create(&filepath).await.unwrap();

    let mut stream = resp.bytes_stream();
    while let Some(chunk) = stream.next().await {
        let chunk = chunk.unwrap();
        pb.inc(chunk.len() as u64);
        file.write_all(&chunk).await.unwrap();
    }

    pb.finish_and_clear();

    // chmod +x
    let mut perms = fs::metadata(&filepath).unwrap().permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&filepath, perms).unwrap();

    println!(":: {} installed to {}/{}", pkg, INSTALL_DIR, filename);
    println!(":: Run it with: {}", pkg);
}
