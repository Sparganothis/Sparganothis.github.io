use crate::game::tet::GameSeed;

use super::game_board::{OpponentGameBoard, PlayerGameBoard};
use leptos::*;
#[component]
pub fn GameCPUPage() -> impl IntoView {
    let seed: GameSeed = 0;

    view! {
        <div class="main_left">
            <PlayerGameBoard seed=seed/>
        </div>
        <div class="main_right">
            <OpponentGameBoard seed=seed/>
        </div>
    }
}
