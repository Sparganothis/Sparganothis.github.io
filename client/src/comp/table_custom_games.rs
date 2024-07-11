use leptos::*;


use game::{
    api::websocket::GetAllCustomGames, tet::GameState
};

use game::api::table_paginate::TablePaginateDirection;
use crate::websocket::demo_comp::call_api_sync;

use crate::comp::table_generic::DisplayTableGeneric;

#[component]
pub fn ListAllCustomGames() -> impl IntoView {
    let fi = Callback::new(move |(k, cb): (TablePaginateDirection<_>, Callback<_>)| {

        call_api_sync::<GetAllCustomGames>(k, move |x| {
            cb.call(x);
        });
    });

    let column_display_fns: Vec<(String, Callback<(String, GameState), View>)> = vec![
        (
            "Save Name".to_string(),
            Callback::new(|(_k, _it):(String, GameState)| {
                _k.into_view()
            })
        ),

        (
            "Start Time".to_string(),
            Callback::new(|(_k, _it):(String, GameState)| {
                _it.start_time.to_string().into_view()
            })
        ),

        (
            "Edit".to_string(),
            Callback::new(|(_k, _it):(String, GameState)| {
                let url = format!("/edit-custom-game/{}", _k);
                view!{
                    <a href={url}>EDIT</a>
                }.into_view()
            })
        ),

        (
            "Play".to_string(),
            Callback::new(|(_k, _it):(String, GameState)| {
                let url = format!("/play-custom-game/{}", _k);
                view!{
                    <a href={url}>PLAY</a>
                }.into_view()
            })
        ),
    ];

    view! {
        <
        DisplayTableGeneric<
                GameState,
                String,
            >
            fetch_items=fi
            column_display_fns
        />
    }.into_view()
}