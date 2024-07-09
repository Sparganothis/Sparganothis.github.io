/// https://codemyroad.wordpress.com/2013/04/14/tetris-ai-the-near-perfect-player/
/// https://leeyiyuan.github.io/tetrisai/
/// Zis Bot uses 4 nummbers to evaluate game states.
use crate::tet::GameState;

use super::TetBot;

pub struct WordpressBlogBot;

use super::random_choice_bot::get_best_move_for_score_fn;

fn get_wordpress_score_for_board(
    old_state: &GameState,
    new_state: &GameState,
) -> anyhow::Result<f64> {
    let line_diff = new_state.total_lines - old_state.total_lines;

    let bumpi: i32 = new_state.main_board.board_bumpi();
    let holes: i32 = new_state.main_board.board_holes();

    Ok(
        -0.51 * (new_state.main_board.get_height() as f64) + 0.76 * (line_diff as f64)
            - 0.35 * (holes as f64)
            - 0.18 * (bumpi as f64),
    )
}

impl TetBot for WordpressBlogBot {
    fn choose_move(
        &self,
        game_state: &crate::tet::GameState,
    ) -> anyhow::Result<Vec<crate::tet::TetAction>> {
        get_best_move_for_score_fn(game_state, get_wordpress_score_for_board)
    }
}
