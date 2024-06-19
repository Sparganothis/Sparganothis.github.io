use crate::{random::GameSeed, tet::GameReplaySegment};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct GameSegmentId {
    game_id: GameId,
    segment_id: u32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct GameId {
    pub user_id: uuid::Uuid,
    pub init_seed: GameSeed,
    pub start_time: i64,
}

use serde::{Deserialize, Serialize};
