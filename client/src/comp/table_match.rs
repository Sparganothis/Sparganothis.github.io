use game::{
    api::{
        game_match::GameMatch, websocket::{GetMatchList, GetMatchListArg}
    },
    timestamp::get_human_readable_nano,
};
use uuid::Uuid;

use crate::websocket::demo_comp::call_api_sync;
use game::api::table_paginate::TablePaginateDirection;
use leptos::*;
use crate::comp::table_generic::DisplayTableGeneric;

#[component]
pub fn AllMatchTable(list_type: GetMatchListArg) -> impl IntoView {

    let fi = Callback::new(move |(k, cb): (TablePaginateDirection<_>, Callback<_>)| {
        
        call_api_sync::<GetMatchList>((list_type, k), move |x| {
            cb.call(x);
        });
    });

    let column_display_fns: Vec<(String, Callback<(Uuid, GameMatch), View>)> = vec![
        (
            "Open Match".to_string(),
            Callback::new(|(_k, _it):(Uuid, GameMatch)| {
                view!{
                    <a  style="border: 1px solid black" href=format!("/match/{:?}", _k)>
                        {format!("{:?}", _k)[0..8].to_string() }
                    </a>
                }.into_view()
            })
        ),
        
        (
            "Seed".to_string(),
            Callback::new(|(_k, _it):(Uuid, GameMatch)| {
                format!("{:?}, ..", _it.seed[0]).into_view()
            })
        ),

        (
            "Start Time".to_string(),
            Callback::new(|(_k, _it):(Uuid, GameMatch)| {
                get_human_readable_nano(_it.time).into_view()
            })
        ),

        (
            "User 0".to_string(),
            Callback::new(|(_k, _it):(Uuid, GameMatch)| {
                let uid = _it.users[0];
                view!{
                    <a href=format!("/user/{:?}", uid)>
                        <p style="border: 1px solid black">
                            {format!("{:?}",  uid)[0..8].to_string() }
                        </p>
                    </a>
                }.into_view()
            })
        ),

        (
            "User 1".to_string(),
            Callback::new(|(_k, _it):(Uuid, GameMatch)| {
                let uid = _it.users[1];
                view!{
                    <a href=format!("/user/{:?}", uid)>
                        <p style="border: 1px solid black">
                            {format!("{:?}",  uid)[0..8].to_string() }
                        </p>
                    </a>
                }.into_view()
            })
        ),
    ];

    view! {
        <
            DisplayTableGeneric<
                GameMatch,
                Uuid,
            >
            fetch_items=fi
            column_display_fns
        />
    }.into_view()

}
