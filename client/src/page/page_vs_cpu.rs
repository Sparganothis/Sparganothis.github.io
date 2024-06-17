use game::random::GameSeed;

use crate::comp::game_board::{PlayerGameBoard, RandomOpponentGameBoard};
use leptos::*;
#[component]
pub fn GameCPUPage() -> impl IntoView {
    let seed: GameSeed = [0; 32];

    view! {
        <div class="main_left">
            <PlayerGameBoard/>
        </div>
        <div class="main_right">
            <RandomOpponentGameBoard seed=seed/>
        </div>
    }
}