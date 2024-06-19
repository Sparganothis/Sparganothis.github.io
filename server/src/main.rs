#![deny(unused_crate_dependencies)]
use tower as _;

pub mod backend;
pub mod database;

#[tokio::main]
pub async fn main() {
    use crate::backend::server_main::server_main;
    server_main().await
}

#[cfg(test)]
pub mod tests {

    #[test]
    pub fn test_2_plus_2() {
        assert_eq!(2 + 2, 4)
    }
}
