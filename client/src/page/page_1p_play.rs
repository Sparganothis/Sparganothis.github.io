use game::api::game_replay::GameId;
use leptos::*;
use leptos_router::{use_navigate, use_params_map, NavigateOptions};
use crate::comp::game_board_player::PlayerGameBoardFromId;

#[component]
pub fn Game1PPage() -> impl IntoView {
    let game_id = create_rw_signal(None);
    let params = use_params_map();
    create_effect(
        move |_| {
            if let Some(str) = params.with(|params| params.get("game_id").cloned()) {
                if let Ok(val) = GameId::from_url(str) {
                    game_id.set(Some(val));
                }
            }
        }
    );
    let on_reset = Callback::new(move |_|{
        let navigate = use_navigate();             
        navigate("/solo", NavigateOptions::default());
    });

    view! {
        <div class="main_left">
            {move || {
                match game_id.get() {
                    Some(id) => {
                        view! { <PlayerGameBoardFromId game_id=id on_reset/> }
                            .into_view()
                    }
                    None => view! { <h1>"bad url"</h1> }.into_view(),
                }
            }}

        </div>
    }
}
