use leptos::*;
use crate::{game_board::GameBoard, tet::GameState};
#[component]
pub fn Game1P()-> impl IntoView {
    let (state,set_state)=create_signal(GameState::empty());
    view!{
        
        <div class="main_left">
        <GameBoard game_state=state/>
    </div>
    }
}