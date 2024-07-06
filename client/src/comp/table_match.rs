use game::{
    api::{
        game_match::GameMatch, websocket::{GetMatchList, GetMatchListArg}
    },
    random::GameSeed,
    timestamp::get_human_readable_nano,
};
use uuid::Uuid;

use crate::{comp::table_generic::DisplayTableGeneric, websocket::demo_comp::call_api_sync};
use game::api::table_paginate::TablePaginateDirection;
use leptos::*;
use leptos_struct_table::*;

#[component]
pub fn AllMatchTable(list_type: GetMatchListArg) -> impl IntoView {


    let fi = Callback::new(move |(k, cb): (TablePaginateDirection<_>, Callback<_>)| {
        
        call_api_sync::<GetMatchList>((list_type, k), move |x| {
            cb.call(x);
        });
    });

    type DataP = Vec<GameMatchTableRow>;
    log::warn!("hello sirs");
    use uuid::Uuid;
    view! {
        <
            DisplayTableGeneric<
                GameMatch,
                GameMatchTableRow,
                Uuid,
                DataP,
            > 
        
            fetch_items=fi 
        />
    }.into_view()

}

impl CustomRowExtraView for GameMatchTableRow {
}

use leptos_struct_table::BootstrapClassesPreset;

use super::table_generic::CustomRowExtraView;

#[derive(TableRow, Clone, Debug)]
#[table( 
    classes_provider = "BootstrapClassesPreset", impl_vec_data_provider)]
pub struct GameMatchTableRow {
    #[table(renderer = "MatchLinkRenderer")]
    pub match_id: uuid::Uuid,
    #[table(renderer = "SeedRenderer")]
    pub init_seed: GameSeed,
    #[table(renderer = "TimeRenderer")]
    pub start_time: i64,

    #[table(renderer = "UserLinkRenderer")]
    pub user0: uuid::Uuid,
    #[table(renderer = "UserLinkRenderer")]
    pub user1: uuid::Uuid,
    // pub title: String,
}


impl From<(Uuid, GameMatch)> for GameMatchTableRow {
    fn from(db_row: (Uuid, GameMatch)) -> Self {
        Self {
            match_id: db_row.0,
            init_seed: db_row.1.seed,
            start_time: db_row.1.time,
            user0: db_row.1.users[0],
            user1:  db_row.1.users[1]
            // title : db_row.1.title,
        }
    }
}


#[allow(unused_variables)]
#[component]
fn TimeRenderer<F>(
    class: String,
    #[prop(into)] value: MaybeSignal<i64>,
    on_change: F,
    index: usize,
) -> impl IntoView
where
    F: Fn(i64) + 'static,
{
    view! {
        <td class=class>
            <p>{move || { get_human_readable_nano(value.get()) }}</p>
        </td>
    }
}

#[allow(unused_variables)]
#[component]
fn UserLinkRenderer<F>(
    class: String,
    #[prop(into)] value: MaybeSignal<uuid::Uuid>,
    on_change: F,
    index: usize,
) -> impl IntoView
where
    F: Fn(uuid::Uuid) + 'static,
{
    view! {
        <td class=class>
            <a href=format!("/user/{:?}", value.get())>
                <p style="border: 1px solid black">
                    {move || { format!("{:?}", value.get())[0..8].to_string() }}
                </p>
            </a>
        </td>
    }
}

#[allow(unused_variables)]
#[component]
fn MatchLinkRenderer<F>(
    class: String,
    #[prop(into)] value: MaybeSignal<uuid::Uuid>,
    on_change: F,
    index: usize,
) -> impl IntoView
where
    F: Fn(uuid::Uuid) + 'static,
{
    view! {
        <td class=class>
            <a href=format!("/match/{:?}", value.get())>
                <p style="border: 1px solid black">
                    {move || { format!("{:?}", value.get())[0..8].to_string() }}
                </p>
            </a>
        </td>
    }
}

#[allow(unused_variables)]
#[component]
fn SeedRenderer<F>(
    class: String,
    #[prop(into)] value: MaybeSignal<GameSeed>,
    on_change: F,
    index: usize,
) -> impl IntoView
where
    F: Fn(GameSeed) + 'static,
{
    view! {
        <td class=class>
            <p>{move || format!("{:?}, ..", value.get()[0])}</p>
        </td>
    }
}
