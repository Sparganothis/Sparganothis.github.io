mod game_state;
mod matrix;
mod random;
mod rot;
mod tet;
mod replay_segment;

pub use game_state::{
    CurrentPcsInfo, GameOverReason,
    GameState, HoldPcsInfo,
};

pub use matrix::{BoardMatrix, BoardMatrixHold, BoardMatrixNext, CellValue};
pub use random::{get_random_seed, GameSeed};
pub use rot::RotState;
pub use tet::{Tet, TetAction};
pub use replay_segment::{GameReplaySegment,segments_to_states, GameReplaySegmentData};


#[cfg(test)]
pub mod tests {
    use super::super::timestamp::get_timestamp_now_nano;
    use super::*;
    use replay_segment::GameReplaySegmentData;
    use tet::TetAction;
    // use pretty_assertions::assert_eq;
    use wasm_bindgen_test::*;

    #[test]
    #[wasm_bindgen_test]
    pub fn random_have_pinned_results() {
        let seed = [0; 32];
        let mut state = GameState::new(&seed, 0);

        // let expected_seed = [0;32];
        // assert_eq!(expected_seed, state.seed);

        state.apply_action_if_works(TetAction::SoftDrop, 0).unwrap();

        let expected_seed = [107, 116, 72, 173, 199, 119, 126, 243, 27, 37, 163, 151, 25, 37, 86, 218, 222, 178, 194, 132, 176, 191, 126, 175, 209, 124, 72, 136, 175, 59, 102, 182];
        assert_eq!(state.seed, expected_seed,);

        state.apply_action_if_works(TetAction::HardDrop, 1).unwrap();

        let expected_seed = [251, 180, 138, 18, 29, 170, 97, 61, 132, 13, 186, 191, 92, 60, 157, 46, 90, 117, 52, 227, 105, 37, 235, 227, 171, 31, 91, 121, 30, 182, 164, 248];
        assert_eq!(expected_seed, state.seed);
    }

    #[test]
    #[wasm_bindgen_test]
    pub fn active_game_is_deterministic() {
        for i in 0..255 {
            let seed = [i; 32];
            let mut state1 = GameState::new(&seed, get_timestamp_now_nano());
            let mut state2 = GameState::new(&seed, state1.start_time);

            loop {
                let action = TetAction::random();
                let t2 = get_timestamp_now_nano();
                let res1 = state1.try_action(action, t2).map_err(|_| "bad");
                let res2 = state2.try_action(action, t2).map_err(|_| "bad");
                assert_eq!(res1, res2);
                if res1.is_ok() {
                    let (_new_state1, new_segments1) = res1.unwrap();
                    let (_new_state2, _new_segments2) = res2.unwrap();

                    state1 = _new_state1;
                    state2 = _new_state2;
                    
                    if state1.game_over() {
                        let last_segment = new_segments1.last().unwrap();
                        let GameReplaySegmentData::GameOver(_reason) = last_segment.data else {
                            panic!("last segment not game over!")
                        };
                        assert!(_reason == GameOverReason::Knockout, "game ended other than knockout");
                        break;
                    }
                }

            }
        }
    }

    #[test]
    #[wasm_bindgen_test]
    fn passive_game_tracks_active_one() {
        for i in 0..255 {
            let seed = [i; 32];

            let mut active_game = GameState::new(&seed, get_timestamp_now_nano());
            let mut passive_game = GameState::new(&seed, active_game.start_time);
            let mut _segments = vec![];

            loop {
                let action = TetAction::random();
                let res = active_game.try_action(action, get_timestamp_now_nano());
                if let Ok((new_active_game, mut new_segments)) = res {
                    active_game = new_active_game;
                    _segments.append(&mut new_segments);
                } else {
                    continue;
                }
                if active_game.game_over() {
                    break;
                }
            }

            for segment in _segments {
                log::info!("accept replay slice: {segment:?}");
                passive_game.accept_replay_segment(&segment).unwrap();
            }

            assert_eq!(active_game, passive_game);
        }
    }
}
