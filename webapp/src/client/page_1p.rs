use super::game_board::PlayerGameBoard;
use crate::game::tet::GameSeed;
use leptos::*;
#[component]
pub fn Game1PPage() -> impl IntoView {
    let seed: GameSeed = 0;
    view! {
        <div class="main_left">
            <PlayerGameBoard seed=seed/>
        </div>
    }
}
