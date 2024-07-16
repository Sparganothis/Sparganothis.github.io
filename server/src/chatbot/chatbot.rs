use matrix_sdk::{
    config::SyncSettings,
    Client,
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

async fn login_and_sync(cfg: &ChatbotConfig) -> anyhow::Result<()> {
    let client = do_login(&cfg.login_config).await?;
    log::info!("matrix bot sync #1 start...");
    let response = client.sync_once(SyncSettings::default()).await?;
    log::info!("matrix bot sync #1 done.");

    log::info!("matrix bot send init message...");
    let startup_msg = format!("server start, version={:?}", GIT_VERSION);
    bot_send_message(&client, BotRoomType::ServerLog, startup_msg).await?;

    log::info!("matrix bot starting handler...");
    // add our CommandBot to be notified of incoming messages, we do this after the
    // initial sync to avoid responding to messages before the bot was running.
    client.add_room_event_handler(get_bot_room(&client, BotRoomType::ServerLog)?.room_id(), on_server_log_room_message, );
    client.add_room_event_handler(get_bot_room(&client, BotRoomType::FeedbackForm)?.room_id(), on_feedback_room_message, );
    client.add_room_event_handler(get_bot_room(&client, BotRoomType::Public)?.room_id(), on_public_room_message, );
    // since we called `sync_once` before we entered our sync loop we must pass
    // that sync token to `sync`
    let settings = SyncSettings::default().token(response.next_batch);
    // this keeps state from the server streaming in to CommandBot via the
    // EventHandler trait
    log::info!("matrix bot sync #2 (forever).");
    client.sync(settings).await?;

    Ok(())
}

pub async fn bot_main() -> anyhow::Result<()> {
    if let Some(cfg) = get_bot_config() {
        log::info!("matrix bot init...");
        login_and_sync(&cfg).await?;
        log::info!("matrix bot init OK.");
    } else {
        log::info!("matrix bot not configured, skipping.")
    }

    Ok(())
}