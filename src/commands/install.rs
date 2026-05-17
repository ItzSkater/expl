use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::process::Command;
use std::time::Duration;
use tokio::io::AsyncWriteExt;

use crate::config::INSTALL_DIR;
use crate::repo;
use crate::tips::random_tip;

enum PackageFormat {
    AppImage,
    TarGz,
    Deb,
    Rpm,
    Unknown,
}

fn detect_format(url: &str) -> PackageFormat {
    if url.ends_with(".AppImage") || url.ends_with(".appimage") {
        PackageFormat::AppImage
    } else if url.ends_with(".tar.gz") || url.ends_with(".tgz") {
        PackageFormat::TarGz
    } else if url.ends_with(".deb") {
        PackageFormat::Deb
    } else if url.ends_with(".rpm") {
        PackageFormat::Rpm
    } else {
        PackageFormat::Unknown
    }
}

fn install_appimage(filepath: &PathBuf, pkg: &str, install_dir: &PathBuf) {
    let dest = install_dir.join(format!("{}.AppImage", pkg));
    fs::copy(filepath, &dest).unwrap();
    let mut perms = fs::metadata(&dest).unwrap().permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&dest, perms).unwrap();
    println!(":: {} installed to {}", pkg, dest.display());
    println!(":: Run it with: {}", pkg);
}

fn install_targz(filepath: &PathBuf, pkg: &str, install_dir: &PathBuf) {
    let tmp_dir = PathBuf::from(format!("/tmp/expl-{}", pkg));
    fs::create_dir_all(&tmp_dir).ok();

    let status = Command::new("bsdtar")
        .args([
            "-xf",
            filepath.to_str().unwrap(),
            "-C",
            tmp_dir.to_str().unwrap(),
        ])
        .status();

    match status {
        Ok(s) if s.success() => {
            let binary = find_binary(&tmp_dir, pkg);
            match binary {
                Some(bin) => {
                    let dest = install_dir.join(pkg);
                    fs::copy(&bin, &dest).unwrap();
                    let mut perms = fs::metadata(&dest).unwrap().permissions();
                    perms.set_mode(0o755);
                    fs::set_permissions(&dest, perms).unwrap();
                    println!(":: {} installed to {}", pkg, dest.display());
                    println!(":: Run it with: {}", pkg);
                }
                None => eprintln!("error: could not find binary in archive"),
            }
        }
        _ => eprintln!("error: bsdtar failed, is it installed?"),
    }

    fs::remove_dir_all(&tmp_dir).ok();
}

fn install_deb(filepath: &PathBuf, pkg: &str, install_dir: &PathBuf) {
    let tmp_dir = PathBuf::from(format!("/tmp/expl-{}", pkg));
    fs::create_dir_all(&tmp_dir).ok();

    let status = Command::new("bsdtar")
        .args([
            "-xf",
            filepath.to_str().unwrap(),
            "-C",
            tmp_dir.to_str().unwrap(),
        ])
        .status();

    match status {
        Ok(s) if s.success() => {
            let data_tar = find_data_tar(&tmp_dir);
            match data_tar {
                Some(tar) => {
                    let data_dir = tmp_dir.join("data");
                    fs::create_dir_all(&data_dir).ok();
                    Command::new("bsdtar")
                        .args([
                            "-xf",
                            tar.to_str().unwrap(),
                            "-C",
                            data_dir.to_str().unwrap(),
                        ])
                        .status()
                        .ok();

                    let binary = find_binary_in_usr(&data_dir, pkg);
                    match binary {
                        Some(bin) => {
                            let dest = install_dir.join(pkg);
                            fs::copy(&bin, &dest).unwrap();
                            let mut perms = fs::metadata(&dest).unwrap().permissions();
                            perms.set_mode(0o755);
                            fs::set_permissions(&dest, perms).unwrap();
                            println!(":: {} installed to {}", pkg, dest.display());
                            println!(":: Run it with: {}", pkg);
                        }
                        None => eprintln!("error: could not find binary in deb package"),
                    }
                }
                None => eprintln!("error: could not find data.tar in deb package"),
            }
        }
        _ => eprintln!("error: bsdtar failed, is it installed?"),
    }

    fs::remove_dir_all(&tmp_dir).ok();
}

fn install_rpm(filepath: &PathBuf, pkg: &str, install_dir: &PathBuf) {
    let tmp_dir = PathBuf::from(format!("/tmp/expl-{}", pkg));
    fs::create_dir_all(&tmp_dir).ok();

    let status = Command::new("bsdtar")
        .args([
            "-xf",
            filepath.to_str().unwrap(),
            "-C",
            tmp_dir.to_str().unwrap(),
        ])
        .status();

    match status {
        Ok(s) if s.success() => {
            let binary = find_binary_in_usr(&tmp_dir, pkg);
            match binary {
                Some(bin) => {
                    let dest = install_dir.join(pkg);
                    fs::copy(&bin, &dest).unwrap();
                    let mut perms = fs::metadata(&dest).unwrap().permissions();
                    perms.set_mode(0o755);
                    fs::set_permissions(&dest, perms).unwrap();
                    println!(":: {} installed to {}", pkg, dest.display());
                    println!(":: Run it with: {}", pkg);
                }
                None => eprintln!("error: could not find binary in rpm package"),
            }
        }
        _ => eprintln!("error: bsdtar failed, is it installed?"),
    }

    fs::remove_dir_all(&tmp_dir).ok();
}

fn find_binary(dir: &PathBuf, pkg: &str) -> Option<PathBuf> {
    for entry in walkdir(dir) {
        let name = entry.file_name().to_string_lossy().to_lowercase();
        if name == pkg || name == pkg.replace("-", "_") {
            if is_executable(&entry.path().to_path_buf()) {
                return Some(entry.path().to_path_buf());
            }
        }
    }
    for entry in walkdir(dir) {
        if is_executable(&entry.path().to_path_buf()) {
            return Some(entry.path().to_path_buf());
        }
    }
    None
}

fn find_binary_in_usr(dir: &PathBuf, pkg: &str) -> Option<PathBuf> {
    let paths = [
        dir.join("usr/bin").join(pkg),
        dir.join("usr/local/bin").join(pkg),
        dir.join("usr/bin").join(pkg.replace("-", "_")),
    ];
    for p in &paths {
        if p.exists() {
            return Some(p.clone());
        }
    }
    find_binary(dir, pkg)
}

fn find_data_tar(dir: &PathBuf) -> Option<PathBuf> {
    let names = ["data.tar.gz", "data.tar.xz", "data.tar.zst", "data.tar.bz2"];
    for name in &names {
        let p = dir.join(name);
        if p.exists() {
            return Some(p);
        }
    }
    None
}

fn walkdir(dir: &PathBuf) -> Vec<std::fs::DirEntry> {
    let mut entries = vec![];
    if let Ok(rd) = fs::read_dir(dir) {
        for entry in rd.flatten() {
            let path = entry.path();
            if path.is_dir() {
                entries.extend(walkdir(&path));
            } else {
                entries.push(entry);
            }
        }
    }
    entries
}

fn is_executable(path: &PathBuf) -> bool {
    if let Ok(meta) = fs::metadata(path) {
        if meta.is_file() {
            return meta.permissions().mode() & 0o111 != 0;
        }
    }
    false
}

fn fallback_to_native(pkg: &str) {
    let managers = [
        ("yay", vec!["-S", "--noconfirm", pkg]),
        ("pacman", vec!["-S", "--noconfirm", pkg]),
    ];

    for (manager, args) in &managers {
        if Command::new("which")
            .arg(manager)
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
        {
            println!(":: Using {}...", manager);
            Command::new(manager)
                .args(args)
                .status()
                .ok();
            return;
        }
    }

    eprintln!("error: no package manager found (yay/pacman)");
}

pub async fn run(pkg: &str) {
    println!(":: Looking for {}...", pkg);

    let package = match repo::find_package(pkg) {
        Some(p) => p,
        None => {
            repo::sync_index().await;
            match repo::find_package(pkg) {
                Some(p) => p,
                None => {
                    eprintln!("error: package '{}' not found in expl index", pkg);
                    println!(":: Trying native package manager...");
                    fallback_to_native(pkg);
                    return;
                }
            }
        }
    };

    println!(":: Found {} v{}", pkg, package.version);
    println!(":: {}", package.description);
    println!();

    let format = detect_format(&package.url);

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

    let install_dir = PathBuf::from(
        INSTALL_DIR.replace("~", &std::env::var("HOME").unwrap_or_default()),
    );
    fs::create_dir_all(&install_dir).ok();

    let ext = if package.url.ends_with(".AppImage") || package.url.ends_with(".appimage") {
        "AppImage"
    } else if package.url.ends_with(".tar.gz") || package.url.ends_with(".tgz") {
        "tar.gz"
    } else if package.url.ends_with(".deb") {
        "deb"
    } else if package.url.ends_with(".rpm") {
        "rpm"
    } else {
        "bin"
    };

    let tmp_file = PathBuf::from(format!("/tmp/expl-{}.{}", pkg, ext));
    let mut file = tokio::fs::File::create(&tmp_file).await.unwrap();

    let mut stream = resp.bytes_stream();
    while let Some(chunk) = stream.next().await {
        let chunk = chunk.unwrap();
        pb.inc(chunk.len() as u64);
        file.write_all(&chunk).await.unwrap();
    }

    pb.finish_and_clear();

    match format {
        PackageFormat::AppImage => install_appimage(&tmp_file, pkg, &install_dir),
        PackageFormat::TarGz => install_targz(&tmp_file, pkg, &install_dir),
        PackageFormat::Deb => install_deb(&tmp_file, pkg, &install_dir),
        PackageFormat::Rpm => install_rpm(&tmp_file, pkg, &install_dir),
        PackageFormat::Unknown => {
            eprintln!("error: unknown package format");
        }
    }

    fs::remove_file(&tmp_file).ok();
}