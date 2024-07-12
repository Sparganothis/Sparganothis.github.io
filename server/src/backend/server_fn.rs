use crate::backend::server_info::GIT_VERSION;
use crate::database::tables::*;

use anyhow::Context;
use game::api::game_match::GameMatch;
use game::api::game_match::GameMatchType;
use game::api::game_replay::GameId;
use game::api::game_replay::GameSegmentId;
use game::api::table_paginate::TablePaginateDirection;
use game::api::user::UserProfile;
use game::api::user_settings::UserSettingType;
use game::api::websocket::GameSegmentCountReply;
use game::api::websocket::GetMatchListArg;
use game::bot::get_bot_from_id;
use game::bot::get_bot_id;
use game::tet::GameOverReason;
use game::tet::GameReplaySegment;
use game::tet::GameState;
use game::timestamp::get_timestamp_now_nano;
use rand::Rng;

pub fn get_profile(
    user_id: uuid::Uuid,
    _current_session: CurrentSessionInfo,
) -> anyhow::Result<UserProfile> {
    use crate::database::tables::get_user_profile;
    get_user_profile(&user_id)
}

pub fn git_version(_: (), _session_info: CurrentSessionInfo) -> anyhow::Result<String> {
    Ok(GIT_VERSION.to_string())
}

pub fn create_new_game_id(
    _: (),
    _current_session: CurrentSessionInfo,
) -> anyhow::Result<GameId> {
    for existing_game in GAME_IS_IN_PROGRESS_DB.range(GameId::get_range_for_user(
        &_current_session.guest_id.user_id,
    )) {
        let (old_game_id, is_in_progress) = existing_game?;
        if is_in_progress {
            GAME_IS_IN_PROGRESS_DB.insert(&old_game_id, &false)?;
        }
    }

    let mut rand = rand::thread_rng();
    let g = GameId {
        user_id: _current_session.guest_id.user_id,
        init_seed: rand.gen(),
        start_time: get_timestamp_now_nano(),
    };

    GAME_IS_IN_PROGRESS_DB.insert(&g, &true)?;
    GAME_SEGMENT_COUNT_DB.insert(&g, &0)?;
    Ok(g)
}

pub fn append_game_segment(
    (id, segment_json): (GameId, String),
    _current_session: CurrentSessionInfo,
) -> anyhow::Result<Option<GameOverReason>> {
    _check_is_global_locked(_current_session, id)?;
    let new_segment: GameReplaySegment =
        serde_json::from_str(&segment_json).expect("json never fail");

    let who = _current_session.guest_id.user_id;
    if !who.eq(&id.user_id) {
        anyhow::bail!("no impersonate plz");
    }
    do_append_game_segment(id, new_segment, _current_session)
}

pub fn append_bot_game_segment(
    (id, segment_json): (GameId, String),
    _current_session: CurrentSessionInfo,
) -> anyhow::Result<Option<GameOverReason>> {
    let new_segment: GameReplaySegment =
        serde_json::from_str(&segment_json).expect("json never fail");

    let who = _current_session.guest_id.user_id;
    let bot_name = get_bot_from_id(id.user_id)?;
    // cchecck that the game id is listed for a mamtcch  vs. this bot
    // and the other player in the mamtch is "who"

    let match_id = GAME_MATCH_FOR_GAME_ID_DB
        .get(&id)?
        .context("no mmatch found for ur game")?;
    let match_info = GAME_MATCH_DB
        .get(&match_id)?
        .context("no mmatchc found for ur game")?;

    match match_info.type_ {
        GameMatchType::ManVsCar(match_bot_name) => {
            if match_bot_name != bot_name {
                anyhow::bail!("wrong bot type for this matcch");
            }
            if who != match_info.users[0] && who != match_info.users[1] {
                anyhow::bail!("no impersonating the bots plz");
            }
        }
        _ => anyhow::bail!("wrong match type for this game!"),
    }

    do_append_game_segment(id, new_segment, _current_session)
}

fn do_append_game_segment(
    id: GameId,
    new_segment: GameReplaySegment,
    _current_session: CurrentSessionInfo,
) -> anyhow::Result<Option<GameOverReason>> {
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
                    if old_update.idx + 1 != update_seg.idx {
                        anyhow::bail!(
                            "segment idx do not match up - missing/duplicate"
                        );
                    }
                }
                GameReplaySegment::GameOver(_) => {
                    anyhow::bail!("already have old segmnet for game over");
                }
            }
        }
        GameReplaySegment::GameOver(_) => {
            log::info!("append segment game over");
        }
    };
    let game_in_progress = match &new_segment {
        GameReplaySegment::Init(_) => true,
        GameReplaySegment::Update(_) => true,
        GameReplaySegment::GameOver(_) => false,
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
        GameReplaySegment::GameOver(_) => {
            let last_state = last_state.context("no last state found")?;
            if !last_state.game_over {
                anyhow::bail!("got game over but reconstructed state is not game over")
            }
            last_state
        }
    };
    GAME_FULL_DB.insert(&id, &new_game_state)?;

    Ok(is_game_over_because_sommething(id, _current_session)?)
}

fn is_game_over_because_sommething(game_id: GameId,  _current_session: CurrentSessionInfo,) -> anyhow::Result<Option<GameOverReason>> {{

    if let Some(match_) = GAME_MATCH_FOR_GAME_ID_DB.get(&game_id)? {
        if let Some(match_info) = GAME_MATCH_DB.get(&match_)?{
            match match_info.type_ {
                GameMatchType::ManVsCar(_) | GameMatchType::_1v1 => {
                    if other_game_lost(&game_id, &match_info).unwrap_or(false) {
                        return Ok(Some(GameOverReason::Win))
                    }
                },

                GameMatchType::_40lines => todo!(),
                GameMatchType::_10v10 => todo!(),
                GameMatchType::_4v4 => todo!(),
            }
        }
    }


    Ok(None)
}}

fn other_game_lost(game_id: &GameId, match_info: &GameMatch) -> anyhow::Result<bool> {
    for user in match_info.users.iter() {
        if *user != game_id.user_id {
            let other_game_id = GameId {
                user_id: *user,
                init_seed: match_info.seed,
                start_time: match_info.time,
            };
            let other_in_progress = GAME_IS_IN_PROGRESS_DB.get(&other_game_id)?.context("other match not found")?;
            return Ok(!other_in_progress)
        }
    }
    anyhow::bail!("oculd not find other game, am i playuing myself??")
}

pub fn get_last_full_game_state(
    game_id: GameId,
    _current_session: CurrentSessionInfo,
) -> anyhow::Result<Option<GameState>> {
    Ok(GAME_FULL_DB.get(&game_id)?)
}

pub fn get_all_segments_for_game(
    game_id: GameId,
    _current_session: CurrentSessionInfo,
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
        GameReplaySegment::GameOver(_) => i32::MAX,
    });
    Ok(r)
}

pub fn get_segment_count(
    game_id: GameId,
    _current_session: CurrentSessionInfo,
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
const PAGE_SIZE: usize = 24;

pub fn get_all_games(
    (arg, _pag): (GetAllGamesArg, TablePaginateDirection<GameId>),
    _current_session: CurrentSessionInfo,
) -> anyhow::Result<Vec<(GameId, GameSegmentCountReply)>> {
    let load_all_games = || -> anyhow::Result<_> {
        let mut v = vec![];
        for game_id in GAME_IS_IN_PROGRESS_DB.iter().keys() {
            let game_id = game_id?;
            let r = get_segment_count(game_id, _current_session)?;
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
            let r = get_segment_count(game_id, _current_session)?;
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
            sort_best(load_games_for_user(&_current_session.guest_id.user_id)?)?
        }
        GetAllGamesArg::MyRecentGames => {
            sort_recent(load_games_for_user(&_current_session.guest_id.user_id)?)?
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

#[allow(unused_variables)]
pub fn get_all_gustom(
    _pag: TablePaginateDirection<String>,
    _current_session: CurrentSessionInfo,
) -> anyhow::Result<Vec<(String, GameState)>> {
    let mut v = vec![];
    for x in CUSTOM_GAME_BOARD_DB.iter() {
        let x = x?;
        v.push(x);
    }
    Ok(v)
}

pub fn get_gustom_game(
    arg: String,
    _current_session: CurrentSessionInfo,
) -> anyhow::Result<GameState> {
    Ok(CUSTOM_GAME_BOARD_DB.get(&arg)?.context("not found")?)
}

pub fn update_custom_game(
    arg: (String, GameState),
    _current_session: CurrentSessionInfo,
) -> anyhow::Result<()> {
    CUSTOM_GAME_BOARD_DB.insert(&arg.0, &arg.1)?;
    Ok(())
}

pub fn random_word2(
    _: (),
    _current_session: CurrentSessionInfo,
) -> anyhow::Result<String> {
    Ok(random_word())
}

pub struct MatchMakingItem {
    channel: tokio::sync::mpsc::Sender<(uuid::Uuid, GameMatch)>,
    player_id: uuid::Uuid,
}

pub static MATCH_MAKING_QUEUE: Lazy<MatchMakingQueue> =
    Lazy::new(|| MatchMakingQueue {
        v: Arc::new(Mutex::new(HashMap::new())),
    });

use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

use super::websocket::CurrentSessionInfo;
pub struct MatchMakingQueue {
    v: Arc<Mutex<HashMap<uuid::Uuid, MatchMakingItem>>>,
}

pub async fn start_match(
    _type: GameMatchType,
    _current_session: CurrentSessionInfo,
) -> anyhow::Result<(uuid::Uuid, GameMatch)> {
    // Ok(uuid::Uuid::nil(), GameMatch)

    match _type {
        GameMatchType::_1v1 => start_new_1v1_match(_current_session).await,
        GameMatchType::ManVsCar(bot_type) => {
            start_new_man_vs_car_match(bot_type, _current_session).await
        }
        GameMatchType::_40lines => todo!(),
        GameMatchType::_10v10 => todo!(),
        GameMatchType::_4v4 => todo!(),
    }
}

async fn start_new_man_vs_car_match(
    bot_type: String,
    _current_session: CurrentSessionInfo,
) -> anyhow::Result<(uuid::Uuid, GameMatch)> {
    let bot_player_id = get_bot_id(&bot_type).context("bot not found")?;

    let new_match = GameMatch {
        seed: (&mut rand::thread_rng()).gen(),
        time: get_timestamp_now_nano(),
        users: vec![_current_session.guest_id.user_id, bot_player_id],
        title: format!(
            "1v1 {} vs. {}",
            bot_player_id, _current_session.guest_id.user_id
        ),
        type_: GameMatchType::ManVsCar(bot_type),
    };
    let new_match_id = uuid::Uuid::new_v4();
    GAME_MATCH_DB.insert(&new_match_id, &new_match)?;

    create_db_match_entry(new_match_id, &new_match)?;

    Ok((new_match_id, new_match))
}

async fn start_new_1v1_match(
    _current_session: CurrentSessionInfo,
) -> anyhow::Result<(uuid::Uuid, GameMatch)> {
    let mut _waiting_for_match: Option<_> = None;
    let mut _got_new_match: Option<_> = None;
    {
        let mut q = MATCH_MAKING_QUEUE.v.lock().await;
        if q.is_empty() {
            // creezi chan, te bagi in el
            let (tx, rx) = tokio::sync::mpsc::channel(1);
            let player_id = _current_session.guest_id.user_id;
            let new_item = MatchMakingItem {
                channel: tx,
                player_id,
            };

            _waiting_for_match = Some(rx);
            q.insert(player_id, new_item);
        } else {
            if q.contains_key(&_current_session.guest_id.user_id) {
                anyhow::bail!("another game is already in matchmaking!");
            } else {
                let k = *q.keys().next().unwrap();
                let other_player = q.remove(&k).unwrap();
                let new_match = GameMatch {
                    seed: (&mut rand::thread_rng()).gen(),
                    time: get_timestamp_now_nano(),
                    users: vec![
                        other_player.player_id,
                        _current_session.guest_id.user_id,
                    ],
                    title: format!(
                        "1v1 {} vs. {}",
                        other_player.player_id, _current_session.guest_id.user_id
                    ),
                    type_: GameMatchType::_1v1,
                };
                let new_match_id = uuid::Uuid::new_v4();
                GAME_MATCH_DB.insert(&new_match_id, &new_match)?;
                other_player
                    .channel
                    .send((new_match_id, new_match.clone()))
                    .await?;
                _got_new_match = Some((new_match_id, new_match));
            }
        }
    }
    if let Some(mut waiting_rx) = _waiting_for_match {
        if let Some(match_info) = waiting_rx.recv().await {
            create_db_match_entry(match_info.0, &match_info.1)?;
            Ok(match_info)
        } else {
            anyhow::bail!("cannot read from channel");
        }
    } else {
        let r = _got_new_match.context("never happens")?;

        create_db_match_entry(r.0, &r.1)?;

        Ok(r)
    }
}

fn create_db_match_entry(
    match_id: uuid::Uuid,
    match_info: &GameMatch,
) -> anyhow::Result<()> {
    let gameinfo_0 = GameId {
        user_id: match_info.users[0],
        init_seed: match_info.seed,
        start_time: match_info.time,
    };
    let gameinfo_1 = GameId {
        user_id: match_info.users[1],
        init_seed: match_info.seed,
        start_time: match_info.time,
    };

    GAME_IS_IN_PROGRESS_DB.insert(&gameinfo_0, &true)?;
    GAME_SEGMENT_COUNT_DB.insert(&gameinfo_0, &0)?;
    GAME_MATCH_FOR_GAME_ID_DB.insert(&gameinfo_0, &match_id)?;

    GAME_IS_IN_PROGRESS_DB.insert(&gameinfo_1, &true)?;
    GAME_SEGMENT_COUNT_DB.insert(&gameinfo_1, &0)?;
    GAME_MATCH_FOR_GAME_ID_DB.insert(&gameinfo_1, &match_id)?;

    Ok(())
}

pub fn get_match_list(
    (_arg, _pag): (GetMatchListArg, TablePaginateDirection<uuid::Uuid>),
    _current_session: CurrentSessionInfo,
) -> anyhow::Result<Vec<(uuid::Uuid, GameMatch)>> {
    let mut v = vec![];
    for x in GAME_MATCH_DB.iter() {
        let (uuid, _match) = x?;
        v.push((uuid, _match));
    }
    Ok(v)
}

pub fn get_match_info(
    match_id: uuid::Uuid,
    _current_session: CurrentSessionInfo,
) -> anyhow::Result<GameMatch> {
    GAME_MATCH_DB.get(&match_id)?.context(".not found")
}

pub fn get_user_setting(
    setting_name: UserSettingType,
    _current_session: CurrentSessionInfo,
) -> anyhow::Result<Vec<u8>> {
    match USER_SETTING_DB.get(&(_current_session.guest_id.user_id, setting_name))? {
        Some(s) => Ok(s),
        None => Ok(vec![]),
    }

    // match setting_name{
    //     UserSettingType::SoundSetting(s) => match s {
    //         game::api::user_settings::SoundSettingType::DisableAllSounds => todo!(),
    //         game::api::user_settings::SoundSettingType::DisableMenuMusic => todo!(),
    //         game::api::user_settings::SoundSettingType::MenuMusicVolume => todo!(),
    //         game::api::user_settings::SoundSettingType::MenuSoundsVolume => todo!(),
    //     },
    //     UserSettingType::ControlSetting(s) => match s {
    //         game::api::user_settings::ControlSettingType::IHaveADHD => todo!(),
    //     },
    //     UserSettingType::ThemeSetting(s) => match s {
    //         game::api::user_settings::ThemeSettingType::BackgroundColor => todo!(),
    //     },
    // }
}

pub fn set_user_setting(
    (setting_name, setting_val): (UserSettingType, Vec<u8>),
    _current_session: CurrentSessionInfo,
) -> anyhow::Result<()> {
    if setting_val.len() > 100 {
        anyhow::bail!("too many bytes pls!");
    }
    USER_SETTING_DB.insert(
        &(_current_session.guest_id.user_id, setting_name),
        &setting_val,
    )?;

    Ok(())
}

pub fn set_global_play_lock(
    (lock, lock_for_game_id): (bool, Option<GameId>),
    _current_session: CurrentSessionInfo,
) -> anyhow::Result<()> {
    if lock {
        _lock_global_for_game(
            _current_session,
            lock_for_game_id.context("game not given")?,
        )
    } else {
        _unlock_global_lock_id(_current_session)
    }
}

pub struct GlobalGameLock {
    v: Arc<std::sync::Mutex<HashMap<uuid::Uuid, (GameId, CurrentSessionInfo)>>>,
}

pub static GLOBAL_GAME_LOCKS: Lazy<GlobalGameLock> = Lazy::new(|| GlobalGameLock {
    v: Arc::new(std::sync::Mutex::new(HashMap::new())),
});

fn _lock_global_for_game(
    _current_session: CurrentSessionInfo,
    game_id: GameId,
) -> anyhow::Result<()> {
    {
        match GLOBAL_GAME_LOCKS.v.lock() {
            Ok(mut g) => {
                let is_already_in = g.contains_key(&_current_session.guest_id.user_id);
                if is_already_in {
                    anyhow::bail!(
                        "already conneccted in another session; pls go there."
                    )
                }
                g.insert(
                    _current_session.guest_id.user_id,
                    (game_id, _current_session),
                );
            }
            Err(e) => {
                let e_str = format!("e: {:?}", e);
                anyhow::bail!("{e_str}");
            }
        }
    }
    Ok(())
}

pub fn _unlock_global_lock_id(
    _current_session: CurrentSessionInfo,
) -> anyhow::Result<()> {
    {
        match GLOBAL_GAME_LOCKS.v.lock() {
            Ok(mut g) => {
                g.remove(&_current_session.guest_id.user_id);
            }
            Err(e) => {
                let e_str = format!("e: {:?}", e);
                anyhow::bail!("{e_str}");
            }
        }
    }
    Ok(())
}

fn _check_is_global_locked(
    _current_session: CurrentSessionInfo,
    _current_game_id: GameId,
) -> anyhow::Result<()> {
    {
        match GLOBAL_GAME_LOCKS.v.lock() {
            Ok(g) => {
                let existing = g.get(&_current_session.guest_id.user_id);
                if existing.is_none() {
                    anyhow::bail!("you forgot to ask for GameLock with the message SetGlobalPlayLock");
                }
                let (game_id, locked_session) = existing.unwrap();
                if locked_session.websocket_id != _current_session.websocket_id {
                    anyhow::bail!("gamelock already set for different websocket id");
                }
                if *game_id != _current_game_id {
                    anyhow::bail!("gamelock already set for different game id");
                }
            }
            Err(e) => {
                let e_str = format!("e: {:?}", e);
                anyhow::bail!("{e_str}");
            }
        }
    }
    Ok(())
}
