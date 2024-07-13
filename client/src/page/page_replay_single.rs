use game::{
    api::{game_replay::GameId, websocket::GetAllSegments},
    tet::GameState,
};
use leptos_router::use_params_map;
use crate::comp::game_board_flex::FlexText;
use crate::{
    comp::{
        game_board_replay::ReplayGameBoardFromSegmments, table_replay_segments::TableReplaySegments,
    },
    websocket::demo_comp::call_api_sync,
};
use leptos::*;

#[component]
pub fn GameReplaySinglePage() -> impl IntoView {
    let params = use_params_map();
    let all_segments = create_rw_signal(vec![]);

    let game_id = move || -> Result<GameId, String> {
        let p = params.with(|params| params.get("game_id").cloned());
        let p = p.ok_or("param missing".to_string())?;
        let p = GameId::from_url(p).map_err(|_e| "url corrupted".to_string())?;

        call_api_sync::<GetAllSegments>(p , move |r| {
            all_segments.set(r);
        });

        Ok(p)
    };

    let all_segments = move || {
        all_segments.get()
    };
    let slider = create_rw_signal(0.0);

    let game_state = create_rw_signal(GameState::new(&[0; 32], 0));

    view! {
        <div class="main_left">
            {move || {
                match game_id() {
                    Ok(_game_id) => {
                        view! {
                            <ReplayGameBoardFromSegmments
                                all_segments=all_segments.into_signal()
                                slider
                                game_state
                                player_id=_game_id.user_id
                                top_bar_override=None
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


#[component]
pub fn GameReplayBoardMini(    game_id: GameId) -> impl IntoView{ 
    let all_segments = create_rw_signal(vec![]);
    call_api_sync::<GetAllSegments>(game_id , move |r| {
         all_segments.set(r);
    });
    let all_segments = move || {
        all_segments.get()
    };

    let slider = create_rw_signal(0.0);
    let game_state = create_rw_signal(GameState::new(&[0; 32], 0));

    view! {
        <ReplayGameBoardFromSegmments
            all_segments=all_segments.into_signal()
            slider
            game_state
            hide_controller=true
            player_id=game_id.user_id
            top_bar_override=Some(view! { <FlexText size_cqh=30.0 text="mini replay"/> }.into_view())
        />
    }
}