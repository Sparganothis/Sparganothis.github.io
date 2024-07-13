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
    blitz,
}

impl GameMatchType {
    pub fn to_url(item: &Option<Self>) -> String {
        match item {
            None => "solo".to_string(),
            Some(Self::_40lines) => "40lines".to_string(),
            Some(Self::_1v1) => "1v1".to_string(),
            Some(Self::blitz) => "blitz".to_string(),
            Some(Self::_4v4) => "4v4".to_string(),
            Some(Self::_10v10) => "10v10".to_string(),
            Some(Self::ManVsCar(_bot)) => format!("bot_{}
            ", _bot)
        }
    }

    pub fn from_url(s: &str) -> anyhow::Result< Option<Self>> {
        Ok(match s {
             "solo"       =>   None                    ,
             "40lines"       =>   Some(Self::_40lines) ,
             "1v1"       =>   Some(Self::_1v1)       ,
             "blitz"       =>   Some(Self::blitz)      ,
             "4v4"       =>   Some(Self::_4v4)       ,
             "10v10"       =>   Some(Self::_10v10)     ,
            _ => {
                if s.starts_with("bot_") {
                    let bot_name = &s[4..];
                    Some(Self::ManVsCar(bot_name.to_string()))
                } else {anyhow::bail!("bad url!");}
            }
        })
    }

    pub fn get_match_num_players(item: &Option<Self>) -> usize {
        match item {
            None => 0,
            Some(Self::_40lines) => 1,
            Some(Self::_1v1) => 1,
            Some(Self::blitz) => 1,
            Some(Self::_4v4) => 8,
            Some(Self::_10v10) => 20,
            Some(Self::ManVsCar(_bot)) => 1,
        }
    }
}
