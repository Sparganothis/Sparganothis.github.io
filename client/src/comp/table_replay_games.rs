use game::{
    api::{
        game_replay::GameId,
        websocket::{GameSegmentCountReply, GetAllGames},
    },
    timestamp::get_human_readable_nano,
};

use crate::websocket::demo_comp::call_api_sync;
use game::api::table_paginate::TablePaginateDirection;

use game::api::websocket::GetAllGamesArg;
use leptos::*;
use crate::comp::table_generic::DisplayTableGeneric;

#[component]
pub fn AllGamesTable(list_type: GetAllGamesArg) -> impl IntoView {

    let fi = Callback::new(move |(k, cb): (TablePaginateDirection<_>, Callback<_>)| {

        call_api_sync::<GetAllGames>((list_type, k), move |x| {
            cb.call(x);
        });
    });

    let column_display_fns: Vec<(String, Callback<(GameId, GameSegmentCountReply), View>)> = vec![
        (
            "User Id".to_string(),
            Callback::new(|(_k, _it):(GameId, _)| {
                view!{
                    <a style="border: 1px solid black" href=format!("/user/{:?}", _k.user_id)>
                            {format!("{:?}",  _k.user_id)[0..8].to_string() }
                    </a>
                }.into_view()
            })
        ),

        (
            "Seed".to_string(),
            Callback::new(|(_k, _it):(GameId, _)| {
                format!("{:?}, ..", _k.init_seed[0]).into_view()
            })
        ),

        (
            "Start Time".to_string(),
            Callback::new(|(_k, _it):(GameId, _)| {
                get_human_readable_nano(_k.start_time).into_view()
            })
        ),
        (
            "Num Segments".to_string(),
            Callback::new(|(_k, _it):(GameId, GameSegmentCountReply)| {
                format!("{:?}", _it.segment_count).into_view()
            })
        ),
        
        (
            "In Progress".to_string(),
            Callback::new(|(_k, _it):(GameId, GameSegmentCountReply)| {
                format!("{:?}", _it.is_in_progress).into_view()
            })
        ),
               (
            "Open".to_string(),
            Callback::new(|(_k, _it):(GameId, GameSegmentCountReply)| {

    
                let is_in_prog =  _it.is_in_progress;
                let url = _k.to_url();
            
                let url2 = url.clone();
                   view! {
                       <a href=move || {
                           if is_in_prog {
                               format!("/spectate-game/{}", url)
                           } else {
                               format!("/view-game/{}", url2)
                           }
                       }>
                           {move || {
                               if is_in_prog { "Spectate".to_string() } else { "Replay".to_string() }
                           }}
            
                       </a>
                   }.into_view()

            })
        ),
    ];
    view! {
        <
            DisplayTableGeneric<
                GameSegmentCountReply,
                GameId,
            >
            fetch_items=fi
            column_display_fns
        />
    }.into_view()

}
