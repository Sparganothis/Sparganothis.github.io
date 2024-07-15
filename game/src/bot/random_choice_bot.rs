use std::collections;

use crate::tet::{GameState, TetAction};

use super::TetBot;

pub struct RandomChoiceBot;


use once_cell::sync::Lazy;

pub static ALL_MOVE_CHAINS: Lazy<Vec<Vec<TetAction>>> =
    Lazy::new(|| make_all_move_chains());
pub fn get_all_move_chains() ->  Vec<Vec<TetAction>> {
    ALL_MOVE_CHAINS.clone()
}
fn make_all_move_chains() -> Vec<Vec<TetAction>> {
    let rotate_actions = vec![
        vec![],
        vec![TetAction::RotateLeft],
        vec![TetAction::RotateRight],
        vec![TetAction::RotateRight, TetAction::RotateRight],
        vec![TetAction::Hold],
        vec![TetAction::Hold, TetAction::RotateLeft],
        vec![TetAction::Hold, TetAction::RotateRight],
        vec![
            TetAction::Hold,
            TetAction::RotateRight,
            TetAction::RotateRight,
        ],
    ];

    let mut all_action_chains = vec![];
    for rotate_action_chain in rotate_actions {
        for is_left in 0..=1 {
            for move_length in 0..=5 {
                let mut new_action_chain = rotate_action_chain.clone();
                for _ in 0..move_length {
                    let new_move_action = if is_left == 1 {
                        TetAction::MoveLeft
                    } else {
                        TetAction::MoveRight
                    };
                    new_action_chain.push(new_move_action);
                }
                new_action_chain.push(TetAction::HardDrop);
                all_action_chains.push(new_action_chain);
            }
        }
    }
    all_action_chains
}

pub fn get_action_chain_score<F>(
    game_state: &GameState,
    action_chain: &Vec<TetAction>,
    f: F,
    cache: &mut collections::HashMap::<Vec<TetAction>, anyhow::Result<GameState>>,
) -> anyhow::Result<f64>
where
    F: Fn(&GameState, &GameState) -> anyhow::Result<f64>,
{
    let old_state = game_state;
    let mut _current_chain = vec![];
    let mut _prev_chain = vec![];
    for action in action_chain.clone() {
        _prev_chain = _current_chain.clone();
        _current_chain.push(action);
        if let Some(_existing_entry) = cache.get(&_current_chain) {
            continue;
        }

        let new_result = match cache.get(&_prev_chain).expect("prev chain must exist") {
            Ok(ref state) => {
                let mut state = state.clone();
                let r = state.apply_action_if_works(action, 0);
                if state.game_over() {
                    anyhow::Result::Err(anyhow::anyhow!("game over"))
                } else {
                    r.map(|_x| state)
                }
            },
            Err(e) => anyhow::Result::Err(anyhow::anyhow!("{e}"))
        };

        cache.insert(_current_chain.clone(), new_result);
    }

    let state = cache.get(action_chain).expect("result not found in cache after iterating");
    let mut state = match state {
        Err(e) => anyhow::bail!("{e}"),
        Ok(state) => state
    }.clone();

    if state.game_over() {
        anyhow::bail!("action leads to game over");
    }

    if let Some(c_pcsc) = state.current_pcs {
        state.main_board.delete_piece(&c_pcsc)?;
        let score = f(old_state, &state)?;
        return Ok(score);
    }
    anyhow::bail!("error in generating board");
}

pub fn get_best_move_for_score_fn<F>(
    game_state: &GameState,
    f: F,
) -> anyhow::Result<Vec<crate::tet::TetAction>>
where
    F: Fn(&GameState, &GameState) -> anyhow::Result<f64>,
{
    let game_state = {
        if game_state.game_over() {
            return Ok(vec![]);
        }
        let mut game_state = game_state.clone();
        game_state.replay.replay_slices.clear();
        game_state
    };
    let mut all_action_chains = get_all_move_chains();

    use rand::seq::SliceRandom;
    use rand::thread_rng;
    let mut rng = thread_rng();
    all_action_chains.shuffle(&mut rng);
    all_action_chains.sort_by_key(|k| k.len());

    let mut best_action_chain = vec![TetAction::SoftDrop];
    let mut best_acction_score = f64::MIN;
    let mut action_result_cache = collections::HashMap::<Vec<TetAction>, anyhow::Result<GameState>>::new();
    action_result_cache.insert(vec![], Ok(game_state.clone()));
    for act in all_action_chains {
        if let Ok(sccore) = get_action_chain_score(&game_state, &act, &f, &mut action_result_cache) {
            if sccore > best_acction_score {
                best_acction_score = sccore;
                best_action_chain = act.clone();
            }
        }
    }

    Ok(best_action_chain)
}

fn get_score_for_board(
    old_state: &GameState,
    new_state: &GameState,
) -> anyhow::Result<f64> {
    let line_diff = new_state.total_lines - old_state.total_lines;

    Ok(-0.51 * (new_state.main_board.get_height() as f64) + 0.76 * (line_diff as f64))
}

impl TetBot for RandomChoiceBot {
    fn choose_move(
        &self,
        game_state: &crate::tet::GameState,
    ) -> anyhow::Result<Vec<crate::tet::TetAction>> {
        get_best_move_for_score_fn(game_state, get_score_for_board)
    }
}
