use super::game_board::PlayerGameBoard;
use crate::game::random::GameSeed;
use leptos::*;
#[component]
pub fn Game1PPage() -> impl IntoView {
    let seed: GameSeed = [0;32];
    view! {
        <div class="main_left">
            <PlayerGameBoard seed=seed/>
        </div>
    }
}
