use leptonic::prelude::*;
use leptos_router::{use_navigate, use_params_map, NavigateOptions};
use std::collections::VecDeque;

use game::{
    api::websocket::{GetCustomGame, GetRandomWord, UpdateCustomGame},
    tet::{CellValue, CurrentPcsInfo, GameState, Tet},
};
use leptonic::{
    input::TextInput,
    select::Select,
};
use leptos::*;

use crate::{
    comp::{game_board::{GameBoard, PlayerGameBoardSingle}, multiselect_repeat::MultiSelectSmecher, table_custom_games::ListAllCustomGames},
    websocket::demo_comp::{call_websocket_api, WebsocketAPI},
};

#[component]
pub fn MsPaintPlayPage() ->impl IntoView{
    let api = expect_context::<WebsocketAPI>();
    let game_state = create_rw_signal(GameState::empty());
    let params = use_params_map();
    let (save_name, set_save_name) = create_signal("".to_string());

    let api2 = api.clone();
    create_effect(
        move |_| {
            let api2 = api2.clone();
            
            let p = params.with(|params| params.get("save_id").cloned());
            log::info!("readcting to URL papram save_id = {:?}", p);
            if let Some(url_save_name) = p {
                    set_save_name.set(url_save_name.clone());
                    spawn_local(
                        async move {
                            if let Ok(ceva) = call_websocket_api::<GetCustomGame>(api2, url_save_name) {
                            if let Ok(ceva) = ceva.await {
                                game_state.set(ceva);
                            }
                        }
                });
            }
        }
    );
    view! {
        <h1>"play custom     | " {save_name}</h1>
        <PlayerGameBoardSingle state=game_state/>
    }
}
#[component]
pub fn MsPaintPage() -> impl IntoView {

    
    let api = expect_context::<WebsocketAPI>();
    let (save_name, set_save_name) = create_signal("".to_string());

    let (status, set_status) = create_signal("...".to_string());
    
    let params = use_params_map();

    let game_state = create_rw_signal(GameState::empty());

    let api2= api.clone();
    create_effect( move |_| {
        let p = params.with(|params| params.get("save_id").cloned());
        log::info!("readcting to URL papram save_id = {:?}", p);
        if let Some(url_save_name) = p {
            set_save_name.set(url_save_name.clone());
            let api2 = api2.clone();
            spawn_local(
                    async move {
                        if let Ok(ceva) = call_websocket_api::<GetCustomGame>(api2, url_save_name) {
                           if let Ok(ceva) = ceva.await {
                            game_state.set(ceva);
                           }
                        }
            });
        } else {
            let navigate = use_navigate();
    
            let api2 = api2.clone();
            spawn_local(async move {
                if let Ok(ceva) = call_websocket_api::<GetRandomWord>(api2, ()) {
                    let ceva = ceva.await;
                    if let Ok(ceva) = ceva {
                        set_save_name.set(ceva.clone());
                        let new_url = format!("/edit-custom-game/{}", ceva);
                        navigate(&new_url, NavigateOptions::default());
                    }
                }
            });
        }
    });

    let api2 = api.clone();
    let on_save = move |_| {
        let api = api2.clone();
        spawn_local(async move {
            if let Ok(ceva) = call_websocket_api::<UpdateCustomGame>(
                api,
                (save_name.get_untracked(), game_state.get_untracked()),
            ) {
                let ceva = ceva.await;
                if let Ok(_ceva) = ceva {
                    set_status.set("Save ok".to_string());
                } else {
                    set_status.set("SVDB EERR".to_string());
                }
            } else {
                set_status.set("Connect errrr".to_string());
            }
        });
    };
    let on_save = leptonic::callback::Consumer::<leptos::ev::MouseEvent>::new(on_save);

    view! {
        <div class="main_left">
            <MsPaintGameBoard game_state save_name/>
        </div>
        <div class="main_right">

            <Tabs mount=Mount::WhenShown>

                <Tab name="current-cusxtom-game" label="Edit Custom Game".into_view()>
                    <NextPeaceSelector game_state/>
                    <CurrentPeaceSelector game_state/>
                    <h1>save name</h1>
                    <TextInput get=save_name set=set_save_name/>
                    <Button on_click=on_save color=ButtonColor::Info>
                        "Save"
                    </Button>
                    {move || status.get()}

                    <a href=move || {
                        format!("/play-custom-game/{}", save_name.get())
                    }>Play</a>
                </Tab>

                <Tab name="list-custom-games" label="All Custom Games".into_view()>

                    <ListAllCustomGames/>
                </Tab>

            </Tabs>
        </div>
    }
}


#[component]
pub fn CurrentPeaceSelector(game_state: RwSignal<GameState>) -> impl IntoView {
    let selected =
        move || game_state.with(|game_state| game_state.current_pcs.unwrap().tet);
    let set_selected = move |new_current: Tet| {
        game_state.update(|game_state| {
            let _ = game_state
                .main_board
                .delete_piece(&game_state.current_pcs.unwrap());
            game_state.current_pcs = Some(CurrentPcsInfo {
                pos: new_current.spawn_pos(),
                tet: new_current,
                rs: game_state.current_pcs.unwrap().rs,
                id: game_state.current_pcs.unwrap().id,
            });
            let _ = game_state
                .main_board
                .spawn_piece(&game_state.current_pcs.unwrap());
        });
    };
    view! {
        <h1>"select current piece"</h1>
        <Select
            options=Tet::all()
            search_text_provider=move |o| format!("{o:?}")
            render_option=move |o| format!("{o:?}")
            selected=selected
            set_selected=set_selected
        />
    }
}

#[component]
pub fn NextPeaceSelector(game_state: RwSignal<GameState>) -> impl IntoView {
    let get_next = move || {
        game_state
            .get()
            .next_pcs
            .iter()
            .cloned()
            .collect::<Vec<_>>()
    };
    let set_next = Callback::new(move |v: Vec<Tet>| {
        game_state.update(|game_state| {
            game_state.next_pcs = v.iter().cloned().collect::<VecDeque<_>>();
        })
    });
    view! {
        <h1>"select next pieces"</h1>
        <MultiSelectSmecher
            options=Tet::all()
            selected=get_next.into_signal()
            set_selected=set_next
        />
    }
}
#[component]
pub fn MsPaintGameBoard(game_state: RwSignal<GameState>, save_name: ReadSignal<String>) -> impl IntoView {
    let on_reset_game = Callback::<()>::new(move |_| {});
    let on_click = Callback::<(i8, i8)>::new(move |(y, x)| {
        game_state.update(|game_state| {
            let old_value = game_state.main_board.v[y as usize][x as usize];
            let new_value = match old_value {
                CellValue::Piece(_) => CellValue::Empty,
                CellValue::Garbage => CellValue::Empty,
                CellValue::Empty => CellValue::Piece(Tet::J),
                CellValue::Ghost => CellValue::Piece(Tet::J),
            };
            game_state.main_board.v[y as usize][x as usize] = new_value;
        })
    });

    view! {
        <h1>mspaint.exe " | " {save_name}</h1>
        <GameBoard game_state on_reset_game on_main_cell_click=on_click/>
    }
}
