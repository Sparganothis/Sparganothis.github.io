#[cfg(feature = "ssr")]
#[cfg_attr(feature = "ssr", tokio::main)]
async fn main() {
    use webapp::server::backend::server_main::server_main;
    server_main().await
}

#[cfg(not(feature = "ssr"))]
pub fn main() {}
