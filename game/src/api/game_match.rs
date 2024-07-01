use serde::{Deserialize, Serialize};

use crate::random::GameSeed;

#[derive(
    Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
pub struct GameMatch {
    pub seed: GameSeed,
    pub time: i64,
    pub users: Vec<uuid::Uuid>,
    pub title: String,
    pub type_: GameMatchType,
}

#[derive(
    Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
pub struct UserAndMatchId {
    pub user_id: uuid::Uuid,
    pub match_id: uuid::Uuid,
}

impl UserAndMatchId {
    pub fn get_range_for_user(
        user: &uuid::Uuid,
    ) -> std::ops::RangeInclusive<UserAndMatchId> {
        let m0 = uuid::uuid!("00000000-0000-0000-0000-000000000000");
        let m1 = uuid::uuid!("FFFFFFFF-FFFF-FFFF-FFFF-FFFFFFFFFFFF");
        let v0 = UserAndMatchId {
            user_id: user.clone(),
            match_id: m0,
        };
        let v1 = UserAndMatchId {
            user_id: user.clone(),
            match_id: m1,
        };
        v0..=v1
    }
}

#[derive(
    Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
pub struct UserAndMatchResult {
    pub is_win: bool,
    pub podium_position: u32,
}

#[derive(
    Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
pub enum GameMatchType {
    _1v1,
    ManVsCar(String),
    _40lines,
    _10v10,
    _4v4,
}
