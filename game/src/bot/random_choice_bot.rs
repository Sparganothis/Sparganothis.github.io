use crate::tet::{GameState, TetAction};

use super::TetBot;

pub struct RandomChoiceBot;

pub fn get_all_move_chains() -> Vec<Vec<TetAction>> {
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
) -> anyhow::Result<f64>
where
    F: Fn(&GameState, &GameState) -> anyhow::Result<f64>,
{
    let old_state = game_state;
    let mut state = game_state.clone();
    for action in action_chain {
        let _ = state.apply_action_if_works(*action, 0)?;
    }

    if state.game_over {
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
    let mut all_action_chains = get_all_move_chains();

    use rand::seq::SliceRandom;
    use rand::thread_rng;
    let mut rng = thread_rng();
    all_action_chains.shuffle(&mut rng);
    all_action_chains.sort_by_key(|k| k.len());

    let mut best_action_chain = vec![TetAction::SoftDrop];
    let mut best_acction_score = f64::MIN;
    for act in all_action_chains {
        if let Ok(sccore) = get_action_chain_score(game_state, &act, &f) {
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
