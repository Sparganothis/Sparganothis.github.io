use leptos::*;
use crate::{game_board::GameBoard, tet::GameState};
#[component]
pub fn Game2P()-> impl IntoView {
    let (state,_set_state)=create_signal(GameState::empty());
    view!{
        
        <div class="main_left">
        <GameBoard game_state=state/>
    </div>
    <div class="main_right">
        <GameBoard game_state=state/>
    </div>
    }
}