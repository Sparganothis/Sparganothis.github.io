use leptonic::prelude::*;
use leptos_router::{use_navigate, use_params_map, NavigateOptions};

use game::{
    api::websocket::{GetCustomGame, GetRandomWord, UpdateCustomGame}, bot::get_bot_id, tet::{CellValue, CurrentPcsInfo, GameState, Tet}
};
use leptonic::{
    input::TextInput,
    select::Select,
};
use leptos::*;

use crate::{
    comp::{game_board_player::PlayerGameBoardSingle, multiselect_repeat::MultiSelectSmecher, table_custom_games::ListAllCustomGames},
    websocket::demo_comp::{call_api_sync, call_api_sync_or_error},
};

#[component]
pub fn MsPaintPlayPage() ->impl IntoView{
    let game_state = create_rw_signal(GameState::empty());
    let params = use_params_map();
    let (save_name, set_save_name) = create_signal("".to_string());

    create_effect(
        move |_| {
            let p = params.with(|params| params.get("save_id").cloned());
            log::info!("readcting to URL papram save_id = {:?}", p);
            if let Some(url_save_name) = p {
                    set_save_name.set(url_save_name.clone());

                    call_api_sync::<GetCustomGame>(url_save_name, move |r| {
                        game_state.set(r);
                    });
            }
        }
    );
    view! {
        <div class="main_left">
            <PlayerGameBoardSingle
                state=game_state
                top_bar=view! { <h1>"play custom     | " {save_name}</h1> }.into_view()
                player_id=get_bot_id("random").expect("rand bot must exist")
            />
        </div>
    }
}


#[component]
pub fn MsPaintPage() -> impl IntoView {
    let (save_name, set_save_name) = create_signal("".to_string());
    let (status, set_status) = create_signal("...".to_string());
    
    let params = use_params_map();
    let game_state = create_rw_signal(GameState::empty());

    create_effect( move |_| {
        let p = params.with(|params| params.get("save_id").cloned());
        log::info!("readcting to URL papram save_id = {:?}", p);
        if let Some(url_save_name) = p {
            set_save_name.set(url_save_name.clone());
            call_api_sync_or_error::<GetCustomGame>(url_save_name, move |r| {
                game_state.set(r);
            }, move |err| {
                log::info!("custom get game err: {}", err);
            });
        } else {
            let navigate = use_navigate();
            call_api_sync::<GetRandomWord>((), move |r: String| {
                set_save_name.set(r.clone());
                let new_url = format!("/edit-custom-game/{}", r);
                navigate(&new_url, NavigateOptions{replace:true, ..Default::default()});
            });
        }
    });

    let on_save = move |_| {
        call_api_sync::<UpdateCustomGame>((save_name.get_untracked(), game_state.get_untracked()), move |r| {
            log::info!("saved {r:?}");
            set_status.set("Save ok".to_string());
        });
    };
    let on_save = leptonic::callback::Consumer::<leptos::ev::MouseEvent>::new(on_save);

    view! {
        <div class="main_left">
            <MsPaintGameBoard game_state save_name/>
        </div>
        <div class="main_right">

            <Tabs mount=Mount::Once>

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
            .get_next_pcs()
    };
    let set_next = Callback::new(move |v: Vec<Tet>| {
        game_state.update(|game_state| {
            game_state.set_next_pcs(v.clone());
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
            let old_value = game_state.main_board.get_cell(y as i8, x as i8).unwrap();
            let new_value = match old_value {
                CellValue::Piece(_) => CellValue::Empty,
                CellValue::Garbage => CellValue::Empty,
                CellValue::Empty => CellValue::Piece(Tet::J),
                CellValue::Ghost => CellValue::Piece(Tet::J),
            };
            game_state.main_board.set_cell(y as i8, x as i8, new_value);
            
        })
    });

    let title = create_memo(
        move |_| 
        format!("mspaint.exe | {}", save_name.get())
    );

    view! {
        <GameBoardFlex
            game_state
            on_reset_game
            on_main_cell_click=on_click
            top_bar=view! { <h1 style="font-size: 8cqw;">{title}</h1> }.into_view()

            player_id=get_bot_id("random").unwrap()
        />
    }
}
use crate::comp::game_board_flex::GameBoardFlex;