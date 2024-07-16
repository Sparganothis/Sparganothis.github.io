use matrix_sdk::{config::SyncSettings, Client, LoopCtrl};

use super::config::{ChatbotConfig, ChatbotLoginConfig};
use super::messages::ChatbotMessage;
use crate::chatbot::config::BotRoomType;
use crate::{
    backend::server_info::GIT_VERSION,
    chatbot::{
        config::{bot_send_message, get_bot_config, get_bot_room},
        room_feedback::on_feedback_room_message,
        room_public::on_public_room_message,
        room_serverlog::on_server_log_room_message,
    },
};

async fn do_login(cfg: &ChatbotLoginConfig) -> anyhow::Result<Client> {
    // Note that when encryption is enabled, you should use a persistent store to be
    // able to restore the session with a working encryption setup.
    // See the `persist_session` example.
    let client: Client = Client::builder()
        .homeserver_url(cfg.matrix_homeserver.clone())
        .build()
        .await?;
    log::info!("matrix client built: {:?}", client);
    client
        .matrix_auth()
        .login_username(&cfg.matrix_username, &cfg.matrix_password)
        .initial_device_display_name("server")
        .await?;

    log::info!("matrix bot login OK");
    Ok(client)
}
use std::{sync::Arc, time::Duration};
use tokio::sync::Mutex;

async fn send_messages_to_chat(client: Client, mut rx: tokio::sync::mpsc::Receiver<ChatbotMessage>, srv_hostname: String ) {
    while let Some(r) = rx.recv().await {
        if let Err(e) = bot_send_message(&client, BotRoomType::ServerLog, format!("[{}]: {:#?}", srv_hostname, r)).await {
            log::warn!("failed to send message to matrix: {:?}", e);
        }
    }
}

async fn sync_forever(client: Client, first_batch: String) {
    let next_batch = Arc::new(Mutex::new(first_batch));
    loop {
        log::info!("matrix bot sync #2 (forever).");
        let nb2 = next_batch.clone();
        let x = async move {
            let lock = nb2.lock().await;
            lock.clone()
        }
        .await;
        let settings = SyncSettings::default()
            .token(x)
            .timeout(Duration::from_secs_f32(36.6));
        let nb3 = next_batch.clone();
        if let Err(_e) = client
            .sync_with_callback(settings, |r| {
                let nb4 = nb3.clone();
                async move {
                    let mut lock = nb4.lock().await;
                    *lock = r.next_batch;
                    LoopCtrl::Continue
                }
            })
            .await
        {
            log::warn!("matrix bot sync error: {:?}", _e);
        } else {
            log::warn!("matrix bot sync exit. ?!?!");
        }
        tokio::time::sleep(tokio::time::Duration::from_secs_f32(43.3)).await;
    }
}
async fn login_and_sync_forever(cfg: &ChatbotConfig,rx: tokio::sync::mpsc::Receiver<ChatbotMessage>) -> anyhow::Result<()> {
    let client = do_login(&cfg.login_config).await?;
    log::info!("matrix bot sync #1 start...");
    let response = client.sync_once(SyncSettings::default()).await?;
    let first_batch: String = response.next_batch;
    log::info!("matrix bot sync #1 done.");

    if !cfg.hostname.is_empty() {
        log::info!("matrix bot send init message...");
        let startup_msg = format!(
            "server start, hostname={} version={}",
            cfg.hostname,
            GIT_VERSION.trim()
        );
        bot_send_message(&client, BotRoomType::ServerLog, startup_msg).await?;
    }

    log::info!("matrix bot starting handler...");
    // add our CommandBot to be notified of incoming messages, we do this after the
    // initial sync to avoid responding to messages before the bot was running.
    client.add_room_event_handler(
        get_bot_room(&client, BotRoomType::ServerLog)?.room_id(),
        on_server_log_room_message,
    );
    client.add_room_event_handler(
        get_bot_room(&client, BotRoomType::FeedbackForm)?.room_id(),
        on_feedback_room_message,
    );
    client.add_room_event_handler(
        get_bot_room(&client, BotRoomType::Public)?.room_id(),
        on_public_room_message,
    );

    let _task_send_msg = tokio::spawn(send_messages_to_chat(client.clone(), rx, cfg.hostname.clone()));
    let _task_sync = tokio::spawn(sync_forever(client.clone(), first_batch));
    // since we called `sync_once` before we entered our sync loop we must pass
    // that sync token to `sync`
    // this keeps state from the server streaming in to CommandBot via the
    // EventHandler trait

    _task_send_msg.await?;
    _task_sync.await?;
    Ok(())
}

pub async fn bot_main(rx: tokio::sync::mpsc::Receiver<ChatbotMessage>) -> anyhow::Result<()> {
    if let Some(cfg) = get_bot_config() {
        log::info!("matrix bot init...");
        login_and_sync_forever(&cfg, rx).await?;
        log::info!("matrix bot init OK.");
    } else {
        log::info!("matrix bot not configured, skipping.");
        let mut rx = rx;
        loop{
            while let Some(_x) = rx.recv().await {
                // drop msg.
            }
        }
    }

    Ok(())
}
