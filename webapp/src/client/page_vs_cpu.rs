use super::game_board::{OpponentGameBoard, PlayerGameBoard};
use leptos::*;
#[component]
pub fn GameCPUPage() -> impl IntoView {
    view! {
        <div class="main_left">
            <PlayerGameBoard/>
        </div>
        <div class="main_right">
            <OpponentGameBoard/>
        </div>
    }
}
