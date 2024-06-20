use game::api::game_replay::GameId;
use leptos_router::use_params_map;

use crate::comp::game_board_replay::ReplayGameBoard;
use leptos::*;

#[component]
pub fn GameReplaySinglePage() -> impl IntoView {
    let params = use_params_map();

    let game_id = move || -> Result<GameId, String> {
        let p = params.with(|params| params.get("game_id").cloned());
        let p = p.ok_or("param missing".to_string())?;
        let p = GameId::from_url(p).map_err(|_e| "url corrupted".to_string())?;
        Ok(p)
    };

    view! {
        <div class="main_left">
            {move || {
                match game_id() {
                    Ok(game_id) => {
                        view! { <ReplayGameBoard game_id=game_id/> }.into_view()
                    }
                    Err(err) => view! { <p>{err} ...</p> }.into_view(),
                }
            }}

        </div>
    }
}
