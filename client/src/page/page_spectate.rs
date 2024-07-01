use game::api::game_replay::GameId;
use leptos::*;
use leptos_router::use_params_map;

use crate::comp::game_board_spectator::SpectatorGameBoard;

#[component]
pub fn SpectateGamePage() -> impl IntoView {
    
    let params = use_params_map();
    let left_game_id = create_rw_signal::<Option<GameId>>(None);
    
    create_effect(move |_| {
        let p = params.with(|params| params.get("game_id").cloned());
        if let Some(p) = p {
            if let Ok(p) = GameId::from_url(p) {
               left_game_id.set(Some(p.clone()));
            }
        }
    });

    let left_board = move || {
        if let Some(x) = left_game_id.get() {
            view! { <SpectatorGameBoard game_id=x/> }.into_view()
        } else {
            view! { <p>"bad url?"</p> }.into_view()
        }
    };

    view! {
        <div class="main_left">{left_board}</div>
    }
}