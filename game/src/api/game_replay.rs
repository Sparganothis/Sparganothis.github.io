use crate::random::GameSeed;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct GameSegmentId {
    pub game_id: GameId,
    pub segment_id: u32,
}

impl GameSegmentId {
    pub fn get_range_for_game(
        game: &GameId,
    ) -> std::ops::RangeInclusive<GameSegmentId> {
        let seg0 = u32::MIN;
        let seg1 = u32::MAX;

        let v0 = GameSegmentId {
            game_id: game.clone(),
            segment_id: seg0,
        };
        let v1 = GameSegmentId {
            game_id: game.clone(),
            segment_id: seg1,
        };
        v0..=v1
    }
}

#[derive(
    Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
pub struct GameId {
    pub user_id: uuid::Uuid,
    pub init_seed: GameSeed,
    pub start_time: i64,
}

impl GameId {
    pub fn from_url(url: String) -> anyhow::Result<Self> {
        let b = hex::decode(url)?;
        Ok(bincode::deserialize(&b)?)
    }
    pub fn to_url(&self) -> String {
        let bytes = bincode::serialize(self).unwrap();
        hex::encode(bytes)
    }
    pub fn get_range_for_user(user: &uuid::Uuid) -> std::ops::RangeInclusive<GameId> {
        let seed0: GameSeed = [0; 32];
        let seed1: GameSeed = [u8::MAX; 32];
        let time0 = 0;
        let time1 = i64::MAX;

        let v0 = GameId {
            user_id: user.clone(),
            init_seed: seed0,
            start_time: time0,
        };
        let v1 = GameId {
            user_id: user.clone(),
            init_seed: seed1,
            start_time: time1,
        };
        v0..=v1
    }
}
