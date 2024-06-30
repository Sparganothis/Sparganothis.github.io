
use game::api::websocket::{GetAllGames, GetAllGamesArg};
// use game::tet::GameState;
use leptos::*;


use crate::{comp::{game_board_flex::GameBoardFlexPropsBuilder_Error_Repeated_field_game_state, menu_grid_view::MenuGridView}, mobile_check::is_mobile_phone, page::page_replay_single::GameReplayBoardMini, websocket::demo_comp::call_api_sync};

#[component]
pub fn Homepage()-> impl IntoView{
    let is_mobile = is_mobile_phone();
    log::info!("Are yi a mobile phone???? {is_mobile}");

    let best_gameid = create_rw_signal(None);
    call_api_sync::<GetAllGames>(GetAllGamesArg::BestGames, move |v: Vec<_>| {
            let game_id = v.get(0).clone();
            if let Some((a, _b)) = game_id {
                best_gameid.set(Some(*a));
            }
    });

    let new_url =format!("/mspaint");
    let views:Vec<_> = {0..20}.into_iter().map(|x|{
        match x{
            0 => {
                view! { <h1>todo</h1> }.into_view()
            },
            1=>{
                view! { <h1>{is_mobile_phone}</h1> }.into_view()
            },
            8 =>{
                (move || {
                    match best_gameid.get() {
                        Some(game_id) => view! { <GameReplayBoardMini game_id/> }.into_view(),
                        None => view!{}.into_view(),
                    }
                }).into_view()
            },

            _ => {
                view!{

                }.into_view()
            }
        }


     }).collect();

    view! { <MenuGridView views/> }
}
