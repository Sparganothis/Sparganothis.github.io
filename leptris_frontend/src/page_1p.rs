use crate::{game_board::PlayerGameBoard, tet::GameState};
use leptos::*;
#[component]
pub fn Game1P() -> impl IntoView {
    view! {
        <div class="main_left">
            <PlayerGameBoard/>
        </div>
    }
}
