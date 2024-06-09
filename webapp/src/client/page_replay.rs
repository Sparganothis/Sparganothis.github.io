use super::game_board::OpponentGameBoard;
use leptos::*;
#[component]
pub fn GameReplay() -> impl IntoView {
    view! {
        <div class="main_left">
            <OpponentGameBoard/>
        </div>
        <div class="main_right">
            <OpponentGameBoard/>
        </div>
    }
}
