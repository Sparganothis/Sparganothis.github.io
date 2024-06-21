use leptos::*;


use game::{
    api::{
        game_replay::GameId,
        websocket::{GameSegmentCountReply, GetAllCustomGames, GetAllGames},
    }, random::GameSeed, tet::GameState, timestamp::get_human_readable_nano
};

use crate::websocket::demo_comp::{call_websocket_api, WebsocketAPI};
use game::api::websocket::GetAllGamesArg;
use leptos::*;
use leptos_struct_table::*;

#[component]
pub fn ListAllCustomGames() -> impl IntoView {
    let api2: WebsocketAPI = expect_context();
    let all_games = create_resource(
        || (),
        move |_| {
            let api2 = api2.clone();
            async move {
                // log::info!("calling websocket api");
                let r = call_websocket_api::<GetAllCustomGames>(api2, ())
                    .expect("cannot obtain future")
                    .await;
                // log::info!("got back response: {:?}", r);
                r
            }
        },
    );

    let table_from_rows = move || {
        if let Some(Ok(rows)) = all_games.get() {
            let rows = rows
                .iter()
                .map(|r| CustomGameDbRow::new(r.clone()))
                // .filter(|f| f.num_segments > 0)
                .collect::<Vec<_>>();

            view! {
                <table id="get_custom_games">
                    <TableContent rows />
                </table>
            }
            .into_view()
        } else {
            view! { <p>loading...</p> }.into_view()
        }
    };

    view! { {table_from_rows} }
}


#[derive(TableRow, Clone, Debug)]
#[table(impl_vec_data_provider)]
pub struct CustomGameDbRow {
    pub save_name: String,
    #[table(skip)]
    pub game_state: GameState,
    pub start_time: i64,
}

impl CustomGameDbRow {
    pub fn new(db_row: (String, GameState)) -> Self {
        Self {
            save_name: db_row.0,
            game_state: db_row.1.clone(),
            start_time: db_row.1.start_time,
        }
    }

}

