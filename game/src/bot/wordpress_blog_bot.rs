/// https://codemyroad.wordpress.com/2013/04/14/tetris-ai-the-near-perfect-player/
/// https://leeyiyuan.github.io/tetrisai/
/// Zis Bot uses 4 nummbers to evaluate game states.
use crate::tet::{BoardMatrix, GameState};

use super::TetBot;

pub struct WordpressBlogBot;

use super::random_choice_bot::get_best_move_for_score_fn;

pub fn get_height_for_column(b: &BoardMatrix, col: i32) -> i32 {
    for x in (0..b.get_num_rows()).rev() {
        match b.v[x][col as usize] {
            crate::tet::CellValue::Piece(_) => return x as i32,
            crate::tet::CellValue::Garbage => return x as i32,
            crate::tet::CellValue::Empty => continue,
            crate::tet::CellValue::Ghost => continue,
        }
    }
    0
}

fn board_holes(b: &BoardMatrix) -> i32 {
    let mut holes: i32 = 0;

    for x in (0..b.get_num_cols()).rev() {
        let height = get_height_for_column(b, x as i32);

        for y in 0..height {
            match b.v[y as usize][x as usize] {
                crate::tet::CellValue::Empty | crate::tet::CellValue::Ghost => {
                    holes += 1;
                }
                _ => {}
            };
        }
    }

    holes
}
fn board_bumpi(b: &BoardMatrix) -> i32 {
    let mut max_bumpi = 0;
    for i in 0..(b.get_num_cols() - 1) {
        let left = i;
        let right = i + 1;
        let height_left = get_height_for_column(b, left as i32);
        let height_right = get_height_for_column(b, right as i32);

        let bumpi = height_left - height_right;
        let bumpi = if bumpi > 0 { bumpi } else { -bumpi };
        if bumpi > max_bumpi {
            max_bumpi = bumpi;
        }
    }
    max_bumpi
}

fn get_wordpress_score_for_board(
    old_state: &GameState,
    new_state: &GameState,
) -> anyhow::Result<f64> {
    let line_diff = new_state.total_lines - old_state.total_lines;

    let bumpi: i32 = board_bumpi(&new_state.main_board);
    let holes: i32 = board_holes(&new_state.main_board);

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
