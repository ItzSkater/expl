use std::fs;
use std::process;

const BLOCKED: &[&str] = &[
    "Ubuntu",
    "Kubuntu",
    "Xubuntu",
    "Lubuntu",
    "Ubuntu Budgie",
    "Ubuntu Studio",
    "Ubuntu Cinnamon",
    "Ubuntu Unity",
    "Ubuntu Kylin",
    "Edubuntu",
    "Linux Mint",
    "Pop!_OS",
    "Zorin OS",
    "elementary OS",
    "KDE Neon",
    "Peppermint OS",
    "Vanilla OS",
    "BackBox",
    "Bodhi Linux",
    "Linux Lite",
    "Runtu",
    "Voyage",
];

pub fn check_distro() {
    let os_release = match fs::read_to_string("/etc/os-release") {
        Ok(content) => content,
        Err(_) => return, // не можем определить — пропускаем
    };

    for blocked in BLOCKED {
        if os_release.contains(blocked) {
            eprintln!("error: {} is not supported. Install a real distro.", blocked);
            eprintln!("hint:  Just install Arch :) not Manjaro");
            process::exit(1);
        }
    }
}

pub fn get_distro() -> String {
    let os_release = fs::read_to_string("/etc/os-release").unwrap_or_default();
    for line in os_release.lines() {
        if line.starts_with("NAME=") {
            return line
                .trim_start_matches("NAME=")
                .trim_matches('"')
                .to_string();
        }
    }
    "Unknown".to_string()
}
