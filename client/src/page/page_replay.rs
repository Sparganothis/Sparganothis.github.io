use game::{
    api::{ },
    random::GameSeed,
};

use crate::{
    comp::game_board::RandomOpponentGameBoard,
    websocket::demo_comp::{call_websocket_api, WebsocketAPI},
};
use icondata as i;
use leptonic::prelude::*;
use leptos::*;

use leptos_struct_table::*;

#[derive(TableRow, Clone)]
#[table(impl_vec_data_provider)]
pub struct Person {
    id: u32,
    name: String,
    age: u32,
}
#[component]
pub fn TableDemo() -> impl IntoView {
    let api2: WebsocketAPI = expect_context();
    let all_games = create_resource(
        || (),
        move |_| {
            let api2 = api2.clone();
            async move {
                // log::info!("calling websocket api");
                let r = call_websocket_api::<GetAllFullGameReplays>(api2, ())
                    .expect("cannot obtain future")
                    .await;
                // log::info!("got back response: {:?}", r);
                r
            }
        },
    );

    let trigger_rows = move || {
        if let (Some(Ok(rows))) = (all_games.get()) {
            let rows = rows
                .iter()
                .map(|r| FullGameReplayTableRow::new(r.clone()))
                // .filter(|f| f.num_segments > 0)
                .collect::<Vec<_>>();

            view! {
                <table>
                    <TableContent rows/>
                </table>
            }
            .into_view()
        } else {
            view! { <p>no rows</p> }.into_view()
        }
    };

    view! { {trigger_rows} }
}

#[component]
pub fn GameReplayPage() -> impl IntoView {
    let seed: GameSeed = [0; 32];
    let (value, set_value) = create_signal(6.0);
    view! {
        <div class="main_left">

            <RandomOpponentGameBoard seed=seed/>
        </div>
        <div class="main_right">

            <Slider
                min=0.0
                max=1.0
                step=0.0001
                value=value
                set_value=set_value
                value_display=move |v| format!("{v:.4}")
            />

            <Tabs mount=Mount::Once>
                <Tab name="tab-1" label="Tab 1".into_view()>
                    <div style="font-size: 8em; color: #8f39d3;">
                        <Icon icon=i::AiCarryOutTwotone style="color: green"/>
                        <Icon icon=i::BiGraphql width="2em" height="2em" style="color: green"/>
                        <Icon icon=i::BiGraphql style="color: orange"/>
                        <Icon icon=i::Bs1Circle style="color: red"/>
                        <Icon icon=i::FaBarsSolid/>
                        <Icon icon=i::ImPagebreak/>
                        <Icon icon=i::ImPageBreak/>
                        <Icon icon=i::OcAlertSm/>
                        <Icon icon=i::OcAlertLg width="1em" height="2em"/>

                    </div>
                </Tab>

                <Tab name="tab-2" label="Tab 2".into_view()>
                    <TableDemo/>
                </Tab>

            </Tabs>
        </div>
    }
}

#[derive(TableRow, Clone)]
#[table(impl_vec_data_provider)]
pub struct FullGameReplayTableRow {
    #[table(renderer = "WeedRenderer")]
    pub user_id: uuid::Uuid,
    #[table(renderer = "SeedRenderer")]
    pub init_seed: GameSeed,
    pub start_time: i64,
    pub num_segments: usize,
}

impl FullGameReplayTableRow {
    pub fn new(db_row: FullGameReplayDbRow) -> Self {
        Self {
            user_id: db_row.id.user_id,
            init_seed: db_row.id.init_seed,
            start_time: db_row.id.start_time,
            num_segments: db_row.segments.len(),
        }
    }
}
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
            <p>{move || format!("{:?}", value.get())}</p>
        </td>
    }
}

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
