use serde::{Deserialize, Serialize};

use super::{GameOverReason, GameSeed, GameState, TetAction};



#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct GameReplayInit {
    pub init_seed: GameSeed,
    pub start_time: i64,
}

impl GameReplayInit {
    pub fn empty(seed: &GameSeed, start_time: i64) -> Self {
        Self {
            init_seed: *seed,
            start_time,
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum GameReplaySegmentData {
    Init(GameReplayInit),
    Update(GameReplayUpdate),
    GameOver(GameOverReason),
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct GameReplaySegment {
    pub data: GameReplaySegmentData,
    pub idx: u16,
    pub timestamp: i64,
}



#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct GameReplayUpdate {
    pub event_timestamp: i64,
    pub new_seed: GameSeed,
    pub new_garbage_recv: u16,
    pub new_garbage_applied: u16,
    pub event: GameReplayEvent,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct GameReplayEvent {
    pub action: TetAction,
}

pub fn segments_to_states(all_segments: &Vec<GameReplaySegment>) -> Vec<GameState> {
    let mut current_state = match all_segments.get(0).map(|x| x.data) {
        Some(GameReplaySegmentData::Init(_replay)) => {
            GameState::new(&_replay.init_seed, _replay.start_time)
        }
        _ => {
            log::info!("got no init segment");
            return vec![];
        }
    };
    let mut all_states = vec![];
    all_states.push(current_state.clone());
    for segment in &all_segments[1..] {
        if let Err(e) = current_state.accept_replay_segment(segment) {
            log::error!("accept replay segment failed!!! {e:?}");
            return all_states;
        }
        all_states.push(current_state.clone());
    }
    all_states
}
