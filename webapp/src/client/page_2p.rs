use super::game_board::PlayerGameBoard;
use crate::game::tet::GameSeed;
use leptos::*;
#[component]
pub fn Game2PPage() -> impl IntoView {
    let seed: GameSeed = 0;
    view! {
        <div class="main_left">
            <PlayerGameBoard seed=seed/>
        </div>
        <div class="main_right">
            <PlayerGameBoard seed=seed/>
        </div>
    }
}
