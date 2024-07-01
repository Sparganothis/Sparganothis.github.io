use crate::tet::TetAction;

use super::TetBot;

pub struct RandomChoiceBot;

impl TetBot for RandomChoiceBot {
    fn choose_move(
        &self,
        _: &crate::tet::GameState,
    ) -> anyhow::Result<Vec<crate::tet::TetAction>> {
        Ok(vec![TetAction::random()])
    }
}
