use crate::game::random::GameSeed;

use super::game_board::OpponentGameBoard;
use leptos::*;

#[component]
pub fn GameReplayPage() -> impl IntoView {
    let seed: GameSeed = [0;32];
    view! {
        <div class="main_left">
            <OpponentGameBoard seed=seed/>
        </div>
        <div class="main_right">
            <OpponentGameBoard seed=seed/>
        </div>
    }
}
