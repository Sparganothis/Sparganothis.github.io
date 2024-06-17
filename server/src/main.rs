pub mod backend;
pub mod database;

#[tokio::main]
pub async fn main() {
    use crate::backend::server_main::server_main;
    server_main().await
}
