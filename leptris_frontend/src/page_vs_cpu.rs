use crate::game_board::{OpponentGameBoard, PlayerGameBoard};
use leptos::*;
#[component]
pub fn GameCPU() -> impl IntoView {
    view! {

    <div class="main_left">
        <PlayerGameBoard/>
    </div>
    <div class="main_right">
        <OpponentGameBoard/>
    </div>
    }
}
