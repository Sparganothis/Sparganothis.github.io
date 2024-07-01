/// https://codemyroad.wordpress.com/2013/04/14/tetris-ai-the-near-perfect-player/
/// https://leeyiyuan.github.io/tetrisai/
/// Zis Bot uses 4 nummbers to evaluate game states.
use crate::tet::TetAction;

use super::TetBot;

pub struct WordpressBlogBot;

impl TetBot for WordpressBlogBot {
    fn choose_move(
        &self,
        _: &crate::tet::GameState,
    ) -> anyhow::Result<crate::tet::TetAction> {
        Ok(TetAction::random())
    }
}
