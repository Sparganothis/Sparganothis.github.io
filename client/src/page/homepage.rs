
use game::api::{game_replay::GameId, table_paginate::TablePaginateDirection, websocket::{GetAllGames, GetAllGamesArg}};
// use game::tet::GameState;
use leptos::*;


use crate::{
    comp::{game_board_flex::FlexText, menu_grid_view::MenuGridView}, 
    // page::page_replay_single::GameReplayBoardMini, 
    websocket::demo_comp::call_api_sync
};

#[component]
pub fn Homepage()-> impl IntoView{
    let best_gameid = create_rw_signal(None);
    call_api_sync::<GetAllGames>((GetAllGamesArg::BestGames, TablePaginateDirection::<GameId>::InitialPage
    ), move |v: Vec<_>| {
            let game_id = v.get(0).clone();
            if let Some((a, _b)) = game_id {
                best_gameid.set(Some(*a));
            }
    });

    let views:Vec<_> = {0..20}.into_iter().map(|x|{
        match x{
            0 => {
                view! { <FlexText text="todo"/> }.into_view()
            },
            // 8 =>{
            //     (move || {
            //         match best_gameid.get() {
            //             Some(game_id) => view! { <GameReplayBoardMini game_id/> }.into_view(),
            //             None => view!{}.into_view(),
            //         }
            //     }).into_view()
            // },

            _ => {
                view!{

                }.into_view()
            }
        }


     }).collect();

    view! { <MenuGridView views/> }
}
