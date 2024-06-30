use crate::{comp::{game_board::RandomOpponentGameBoard, menu_grid_view::MenuGridView}, websocket::demo_comp::call_api_sync};
use game::{api::{game_replay::GameId, websocket::CreateNewGameId}, random::GameSeed};
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


#[component]
pub fn GameSoloLobbyPage() -> impl IntoView {
    let seed: GameSeed = [0; 32];
    let redirect_to_new_game = Callback::new(move |_|{
        let navigate = use_navigate();
         call_api_sync::<CreateNewGameId>((), move |r:GameId| {
            let new_url = format!("/play-game-solo/{}", r.to_url());
            navigate(&new_url, NavigateOptions::default());
         });        
    });
    
    let play_button = view! {
        <div style="width:100%;height:100%; container-type: size;">
            <h3
                style="font-size:80cqh; text-align: center;"
                on:click=move |_| { redirect_to_new_game.call(()) }
            >
                PLAY
            </h3>
        </div>
    }
        .into_view();


    let views:Vec<_> = {0..20}.into_iter().map(move |x|{
        match x{
            8 =>view! { <RandomOpponentGameBoard seed=seed/> }
            .into_view(),
            6 => play_button.clone(),
            _ => view!{            }.into_view()
            
        }
     }).collect();

    view! { <MenuGridView views/> }
}


