use game::{
    api::{game_match::{GameMatch, UserAndMatchId, UserAndMatchResult}, game_replay::{GameId, GameSegmentId}},
    tet::{GameReplaySegment, GameState},
};

use super::config::SERVER_DATA_PATH;
use anyhow::Context;

use once_cell::sync::Lazy;

pub static TABLES_DB: Lazy<sled::Db> =
    Lazy::new(|| sled::open(format!("{SERVER_DATA_PATH}/tables.sled")).unwrap());

use game::api::user::UserProfile;

pub static USER_PROFILE_DB: Lazy<typed_sled::Tree<uuid::Uuid, UserProfile>> =
    Lazy::new(|| {
        typed_sled::Tree::<uuid::Uuid, UserProfile>::open(&TABLES_DB, "user_profile_v1")
    });

pub static GAME_IS_IN_PROGRESS_DB: Lazy<typed_sled::Tree<GameId, bool>> =
    Lazy::new(|| {
        typed_sled::Tree::<GameId, bool>::open(&TABLES_DB, "game_is_in_progress_v1")
    });

pub static GAME_SEGMENT_COUNT_DB: Lazy<typed_sled::Tree<GameId, u32>> =
    Lazy::new(|| {
        typed_sled::Tree::<GameId, u32>::open(&TABLES_DB, "game_segment_count_v1")
    });

pub static GAME_SEGMENT_DB: Lazy<typed_sled::Tree<GameSegmentId, GameReplaySegment>> =
    Lazy::new(|| {
        typed_sled::Tree::<GameSegmentId, GameReplaySegment>::open(
            &TABLES_DB,
            "game_segment_db_v1",
        )
    });

pub static CUSTOM_GAME_BOARD_DB: Lazy<typed_sled::Tree<String, GameState>> =
    Lazy::new(|| {
        typed_sled::Tree::<String, GameState>::open(&TABLES_DB, "custom_game_board_v1")
    });

pub static GAME_FULL_DB: Lazy<
        typed_sled::Tree<GameId, GameState>
    > = Lazy::new(|| {
            typed_sled::Tree::<_,_>::open(&TABLES_DB, "game_full_v2")
});

pub fn get_user_profile(uuid: &uuid::Uuid) -> anyhow::Result<UserProfile> {
    Ok(USER_PROFILE_DB
        .get(uuid)
        .context("operation failed")?
        .context("user profile not found")?)
}

pub fn random_word() -> String {
    random_word::gen(random_word::Lang::De).to_string()
}

pub fn get_or_create_user_profile(uuid: &uuid::Uuid) -> anyhow::Result<UserProfile> {
    if let Ok(u) = get_user_profile(uuid) {
        return Ok(u);
    }
    let new: UserProfile = UserProfile {
        display_name: random_word(),
    };
    USER_PROFILE_DB.insert(uuid, &new).context("cannot write")?;
    Ok(new)
}

// ===

pub static GAME_MATCH_DB: Lazy<
        typed_sled::Tree<uuid::Uuid, GameMatch>
    > = Lazy::new(|| {
            typed_sled::Tree::<_,_>::open(&TABLES_DB, "game_match_v2")
});

pub static GAME_MATCH_IS_IN_PROGRESS_DB: Lazy<
        typed_sled::Tree<uuid::Uuid, bool>
    > = Lazy::new(|| {
            typed_sled::Tree::<_,_>::open(&TABLES_DB, "game_match_is_in_progress_v2")
});

pub static GAME_MATCHES_FOR_USER_DB: Lazy<
        typed_sled::Tree<UserAndMatchId, UserAndMatchResult>
    > = Lazy::new(|| {
            typed_sled::Tree::<_,_>::open(&TABLES_DB, "GAME_MATCHES_FOR_USER_DB_v1")
});
