use matrix_sdk::{
    config::SyncSettings,
    Client, LoopCtrl,
};

use crate::{backend::server_info::GIT_VERSION, chatbot::{config::{bot_send_message, get_bot_config, get_bot_room}, room_feedback::on_feedback_room_message, room_public::on_public_room_message, room_serverlog::on_server_log_room_message}};
use crate::chatbot::config::BotRoomType;
use super::config::{ChatbotConfig, ChatbotLoginConfig};

async fn do_login(cfg: &ChatbotLoginConfig) -> anyhow::Result<Client> {
    // Note that when encryption is enabled, you should use a persistent store to be
    // able to restore the session with a working encryption setup.
    // See the `persist_session` example.
    let client: Client = Client::builder().homeserver_url(cfg.matrix_homeserver.clone()).build().await?;
    log::info!("matrix client built: {:?}", client);
    client
        .matrix_auth()
        .login_username(&cfg.matrix_username, &cfg.matrix_password)
        .initial_device_display_name("server")
        .await?;

    log::info!("matrix bot login OK");
    Ok(client)
}
use tokio::sync::Mutex;
use std::{sync::Arc, time::Duration};
async fn login_and_sync_forever(cfg: &ChatbotConfig) -> anyhow::Result<()> {
    let client = do_login(&cfg.login_config).await?;
    log::info!("matrix bot sync #1 start...");
    let response = client.sync_once(SyncSettings::default()).await?;
    let first_batch: String = response.next_batch;
    let next_batch = Arc::new(Mutex::new(first_batch));
    log::info!("matrix bot sync #1 done.");

    log::info!("matrix bot send init message...");
    let startup_msg = format!("server start, version={}", GIT_VERSION.trim());
    bot_send_message(&client, BotRoomType::ServerLog, startup_msg).await?;

    log::info!("matrix bot starting handler...");
    // add our CommandBot to be notified of incoming messages, we do this after the
    // initial sync to avoid responding to messages before the bot was running.
    client.add_room_event_handler(get_bot_room(&client, BotRoomType::ServerLog)?.room_id(), on_server_log_room_message, );
    client.add_room_event_handler(get_bot_room(&client, BotRoomType::FeedbackForm)?.room_id(), on_feedback_room_message, );
    client.add_room_event_handler(get_bot_room(&client, BotRoomType::Public)?.room_id(), on_public_room_message, );
    // since we called `sync_once` before we entered our sync loop we must pass
    // that sync token to `sync`
    // this keeps state from the server streaming in to CommandBot via the
    // EventHandler trait
    loop {
        log::info!("matrix bot sync #2 (forever).");
        let nb2 = next_batch.clone();
        let x = async move {
            let lock = nb2.lock().await;
            lock.clone()
        }.await;
        let settings = SyncSettings::default().token(x).timeout(Duration::from_secs_f32(36.6));
        let nb3 = next_batch.clone();
        if let Err(_e) = client.sync_with_callback(settings, |r| {let nb4 = nb3.clone(); async move {
                let mut lock = nb4.lock().await;
                *lock = r.next_batch;
            LoopCtrl::Continue
        }}).await {
            log::warn!("matrix bot sync error: {:?}", _e);
        } else {
            log::warn!("matrix bot sync exit.");
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(33)).await;
    }
}

pub async fn bot_main() -> anyhow::Result<()> {
    if let Some(cfg) = get_bot_config() {
        log::info!("matrix bot init...");
        login_and_sync_forever(&cfg).await?;
        log::info!("matrix bot init OK.");
    } else {
        log::info!("matrix bot not configured, skipping.")
    }

    Ok(())
}