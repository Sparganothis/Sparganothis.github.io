mod game_state;
mod matrix;
mod random;
mod rot;
mod tet;

pub use game_state::{
    segments_to_states, CurrentPcsInfo, GameOverReason, GameReplaySegment,
    GameReplaySlice, GameState, HoldPcsInfo,
};
pub use matrix::{BoardMatrix, BoardMatrixHold, BoardMatrixNext, CellValue};
pub use random::{get_random_seed, GameSeed};
pub use rot::RotState;
pub use tet::{Tet, TetAction};

#[cfg(test)]
pub mod tests {
    use super::super::timestamp::get_timestamp_now_nano;
    use super::*;
    use game_state::{GameReplaySegment, GameState};
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

        let expected_seed = [
            112, 108, 244, 165, 170, 133, 13, 105, 29, 155, 63, 142, 88, 10, 124, 69,
            11, 204, 19, 247, 111, 162, 42, 131, 23, 33, 17, 116, 136, 66, 83, 241,
        ];
        assert_eq!(state.seed, expected_seed,);

        state.apply_action_if_works(TetAction::HardDrop, 1).unwrap();

        let expected_seed = [
            232, 114, 216, 90, 137, 42, 115, 14, 77, 126, 249, 220, 176, 41, 220, 245,
            8, 135, 202, 145, 162, 178, 110, 179, 247, 50, 34, 76, 254, 161, 54, 31,
        ];
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
                    state1 = res1.unwrap();
                    state2 = res2.unwrap();
                }

                if state1.game_over() {
                    break;
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
            let mut _slices = vec![];

            loop {
                let action = TetAction::random();
                let res = active_game.try_action(action, get_timestamp_now_nano());
                if let Ok(new_active_game) = res {
                    active_game = new_active_game;
                } else {
                    continue;
                }
                if let GameReplaySegment::Update(ref update) = active_game.last_segment
                {
                    _slices.push(update.clone());
                }
                if active_game.game_over() {
                    break;
                }
            }

            for slice in _slices {
                log::info!("accept replay slice: {slice:?}");
                passive_game.accept_replay_slice(&slice).unwrap();
            }

            // assert_eq!(active_game, passive_game);
        }
    }
}
