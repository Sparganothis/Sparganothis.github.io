use crate::game::{random::GameSeed, tet::GameReplaySegment};

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

use leptos::*;
use rand::Rng;
use serde::{Deserialize, Serialize};

#[server]
pub async fn get_full_game_replay(id: GameId) -> Result<FullGameReplayDbRow, ServerFnError> {
    use super::super::database::tables::GAME_REPLAY_DB;
    if let Ok(r) = GAME_REPLAY_DB.get(&id) {
        if let Some(r) = r {
            Ok(r)
        } else {
            Err(ServerFnError::new("vai plm db error"))
        }
    } else {
        Err(ServerFnError::new("vai plm db error"))
    }
}

#[server]
pub async fn get_all_full_game_replays() -> Result<Vec<FullGameReplayDbRow>, ServerFnError> {
    use super::super::database::tables::GAME_REPLAY_DB;
    let mut v = vec![];
    for x in GAME_REPLAY_DB.iter() {
        let y = x.unwrap().1;
        v.push(y);
    }
    Ok(v)
}

#[server]
pub async fn create_new_game_id() -> Result<GameId, ServerFnError> {
    use super::super::database::tables::GAME_REPLAY_DB;
    use super::user::who_am_i;
    use crate::game::timestamp::get_timestamp_now_nano;
    let who = who_am_i().await?.user_id;
    let mut rand = rand::thread_rng();
    let g = GameId {
        user_id: who,
        init_seed: rand.gen(),
        start_time: get_timestamp_now_nano(),
    };
    let row = FullGameReplayDbRow {
        id: (&g).clone(),
        segments: vec![],
    };
    GAME_REPLAY_DB.insert(&g, &row).unwrap();
    Ok(g)
}

#[server]
pub async fn append_game_segment(
    id: GameId,
    segment_json: String,
) -> Result<(), ServerFnError> {
    let segment: GameReplaySegment  = serde_json::from_str(&segment_json).expect("json never fail");
    use super::super::database::tables::GAME_REPLAY_DB;
    use super::user::who_am_i;
    let who = who_am_i().await?.user_id;
    if !who.eq(&id.user_id) {
        return Err(ServerFnError::new("no impersonate plz"));
    }

    let mut existing_game = GAME_REPLAY_DB.get(&id).unwrap().unwrap();

    match &segment {
        GameReplaySegment::Init(_) => {
            if existing_game.segments.len() != 0 {
                return Err(ServerFnError::new("only 1st segment should be init"));
            }
        }
        GameReplaySegment::Update(update_seg) => {
            let last_segment = existing_game.segments.last().unwrap();
            match last_segment {
                GameReplaySegment::Init(_) => {
                    if update_seg.idx != 0 {
                        return Err(ServerFnError::new("1st update segmnet needs idx=0"));
                    }
                }
                GameReplaySegment::Update(old_update) => {
                    if old_update.idx != update_seg.idx - 1 {
                        return Err(ServerFnError::new(
                            "segment idx do not match up - missing/duplicate",
                        ));
                    }
                }
                GameReplaySegment::GameOver => {
                    return Err(ServerFnError::new("already have old segmnet for game over"))
                }
            }
        }
        GameReplaySegment::GameOver => {
            log::info!("append segment game over");
        }
    };
    existing_game.segments.push(segment);
    GAME_REPLAY_DB.insert(&id, &existing_game).unwrap();
    Ok(())
}
