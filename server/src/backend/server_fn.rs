use super::super::database::tables::GAME_REPLAY_DB;
use crate::backend::server_info::GIT_VERSION;
use anyhow::Context;
use game::api::game_replay::FullGameReplayDbRow;
use game::api::game_replay::GameId;
use game::api::user::GuestInfo;
use game::api::user::UserProfile;
use game::tet::GameReplaySegment;
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

pub fn get_all_full_game_replays(
    _: (),
    _current_user_id: GuestInfo,
) -> anyhow::Result<Vec<FullGameReplayDbRow>> {
    let mut v = vec![];
    for x in GAME_REPLAY_DB.iter() {
        let y = x?.1;
        v.push(y);
    }
    Ok(v)
}

pub fn create_new_game_id(_: (), _current_user_id: GuestInfo) -> anyhow::Result<GameId> {
    let who = _current_user_id.user_id;
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
    GAME_REPLAY_DB.insert(&g, &row)?;
    Ok(g)
}

pub fn append_game_segment(
    (id, segment_json): (GameId, String),
    _current_user_id: GuestInfo,
) -> anyhow::Result<()> {
    let segment: GameReplaySegment = serde_json::from_str(&segment_json).expect("json never fail");

    let who = _current_user_id.user_id;
    if !who.eq(&id.user_id) {
        anyhow::bail!("no impersonate plz");
    }

    let mut existing_game = GAME_REPLAY_DB
        .get(&id)
        .context("get err")?
        .context("not found")?;

    match &segment {
        GameReplaySegment::Init(_) => {
            if existing_game.segments.len() != 0 {
                anyhow::bail!("only 1st segment should be init");
            }
        }
        GameReplaySegment::Update(update_seg) => {
            let last_segment = existing_game
                .segments
                .last()
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
    existing_game.segments.push(segment);
    GAME_REPLAY_DB
        .insert(&id, &existing_game)
        .context("insert")?;
    Ok(())
}
