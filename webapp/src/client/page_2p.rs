use super::game_board::PlayerGameBoard;
use crate::game::random::GameSeed;
use leptos::*;
#[component]
pub fn Game2PPage() -> impl IntoView {
    let seed: GameSeed = [0; 32];
    view! {
        <div class="main_left">
            <PlayerGameBoard/>
        </div>
        <div class="main_right">
            <PlayerGameBoard />
        </div>
    }
}
