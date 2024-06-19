use crate::random::GameSeed;
use serde::{Deserialize, Serialize};

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

