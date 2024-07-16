#![deny(unused_crate_dependencies)]
use chatbot::messages::ChatbotMessage;
use rusqlite as _;
use tower as _;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt}; // for matrix sdk

pub mod backend;
pub mod chatbot;
pub mod database;

#[tokio::main]
pub async fn main() {
    // simple_logger::init_with_level(log::Level::Debug).expect("couldn't initialize logging");
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "server=debug,tower_http=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let (tx, rx) = tokio::sync::mpsc::channel::<ChatbotMessage>(16);

    // CHAT BOT MAIN
    use crate::chatbot::chatbot::bot_main;
    let bot_task = tokio::spawn(bot_main(rx));

    // WEBSOCK SERVER MAIN
    use crate::backend::server_main::server_main;
    let server_task = tokio::spawn(server_main(tx));

    if let Err(e) = bot_task.await.unwrap() {
        log::warn!("matrix bot died: {:?}", e);
    } else {
        log::warn!("matrix bot died naturally.");
    }
    let _ = server_task.await;
}

#[cfg(test)]
pub mod tests {

    #[test]
    pub fn test_2_plus_2() {
        assert_eq!(2 + 2, 4)
    }
}
