use crate::backend::server_info::GIT_VERSION;
use crate::database::tables::GAME_FULL_DB;
use crate::database::tables::GAME_IS_IN_PROGRESS_DB;
use crate::database::tables::GAME_SEGMENT_COUNT_DB;
use crate::database::tables::GAME_SEGMENT_DB;

use anyhow::Context;
use game::api::game_replay::GameId;
use game::api::game_replay::GameSegmentId;
use game::api::user::GuestInfo;
use game::api::user::UserProfile;
use game::api::websocket::GameSegmentCountReply;
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

pub fn create_new_game_id(
    _: (),
    _current_user_id: GuestInfo,
) -> anyhow::Result<GameId> {
    let who = _current_user_id.user_id;
    let mut rand = rand::thread_rng();
    let g = GameId {
        user_id: who,
        init_seed: rand.gen(),
        start_time: get_timestamp_now_nano(),
    };

    GAME_IS_IN_PROGRESS_DB.insert(&g, &false)?;
    GAME_SEGMENT_COUNT_DB.insert(&g, &0)?;
    Ok(g)
}

pub fn append_game_segment(
    (id, segment_json): (GameId, String),
    _current_user_id: GuestInfo,
) -> anyhow::Result<()> {
    let new_segment: GameReplaySegment =
        serde_json::from_str(&segment_json).expect("json never fail");

    let who = _current_user_id.user_id;
    if !who.eq(&id.user_id) {
        anyhow::bail!("no impersonate plz");
    }

    let existing_segment_count = GAME_SEGMENT_COUNT_DB
        .get(&id)?
        .context("game segment count not found!")?;
    let last_segment: Option<GameReplaySegment> = if existing_segment_count > 0 {
        let old_segment_id = GameSegmentId {
            game_id: id,
            segment_id: existing_segment_count - 1,
        };
        let maybe_segment =
            GAME_SEGMENT_DB.get(&old_segment_id)?.context("not found")?;
        Some(maybe_segment)
    } else {
        None
    };
    let last_state: Option<GameState> = if existing_segment_count > 0 {
        let maybe_gamestate = GAME_FULL_DB.get(&id)?.context("not found")?;
        Some(maybe_gamestate)
    } else {
        None
    };

    let new_segment_id = GameSegmentId {
        game_id: id,
        segment_id: existing_segment_count,
    };

    match &new_segment {
        GameReplaySegment::Init(_) => {
            if existing_segment_count != 0 {
                anyhow::bail!("only 1st segment should be init");
            }
        }
        GameReplaySegment::Update(update_seg) => {
            let last_segment = last_segment.context("last segment not found")?;
            match last_segment {
                GameReplaySegment::Init(_) => {
                    if update_seg.idx != 0 {
                        anyhow::bail!("1st update segmnet needs idx=0");
                    }
                }
                GameReplaySegment::Update(old_update) => {
                    if old_update.idx != update_seg.idx - 1 {
                        anyhow::bail!(
                            "segment idx do not match up - missing/duplicate"
                        );
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
    GAME_SEGMENT_COUNT_DB.insert(&id, &(existing_segment_count + 1))?;

    let new_game_state = match new_segment {
        GameReplaySegment::Init(replay) => {
            GameState::new(&replay.init_seed, replay.start_time)
        }
        GameReplaySegment::Update(slice) => {
            let mut last_state = last_state.context("no last state found")?;
            last_state.accept_replay_slice(&slice)?;
            last_state
        }
        GameReplaySegment::GameOver => {
            let last_state = last_state.context("no last state found")?;
            if !last_state.game_over {
                anyhow::bail!("got game over but reconstructed state is not game over")
            }
            last_state
        }
    };
    GAME_FULL_DB.insert(&id, &new_game_state)?;

    Ok(())
}

pub fn get_last_full_game_state(
    game_id: GameId,
    _current_user_id: GuestInfo,
) -> anyhow::Result<GameState> {
    Ok(GAME_FULL_DB.get(&game_id)?.context("game not found")?)
}

pub fn get_all_segments_for_game(
    game_id: GameId,
    _current_user_id: GuestInfo,
) -> anyhow::Result<Vec<GameReplaySegment>> {
    let mut r = vec![];
    for item in GAME_SEGMENT_DB
        .range(GameSegmentId::get_range_for_game(&game_id))
        .into_iter()
    {
        let (_segment_id, replay_segment) = item?;
        r.push(replay_segment);
    }
    r.sort_by_key(|s| match s {
        GameReplaySegment::Init(_) => -1,
        GameReplaySegment::Update(_s) => _s.idx as i32,
        GameReplaySegment::GameOver => i32::MAX,
    });
    Ok(r)
}

pub fn get_segment_count(
    game_id: GameId,
    _current_user_id: GuestInfo,
) -> anyhow::Result<GameSegmentCountReply> {
    let is_in_progress = GAME_IS_IN_PROGRESS_DB
        .get(&game_id)?
        .context("not fgound")?;
    let seg_count = GAME_SEGMENT_COUNT_DB.get(&game_id)?.context("not found")?;
    Ok(GameSegmentCountReply {
        is_in_progress,
        segment_count: seg_count,
    })
}
use game::api::websocket::GetAllGamesArg;
const PAGE_SIZE: usize = 9;

pub fn get_all_games(
    arg: GetAllGamesArg,
    _current_user_id: GuestInfo,
) -> anyhow::Result<Vec<(GameId, GameSegmentCountReply)>> {
    let load_all_games = || -> anyhow::Result<_> {
        let mut v = vec![];
        for game_id in GAME_IS_IN_PROGRESS_DB.iter().keys() {
            let game_id = game_id?;
            let r = get_segment_count(game_id, _current_user_id.clone())?;
            v.push((game_id, r));
        }
        Ok(v)
    };
    let load_games_for_user = |user: &uuid::Uuid| -> anyhow::Result<_> {
        let mut v = vec![];
        for game_id in GAME_IS_IN_PROGRESS_DB
            .range(GameId::get_range_for_user(user))
            .keys()
        {
            let game_id = game_id?;
            let r = get_segment_count(game_id, _current_user_id.clone())?;
            v.push((game_id, r));
        }
        Ok(v)
    };
    let sort_best = |mut v: Vec<(_, GameSegmentCountReply)>| -> anyhow::Result<_> {
        v.sort_by_key(|x| -(x.1.segment_count as i32));
        Ok(v)
    };
    let sort_recent = |mut v: Vec<(GameId, _)>| -> anyhow::Result<_> {
        v.sort_by_key(|x| -((x.0.start_time / 100000) as i32));
        Ok(v)
    };

    let mut v = match arg {
        GetAllGamesArg::BestGames => sort_best(load_all_games()?)?,
        GetAllGamesArg::RecentGames => sort_recent(load_all_games()?)?,
        GetAllGamesArg::MyBestGames => {
            sort_best(load_games_for_user(&_current_user_id.user_id)?)?
        }
        GetAllGamesArg::MyRecentGames => {
            sort_recent(load_games_for_user(&_current_user_id.user_id)?)?
        }
        GetAllGamesArg::BestGamesForPlayer(player_id) => {
            sort_best(load_games_for_user(&player_id)?)?
        }
        GetAllGamesArg::RecentGamesForPlayer(player_id) => {
            sort_recent(load_games_for_user(&player_id)?)?
        }
    };
    v.truncate(PAGE_SIZE);
    Ok(v)
}
