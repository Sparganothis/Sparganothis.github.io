use serde::Deserialize;
use serde::Serialize;

use crate::tet::GameReplaySegment;
use crate::tet::GameState;

use super::game_replay::GameId;

#[derive(
    Copy, Debug, Serialize, Deserialize, Clone, PartialEq, PartialOrd, Eq, Ord, Hash,
)]
pub enum WebsocketAPIMessageType {
    GetProfile,
    WhoAmI,
    GitVersion,
    CreateNewGameId,
    AppendGameSegment,
    GetSegmentCount,
    GetAllSegments,
    GetLastFullGameState,
    GetAllGames,

    GetAllCustomGames,
    GetCustomGame,
    UpdateCustomGame,
    GetRandomWord,
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
    type Req: Serialize
        + for<'a> Deserialize<'a>
        + std::marker::Send
        + std::marker::Sync
        + 'static;
    type Resp: Serialize
        + for<'a> Deserialize<'a>
        + std::marker::Send
        + std::marker::Sync
        + 'static;

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

#[derive(
    Copy, Debug, Serialize, Deserialize, Clone, PartialEq, PartialOrd, Eq, Ord, Hash,
)]
pub struct GameSegmentCountReply {
    pub is_in_progress: bool,
    pub segment_count: u32,
}
pub struct GetSegmentCount {}
impl APIMethod for GetSegmentCount {
    const TYPE: WebsocketAPIMessageType = WebsocketAPIMessageType::GetSegmentCount;
    type Req = GameId;
    type Resp = GameSegmentCountReply;
}
pub struct GetAllSegments {}
impl APIMethod for GetAllSegments {
    const TYPE: WebsocketAPIMessageType = WebsocketAPIMessageType::GetAllSegments;
    type Req = GameId;
    type Resp = Vec<GameReplaySegment>;
}

pub struct GetLastFullGameState {}
impl APIMethod for GetLastFullGameState {
    const TYPE: WebsocketAPIMessageType = WebsocketAPIMessageType::GetLastFullGameState;
    type Req = GameId;
    type Resp = GameState;
}

#[derive(
    Copy, Debug, Serialize, Deserialize, Clone, PartialEq, PartialOrd, Eq, Ord, Hash,
)]
pub enum GetAllGamesArg {
    BestGames,
    RecentGames,
    MyBestGames,
    MyRecentGames,
    BestGamesForPlayer(uuid::Uuid),
    RecentGamesForPlayer(uuid::Uuid),
}
pub struct GetAllGames {}
impl APIMethod for GetAllGames {
    const TYPE: WebsocketAPIMessageType = WebsocketAPIMessageType::GetAllGames;
    type Req = GetAllGamesArg;
    type Resp = Vec<(GameId, GameSegmentCountReply)>;
}

pub struct GetAllCustomGames {}
impl APIMethod for GetAllCustomGames {
    const TYPE: WebsocketAPIMessageType = WebsocketAPIMessageType::GetAllCustomGames;
    type Req = ();
    type Resp = Vec<(String, GameState)>;
}

pub struct GetCustomGame {}
impl APIMethod for GetCustomGame {
    const TYPE: WebsocketAPIMessageType = WebsocketAPIMessageType::GetCustomGame;
    type Req = String;
    type Resp = GameState;
}

pub struct UpdateCustomGame {}
impl APIMethod for UpdateCustomGame {
    const TYPE: WebsocketAPIMessageType = WebsocketAPIMessageType::UpdateCustomGame;
    type Req = (String, GameState);
    type Resp = ();
}
pub struct GetRandomWord {}
impl APIMethod for GetRandomWord {
    const TYPE: WebsocketAPIMessageType = WebsocketAPIMessageType::GetRandomWord;
    type Req = ();
    type Resp = String;
}
