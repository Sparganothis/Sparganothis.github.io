use super::game_board::PlayerGameBoard;
use leptos::*;

#[component]
pub fn Game2P() -> impl IntoView {
    view! {
        <div class="main_left">
            <PlayerGameBoard />
        </div>
        <div class="main_right">
            <PlayerGameBoard />
        </div>
    }
}
