use game::{
    api::{
        game_replay::GameId,
        websocket::{GameSegmentCountReply, GetAllGames},
    },
    random::GameSeed,
    timestamp::get_human_readable_nano,
};

use crate::{comp::table_generic::DisplayTableGeneric, websocket::demo_comp::call_api_sync};
use game::api::table_paginate::TablePaginateDirection;

use game::api::websocket::GetAllGamesArg;
use leptos::*;
use leptos_struct_table::*;


#[component]
pub fn AllGamesTable(list_type: GetAllGamesArg) -> impl IntoView {

    let fi = Callback::new(move |(k, cb): (TablePaginateDirection<_>, Callback<_>)| {

        call_api_sync::<GetAllGames>((list_type, k), move |x| {
            cb.call(x);
        });
    });

    type DataP = Vec<FullGameReplayTableRow>;
    log::warn!("hello sirs");

    view! {
        <
            DisplayTableGeneric<
                GameSegmentCountReply,
                FullGameReplayTableRow,
                GameId,
                DataP,
            > 
        
            fetch_items=fi 
        />
    }.into_view()

}

use leptos_struct_table::BootstrapClassesPreset;

use super::table_generic::CustomRowExtraView;

#[derive(TableRow, Clone, Debug)]
#[table( 
    classes_provider = "BootstrapClassesPreset", impl_vec_data_provider)]
pub struct FullGameReplayTableRow {
    #[table(renderer = "WeedRenderer")]
    pub user_id: uuid::Uuid,
    #[table(renderer = "SeedRenderer")]
    pub init_seed: GameSeed,
    #[table(renderer = "TimeRenderer")]
    pub start_time: i64,
    pub num_segments: usize,
    pub is_in_progress: bool,
}

impl CustomRowExtraView for FullGameReplayTableRow {
    fn row_extra_view(&self) -> impl IntoView  {
    
    let is_in_prog =  self.is_in_progress;
    let url = self.to_url();

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
       }
    }
}

impl From<(GameId, GameSegmentCountReply)> for FullGameReplayTableRow {
    fn from(db_row: (GameId, GameSegmentCountReply)) -> Self {
        Self {
            user_id: db_row.0.user_id,
            init_seed: db_row.0.init_seed,
            start_time: db_row.0.start_time,
            num_segments: db_row.1.segment_count as usize,
            is_in_progress: db_row.1.is_in_progress,
        }
    }
}


impl FullGameReplayTableRow {
    pub fn to_url(&self) -> String {
        GameId {
            user_id: self.user_id,
            init_seed: self.init_seed,
            start_time: self.start_time,
        }
        .to_url()
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
fn WeedRenderer<F>(
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
