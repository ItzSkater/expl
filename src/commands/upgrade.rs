use crate::repo;

pub async fn run() {
    println!(":: Syncing index...");
    repo::sync_index().await;
    println!(":: Checking for upgrades...");
    // TODO: сравнивать версии установленных пакетов с индексом
    println!(":: System is up to date");
}
