use serde::Deserialize;
use serde::Serialize;

use crate::tet::GameReplaySegment;
use crate::tet::GameState;

use super::game_match::GameMatch;
use super::game_match::GameMatchType;
use super::game_replay::GameId;
use super::game_replay::GameSegmentId;
use super::table_paginate::TablePaginateDirection;
use super::user_settings::UserSettingType;

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

    SubscribeGamePlz,
    SubscribedGameUpdateNotification,

    StartMatch,
    GetMatchList,

    GetMatchInfo,

    GetUserSetting,
    SetUserSetting,

    AppendBotGameSegment,

    SetGlobalPlayLock,
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
        + 'static
        + Clone;
    type Resp: Serialize
        + for<'a> Deserialize<'a>
        + std::marker::Send
        + std::marker::Sync
        + 'static
        + Clone;

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

pub struct AppendBotGameSegment {}
impl APIMethod for AppendBotGameSegment {
    const TYPE: WebsocketAPIMessageType = WebsocketAPIMessageType::AppendBotGameSegment;
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
    type Resp = Option<GameState>;
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
    type Req = (GetAllGamesArg, TablePaginateDirection<GameId>);
    type Resp = Vec<(GameId, GameSegmentCountReply)>;
}

pub struct GetAllCustomGames {}
impl APIMethod for GetAllCustomGames {
    const TYPE: WebsocketAPIMessageType = WebsocketAPIMessageType::GetAllCustomGames;
    type Req = TablePaginateDirection<String>;
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

#[derive(
    Copy, Debug, Serialize, Deserialize, Clone, PartialEq, PartialOrd, Eq, Ord,
)]
pub struct SubscribeGamePlzArgument {
    pub game_id: GameId,
    pub command: SubscribeGamePlzCommmand,
}
#[derive(
    Copy, Debug, Serialize, Deserialize, Clone, PartialEq, PartialOrd, Eq, Ord, Hash,
)]
pub enum SubscribeGamePlzCommmand {
    StartStreaming,
    StopStreaming,
}

pub struct SubscribeGamePlz {}
impl APIMethod for SubscribeGamePlz {
    const TYPE: WebsocketAPIMessageType = WebsocketAPIMessageType::SubscribeGamePlz;
    type Req = SubscribeGamePlzArgument;
    type Resp = ();
}

pub struct SubscribedGameUpdateNotification {}

impl APIMethod for SubscribedGameUpdateNotification {
    const TYPE: WebsocketAPIMessageType =
        WebsocketAPIMessageType::SubscribedGameUpdateNotification;
    type Req = Vec<(GameSegmentId, GameReplaySegment)>;
    type Resp = ();
}

pub struct StartMatch {}
impl APIMethod for StartMatch {
    const TYPE: WebsocketAPIMessageType = WebsocketAPIMessageType::StartMatch;
    type Req = GameMatchType;
    type Resp = (uuid::Uuid, GameMatch);
}

#[derive(
    Copy, Debug, Serialize, Deserialize, Clone, PartialEq, PartialOrd, Eq, Ord, Hash,
)]
pub enum GetMatchListArg {
    BestGames,
    RecentGames,
    MyBestGames,
    MyRecentGames,
    BestGamesForPlayer(uuid::Uuid),
    RecentGamesForPlayer(uuid::Uuid),
}

pub struct GetMatchList {}
impl APIMethod for GetMatchList {
    const TYPE: WebsocketAPIMessageType = WebsocketAPIMessageType::GetMatchList;
    type Req = (GetMatchListArg, TablePaginateDirection<uuid::Uuid>);
    type Resp = Vec<(uuid::Uuid, GameMatch)>;
}

pub struct GetMatchInfo {}
impl APIMethod for GetMatchInfo {
    const TYPE: WebsocketAPIMessageType = WebsocketAPIMessageType::GetMatchInfo;
    type Req = uuid::Uuid;
    type Resp = GameMatch;
}

pub struct GetUserSetting {}
impl APIMethod for GetUserSetting {
    const TYPE: WebsocketAPIMessageType = WebsocketAPIMessageType::GetUserSetting;
    type Req = UserSettingType;
    type Resp = Vec<u8>;
}

pub struct SetUserSetting {}
impl APIMethod for SetUserSetting {
    const TYPE: WebsocketAPIMessageType = WebsocketAPIMessageType::SetUserSetting;
    type Req = (UserSettingType, Vec<u8>);
    type Resp = ();
}

pub struct SetGlobalPlayLock {}
impl APIMethod for SetGlobalPlayLock {
    const TYPE: WebsocketAPIMessageType = WebsocketAPIMessageType::SetGlobalPlayLock;
    type Req = (bool, Option<GameId>);
    type Resp = ();
}
