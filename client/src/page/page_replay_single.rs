use game::{
    api::{
        game_replay::GameId,
        websocket::{GameSegmentCountReply, GetAllGames},
    },
    random::GameSeed,
    timestamp::get_human_readable_nano,
};
use leptos_router::use_params_map;

use crate::{
    comp::game_board_replay::ReplayGameBoard,
    websocket::demo_comp::{call_websocket_api, WebsocketAPI},
};
use game::api::websocket::GetAllGamesArg;
use icondata as i;
use leptonic::prelude::*;
use leptos::*;
use leptos_struct_table::*;

#[component]
pub fn GameReplaySinglePage() -> impl IntoView {
    let params = use_params_map();

    // id: || -> Option<String>
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
