use crate::{random::GameSeed, tet::GameReplaySegment};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct GameSegmentId {
    pub game_id: GameId,
    pub segment_id: u32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct GameId {
    pub user_id: uuid::Uuid,
    pub init_seed: GameSeed,
    pub start_time: i64,
}

impl GameId {
    pub fn to_url(&self) -> String {
        let bytes = bincode::serialize(self).unwrap();
        hex::encode(bytes)
    }
}

use serde::{Deserialize, Serialize};
use serde_with::Bytes;
