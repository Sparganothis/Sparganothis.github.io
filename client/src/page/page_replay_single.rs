use game::{
    api::{game_replay::GameId, websocket::GetAllSegments},
    tet::GameState,
};
use leptos_router::use_params_map;

use crate::{
    comp::{
        game_board_replay::ReplayGameBoard, table_replay_segments::TableReplaySegments,
    },
    websocket::demo_comp::{call_websocket_api, WebsocketAPI},
};
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

    let api: WebsocketAPI = expect_context();
    let all_segments = create_resource(
        move || game_id(),
        move |game_id| {
            let api3 = api.clone();
            async move {
                if game_id.is_err() {
                    return vec![];
                }
                let game_id = game_id.unwrap();
                // log::info!("calling websocket api");
                let r = call_websocket_api::<GetAllSegments>(api3, game_id)
                    .expect("cannot obtain future")
                    .await;
                // log::info!("got back response: {:?}", r);
                if let Ok(all_segments) = r {
                    all_segments
                } else {
                    vec![]
                }
            }
        },
    );

    let all_segments = move || {
        if let Some(s) = all_segments.get() {
            s
        } else {
            vec![]
        }
    };
    let slider = create_rw_signal(0.0);

    let game_state = create_rw_signal(GameState::new(&[0; 32], 0));

    view! {
        <div class="main_left">
            {move || {
                match game_id() {
                    Ok(_game_id) => {
                        view! {
                            <ReplayGameBoard
                                all_segments=all_segments.into_signal()
                                slider
                                game_state
                            />
                        }
                            .into_view()
                    }
                    Err(err) => view! { <p>{err} ...</p> }.into_view(),
                }
            }}

        </div>
        <div class="main_right">
            <TableReplaySegments
                all_segments=all_segments.into_signal()
                slider
                game_state=game_state.read_only()
            />
        </div>
    }
}
