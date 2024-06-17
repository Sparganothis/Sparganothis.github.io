use serde::Deserialize;
use serde::Serialize;

use super::game_replay::FullGameReplayDbRow;
use super::game_replay::GameId;
#[derive(Serialize, Deserialize, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum SocketType {
    Specctate(uuid::Uuid),
    Game1V1,
}

#[derive(Copy, Debug, Serialize, Deserialize, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum WebsocketAPIMessageType {
    GetProfile,
    WhoAmI,
    GitVersion,
    GetFullGameReplay,
    GetAllFullGameReplays,
    CreateNewGameId,
    AppendGameSegment,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct WebsocketAPIMessageRaw {
    pub id: u32,
    pub is_req: bool,
    pub _type: WebsocketAPIMessageType,
    pub data: Vec<u8>,
}

use anyhow::Context;
pub trait APIMethod {
    const TYPE: WebsocketAPIMessageType;
    type Req: Serialize + for<'a> Deserialize<'a> + std::marker::Send + std::marker::Sync + 'static;
    type Resp: Serialize + for<'a> Deserialize<'a> + std::marker::Send + std::marker::Sync + 'static;

    fn send(msg: Self::Req, sender: impl Fn(Vec<u8>), id: u32) -> anyhow::Result<u32> {
        let b = WebsocketAPIMessageRaw {
            id,
            is_req: true,
            _type: Self::TYPE,
            data: bincode::serialize(&msg).context("error serialize")?,
        };
        let b = bincode::serialize(&b).context("error serialize")?;
        sender(b);
        Ok(id)
    }
}

pub struct GetProfile {}
impl APIMethod for GetProfile {
    const TYPE: WebsocketAPIMessageType = WebsocketAPIMessageType::GetProfile;
    type Req = uuid::Uuid;
    type Resp = crate::api::user::UserProfile;
}

pub struct WhoAmI {}
impl APIMethod for WhoAmI {
    const TYPE: WebsocketAPIMessageType = WebsocketAPIMessageType::WhoAmI;
    type Req = ();
    type Resp = crate::api::user::GuestInfo;
}

pub struct GitVersion {}
impl APIMethod for GitVersion {
    const TYPE: WebsocketAPIMessageType = WebsocketAPIMessageType::GitVersion;
    type Req = ();
    type Resp = String;
}

pub struct GetFullGameReplay {}
impl APIMethod for GetFullGameReplay {
    const TYPE: WebsocketAPIMessageType = WebsocketAPIMessageType::GetFullGameReplay;
    type Req = GameId;
    type Resp = FullGameReplayDbRow;
}

pub struct GetAllFullGameReplays {}
impl APIMethod for GetAllFullGameReplays {
    const TYPE: WebsocketAPIMessageType = WebsocketAPIMessageType::GetAllFullGameReplays;
    type Req = ();
    type Resp = Vec<FullGameReplayDbRow>;
}

pub struct CreateNewGameId {}
impl APIMethod for CreateNewGameId {
    const TYPE: WebsocketAPIMessageType = WebsocketAPIMessageType::CreateNewGameId;
    type Req = ();
    type Resp = GameId;
}

pub struct AppendGameSegment {}
impl APIMethod for AppendGameSegment {
    const TYPE: WebsocketAPIMessageType = WebsocketAPIMessageType::AppendGameSegment;
    type Req = (GameId, String);
    type Resp = ();
}
