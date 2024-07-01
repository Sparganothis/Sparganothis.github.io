use crate::tet::TetAction;

use super::TetBot;

pub struct RandomChoiceBot;

impl TetBot for RandomChoiceBot {
    fn choose_move(
        &self,
        _: &crate::tet::GameState,
    ) -> anyhow::Result<crate::tet::TetAction> {
        Ok(TetAction::random())
    }
}
