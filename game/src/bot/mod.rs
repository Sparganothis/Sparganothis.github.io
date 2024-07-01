use random_choice_bot::RandomChoiceBot;
use wordpress_blog_bot::WordpressBlogBot;

use crate::tet::{GameState, TetAction};

pub mod random_choice_bot;
pub mod wordpress_blog_bot;

pub trait TetBot {
    fn choose_move(&self, game_state: &GameState) -> anyhow::Result<Vec<TetAction>>;
}

pub fn get_bot(bot_name: &str) -> anyhow::Result<Box<dyn TetBot>> {
    Ok(match bot_name {
        "random" => Box::new(RandomChoiceBot),
        "wordpress" => Box::new(WordpressBlogBot),
        _ => anyhow::bail!("bot name not found."),
    })
}

pub fn get_bot_id(bot_name: &str) -> anyhow::Result<uuid::Uuid> {
    Ok(match bot_name {
        "random" => uuid::Uuid::from_u128(0),
        "wordpress" => uuid::Uuid::from_u128(1),
        _ => anyhow::bail!("bot name not found."),
    })
}

pub fn get_bot_from_id(bot_id: uuid::Uuid) -> anyhow::Result<String> {
    Ok(if bot_id == uuid::Uuid::from_u128(0) {
        "random"
    } else if bot_id == uuid::Uuid::from_u128(1) {
        "wordpress"
    } else {
        anyhow::bail!("bot id not found.")
    }
    .to_string())
}
