use std::env;

use anyhow::Context;
use matrix_sdk::{ruma::{events::room::message::RoomMessageEventContent, RoomId}, Room};


#[derive(Clone)]
pub struct ChatbotLoginConfig {
    pub matrix_homeserver: String,
    pub matrix_username: String,
    pub matrix_password: String,
}

#[derive(Clone)]
pub struct ChatbotRoomConfig {
    pub matrix_room_id_feedback: String,
    pub matrix_room_id_server_log: String,
    pub matrix_room_id_public: String,
}

#[derive(Clone)]
pub struct ChatbotConfig {
    pub hostname: String,
    pub login_config: ChatbotLoginConfig,
    pub room_config: ChatbotRoomConfig,
}

#[derive(Clone, Copy, Debug)]
pub enum BotRoomType {
    FeedbackForm,
    ServerLog,
    Public
}

// pub fn bot_room_type_from_id(room_id: &str) -> Option<BotRoomType> {
//     let cfg = get_bot_config().unwrap();
//     if cfg.room_config.matrix_room_id_feedback == room_id {
//         Some(BotRoomType::FeedbackForm)
//     } else if cfg.room_config.matrix_room_id_server_log == room_id {
//         Some(BotRoomType::ServerLog)
//     } else if cfg.room_config.matrix_room_id_public == room_id {
//         Some(BotRoomType::Public)
//     } else {
//         None
//     }
// }

pub fn bot_room_id_from_type(type_: BotRoomType) -> anyhow::Result<String> {
    let cfg = get_bot_config().context("no bot settings")?;
    Ok(match type_ {
        BotRoomType::FeedbackForm => cfg.room_config.matrix_room_id_feedback,
        BotRoomType::ServerLog => cfg.room_config.matrix_room_id_server_log,
        BotRoomType::Public => cfg.room_config.matrix_room_id_public,
    })
}

pub fn get_bot_room(client: &matrix_sdk::Client, _type: BotRoomType) -> anyhow::Result<Room> {
    let room_id_str = bot_room_id_from_type(_type)?;
    let room_id = <&RoomId>::try_from(room_id_str.as_str())?;
    let room = client.get_room(&room_id).context("room not found")?;
    Ok(room)
}

 pub async fn bot_send_message(client: &matrix_sdk::Client, room: BotRoomType, msg: String) -> anyhow::Result<()> {
    let content = RoomMessageEventContent::text_plain(msg);
    let _r = get_bot_room(client, room)?.send(content).await?;
    Ok(())
}

pub fn get_bot_config() -> Option<ChatbotConfig> {
    if let (
        Ok(hs), 
        Ok(un), 
        Ok(pw),
        Ok(room_feedback),
        Ok(room_server_log),
        Ok(room_public),
    ) = ( 
        env::var("SERVER_MATRIX_HOMESERVER"),
        env::var("SERVER_MATRIX_USERNAME"),
        env::var("SERVER_MATRIX_PASSWORD"),
        env::var("SERVER_MATRIX_ROOM_ID_FEEDBACK"),
        env::var("SERVER_MATRIX_ROOM_ID_SERVER_LOG"),
        env::var("SERVER_MATRIX_ROOM_ID_PUBLIC"),
    ) {
        Some(ChatbotConfig {
            login_config: ChatbotLoginConfig {
                matrix_homeserver: hs,
                matrix_username: un,
                matrix_password: pw,
            },
            room_config: ChatbotRoomConfig {
                matrix_room_id_feedback: room_feedback,
                matrix_room_id_server_log: room_server_log,
                matrix_room_id_public: room_public,
            },
            hostname: env::var("SERVER_HOSTNAME").unwrap_or("".to_string())
        })
    } else {
        None
    }
}