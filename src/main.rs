mod commands;
mod config;
mod distro;
mod repo;
mod tips;

#[tokio::main]
async fn main() {
    distro::check_distro();

    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        print_help();
        return;
    }

    let flag = &args[1];
    let pkg = args.get(2).map(|s| s.as_str());

    match flag.as_str() {
        "-S" => {
            match pkg {
                Some(p) => commands::install::run(p).await,
                None => eprintln!("error: -S requires a package name"),
            }
        }
        "-Ss" => {
            match pkg {
                Some(q) => commands::search::run(q).await,
                None => eprintln!("error: -Ss requires a search query"),
            }
        }
        "-R" | "-Rns" => {
            match pkg {
                Some(p) => commands::remove::run(p),
                None => eprintln!("error: -R requires a package name"),
            }
        }
        "-Syu" => commands::upgrade::run().await,
        "-Sy" => repo::sync_index().await,
        "-Sc" => commands::clean::run(),
        "-Scc" => commands::clean::run(),
        "-h" | "--help" => print_help(),
        "-V" | "--version" => println!("expl 0.1.0"),
        _ => {
            eprintln!("error: unknown option '{}'", flag);
            print_help();
        }
    }
}

fn print_help() {
    println!("expl 0.1.0 - Universal Linux package manager for AppImage packages");
    println!();
    println!("Usage: expl <operation> [package]");
    println!();
    println!("Operations:");
    println!("  -S  <pkg>    Install package");
    println!("  -Ss <query>  Search for package");
    println!("  -R  <pkg>    Remove package");
    println!("  -Rns <pkg>   Remove package (with deps)");
    println!("  -Syu         Upgrade all packages");
    println!("  -Sy          Sync package index");
    println!("  -Sc          Clean cache");
    println!("  -Scc         Full cache clean");
    println!("  -V           Print version");
}
