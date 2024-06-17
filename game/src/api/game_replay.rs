use crate::{random::GameSeed, tet::GameReplaySegment};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FullGameReplayDbRow {
    pub id: GameId,
    pub segments: Vec<GameReplaySegment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameId {
    pub user_id: uuid::Uuid,
    pub init_seed: GameSeed,
    pub start_time: i64,
}

use serde::{Deserialize, Serialize};
