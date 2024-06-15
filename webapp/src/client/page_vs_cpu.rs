use crate::game::random::GameSeed;

use super::game_board::{PlayerGameBoard, RandomOpponentGameBoard};
use leptos::*;
#[component]
pub fn GameCPUPage() -> impl IntoView {
    let seed: GameSeed = [0; 32];

    view! {
        <div class="main_left">
            <PlayerGameBoard seed=seed/>
        </div>
        <div class="main_right">
            <RandomOpponentGameBoard seed=seed/>
        </div>
    }
}
