use game::tet::GameState;
use leptos::*;

use crate::comp::game_board::GameBoard;

#[component]
pub fn MsPaintGameBoard()-> impl IntoView {
    let game_state = create_rw_signal(GameState::empty());
    let on_reset_game = Callback::<()>::new(move |_| {
    });
    
    view! {
        <h1>penis </h1>
        <GameBoard game_state on_reset_game/>
    }
}