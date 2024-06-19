use std::arch::x86_64::_CMP_FALSE_OS;

use super::super::database::tables::GAME_REPLAY_DB;
use crate::backend::server_info::GIT_VERSION;
use crate::database::tables::GAME_IS_IN_PROGRESS_DB;
use crate::database::tables::GAME_SEGMENT_COUNT_DB;
use crate::database::tables::GAME_SEGMENT_DB;
use crate::database::tables::GAME_FULL_DB;

use anyhow::Context;
use game::api::game_replay::FullGameReplayDbRow;
use game::api::game_replay::GameId;
use game::api::game_replay::GameSegmentId;
use game::api::user::GuestInfo;
use game::api::user::UserProfile;
use game::tet::GameReplaySegment;
use game::tet::GameState;
use game::timestamp::get_timestamp_now_nano;
use rand::Rng;

pub fn get_profile(
    user_id: uuid::Uuid,
    _current_user_id: GuestInfo,
) -> anyhow::Result<UserProfile> {
    use crate::database::tables::get_user_profile;
    get_user_profile(&user_id)
}

pub fn git_version(_: (), _current_user_id: GuestInfo) -> anyhow::Result<String> {
    Ok(GIT_VERSION.clone())
}

pub fn get_full_game_replay(
    id: GameId,
    _current_user_id: GuestInfo,
) -> anyhow::Result<FullGameReplayDbRow> {
    Ok(GAME_REPLAY_DB
        .get(&id)
        .context("db get error")?
        .context("not fond error")?)
}

// pub fn get_all_full_game_replays(
//     _: (),
//     _current_user_id: GuestInfo,
// ) -> anyhow::Result<Vec<FullGameReplayDbRow>> {
//     let mut v = vec![];
//     for x in GAME_REPLAY_DB.iter() {
//         let y = x?.1;
//         v.push(y);
//     }
//     Ok(v)
// }

pub fn create_new_game_id(_: (), _current_user_id: GuestInfo) -> anyhow::Result<GameId> {
    let who = _current_user_id.user_id;
    let mut rand = rand::thread_rng();
    let g = GameId {
        user_id: who,
        init_seed: rand.gen(),
        start_time: get_timestamp_now_nano(),
    };
    
    GAME_IS_IN_PROGRESS_DB.insert(&g, &false)?;
    GAME_SEGMENT_COUNT_DB.insert(&g, &0);
    Ok(g)
}

pub fn append_game_segment(
    (id, segment_json): (GameId, String),
    _current_user_id: GuestInfo,
) -> anyhow::Result<()> {
    let new_segment :GameReplaySegment = serde_json::from_str(&segment_json).expect("json never fail");

    let who = _current_user_id.user_id;
    if !who.eq(&id.user_id) {
        anyhow::bail!("no impersonate plz");
    }

    let existing_segment_count = GAME_SEGMENT_COUNT_DB.get(&id)?.context("game segment count not found!")?;
    let last_segment: Option<GameReplaySegment> = if existing_segment_count > 0 {
        Some(GAME_SEGMENT_DB.get(&GameSegmentId{game_id:id, segment_id:existing_segment_count-1})?.expect("segment not found")?)
    } else {
        None
    };
    let last_state: Option<GameState> = if existing_segment_count > 0 {
        Some(GAME_FULL_DB.get(&GameSegmentId{game_id:id, segment_id:existing_segment_count-1})?.expect("game full not found")?)
    } else {
        None
    };
    
    let new_segment_id = 
        GameSegmentId{game_id:id, segment_id:existing_segment_count};
    

    match &new_segment {
        GameReplaySegment::Init(_) => {
            if existing_segment_count != 0 {
                anyhow::bail!("only 1st segment should be init");
            }
        }
        GameReplaySegment::Update(update_seg) => {
            let last_segment = last_segment
                .context("last segment not found")?;
            match last_segment {
                GameReplaySegment::Init(_) => {
                    if update_seg.idx != 0 {
                        anyhow::bail!("1st update segmnet needs idx=0");
                    }
                }
                GameReplaySegment::Update(old_update) => {
                    if old_update.idx != update_seg.idx - 1 {
                        anyhow::bail!("segment idx do not match up - missing/duplicate");
                    }
                }
                GameReplaySegment::GameOver => {
                    anyhow::bail!("already have old segmnet for game over");
                }
            }
        }
        GameReplaySegment::GameOver => {
            log::info!("append segment game over");
        }
    };
    let game_in_progress = match &new_segment {
        GameReplaySegment::Init(_) => true,
        GameReplaySegment::Update(_) => true,
        GameReplaySegment::GameOver => false,
    };
    GAME_IS_IN_PROGRESS_DB.insert(&id, &game_in_progress)?;
    GAME_SEGMENT_DB.insert(&new_segment_id, &new_segment)?;
    GAME_SEGMENT_COUNT_DB.insert(&id, &(existing_segment_count+1))?;

    let new_game_state = match new_segment {
        GameReplaySegment::Init(replay) => GameState::new(&replay.init_seed, replay.start_time),
        GameReplaySegment::Update(slice) => {
            let mut last_state = last_state.context("no last state found")?;
            last_state.accept_replay_slice(&slice)?;
            last_state
        },
        GameReplaySegment::GameOver =>{
            let last_state = last_state.context("no last state found")?;
            if !last_state.game_over {
                anyhow::bail!("got game over but reconstructed state is not game over")
            }
            last_state
        },
    };
    GAME_FULL_DB.insert(&new_segment_id, &new_game_state)?;

    Ok(())
}
