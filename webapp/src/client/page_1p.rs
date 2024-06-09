use super::game_board::PlayerGameBoard;
use leptos::*;
#[component]
pub fn Game1P() -> impl IntoView {
    view! {
        <div class="main_left">
            <PlayerGameBoard/>
        </div>
    }
}
