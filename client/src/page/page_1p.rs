use crate::comp::game_board_player::PlayerGameBoard;
use leptos::*;
#[component]
pub fn Game1PPage() -> impl IntoView {
    view! {
        <div class="main_left">
            <PlayerGameBoard/>
        </div>
    }
}

