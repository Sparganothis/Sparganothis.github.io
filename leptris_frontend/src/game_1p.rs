use crate::{game_board::GameBoard, tet::GameState};
use leptos::*;
#[component]
pub fn Game1P() -> impl IntoView {
    let (get_state, _set_state) = create_signal(GameState::empty());
    view! {
        <div class="main_left">
            <crate::hotkey_reader::HotkeyReader on_action=move |_action| _set_state.update( |state| {
                let r = state.try_action(_action);
                if let Ok(new_state) = r {
                    *state =new_state;
                } else {
                    log::warn!("user action {:?} failed: {:?}", _action, r.unwrap_err());
                }
            } ) />
            <GameBoard game_state=get_state/>
        </div>
    }
}
