use rand::Rng;

const TIPS: &[&str] = &[
    "btw i use arch",
    "Just install Arch :) not Manjaro",
    "Have you tried turning off Ubuntu and never turning it back on?",
    "Tip: real men compile from source",
    "Gentoo users already compiled this package",
    "Tip: read the man pages",
    "If it breaks, it's a feature",
    "Tip: pacman -Syu is a lifestyle",
    "Windows users are crying right now",
    "Tip: aliases save lives",
    "Your package is downloading faster than Ubuntu boots",
    "Tip: nano for configs, Cursor for code, perfect setup",
    "Tip: vim users spend more time configuring vim than coding",
];

pub fn random_tip() -> &'static str {
    let mut rng = rand::thread_rng();
    TIPS[rng.gen_range(0..TIPS.len())]
}
