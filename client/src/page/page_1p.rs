use crate::{comp::menu_grid_view::MenuGridView, websocket::demo_comp::call_api_sync};
use game::api::{game_replay::GameId, websocket::CreateNewGameId};
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

    view! {
        <div class="main_left">
            {move || {
                match game_id.get() {
                    Some(id) => view! { <PlayerGameBoardFromId game_id=id/> }.into_view(),
                    None => view! { <h1>"bad url"</h1> }.into_view(),
                }
            }}

        </div>
    }
}


#[component]
pub fn GameSoloLobbyPage() -> impl IntoView {
    
    let redirect_to_new_game = Callback::new(move |_|{
        let navigate = use_navigate();
         call_api_sync::<CreateNewGameId>((), move |r:GameId| {
            let new_url = format!("/play-game-solo/{}", r.to_url());
            navigate(&new_url, NavigateOptions::default());
         });        
    });
    
    let play_button = view! { <h1 on:click=move |_| { redirect_to_new_game.call(()) }>PLAY</h1> }.into_view();


    let views:Vec<_> = {0..20}.into_iter().map(move |x|{
        match x{
            0 => play_button.clone(),
            _ => view!{            }.into_view()
            
        }
     }).collect();

    view! { <MenuGridView views/> }
}


