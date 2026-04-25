use crate::repo;

pub async fn run(query: &str) {
    let index = match repo::load_index() {
        Some(i) => i,
        None => {
            println!(":: Index not found, syncing...");
            repo::sync_index().await;
            match repo::load_index() {
                Some(i) => i,
                None => {
                    eprintln!("error: could not load index");
                    return;
                }
            }
        }
    };

    let results: Vec<_> = index
        .iter()
        .filter(|(name, pkg)| {
            name.contains(query) || pkg.description.to_lowercase().contains(query)
        })
        .collect();

    if results.is_empty() {
        println!(":: No packages found for '{}'", query);
        return;
    }

    for (name, pkg) in results {
        println!("\x1b[32m{}\x1b[0m v{}", name, pkg.version);
        println!("    {}", pkg.description);
    }
}
