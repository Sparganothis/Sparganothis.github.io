
use game::{api::{user::UserProfile, websocket::GetProfile}, bot::get_bot_from_id, tet};
use leptos::*;

use crate::{audio3::play_sound_effect, comp::game_board::BoardTable, style::{flex_gameboard_style, GameBoardTetStyle}, websocket::demo_comp::call_api_sync, };



#[component]
pub fn GameBoardFlex(
    #[prop(into)] game_state: RwSignal<tet::GameState>,

    #[prop(default = Callback::<()>::new(move |_| {}))]
    #[prop(optional)]
    on_reset_game: Callback<()>,

    #[prop(default = Callback::<(i8, i8)>::new(move |_| {}))]
    #[prop(optional)]
    on_main_cell_click: Callback<(i8, i8)>,

    #[prop(into)]
    #[prop(default = create_rw_signal("".to_string()).read_only())]
    #[prop(optional)]
    pre_countdown_text: ReadSignal<String>,

    #[prop(into)]
    #[prop(default = view!{}.into_view())]
    top_bar: View,
    // #[prop(into)]
    // #[prop(default = view!{}.into_view())]
    // bottom_bar: View,

    #[prop(default = false)]
    enable_sound: bool,

    player_id: uuid::Uuid,

) -> impl IntoView {
    let tet_style = GameBoardTetStyle::new();
    if enable_sound{
        let _stop_sounds = leptos::watch(
            move|| game_state.get(),
            move |current, _prev, _| {
                if let Some(_prev) = _prev {
                    if current.replay.replay_slices.len() != _prev.replay.replay_slices.len() {
                        let last_slice = current.replay.replay_slices.last().unwrap();
                        let sound = match last_slice.event.action {
                            tet::TetAction::HardDrop => "hard_drop",
                            tet::TetAction::MoveRight|tet::TetAction::MoveLeft => "move", 
                            tet::TetAction::SoftDrop => "soft_drop",
                            tet::TetAction::Hold =>"hold",
                            tet::TetAction::RotateLeft | tet::TetAction::RotateRight => "rotate",
                            
                            tet::TetAction::Nothing => "",
                        };
                        crate::audio3::play_sound_effect(sound);                        
                        if current.game_over{
                            crate::audio3::play_sound_effect("game_over");
                        }
                        if current.total_lines != _prev.total_lines {
                            crate::audio3::play_sound_effect("clear_line");
                        }
                    }
                }
            },
            false
        );

        let _stop_sounds = watch ( 
            move || pre_countdown_text.get(),
            move |ccurrent, _prev, _| {
                if ccurrent.len() > 0 {
                    play_sound_effect("pre_123")
                }
            },
            false
        );
    
    }
    
    let _style_name = flex_gameboard_style(tet_style, )
        .get_class_name()
        .to_owned();

    let hold_board =
        create_read_slice(game_state, |state: &tet::GameState| state.get_hold_board());

    let next_board =
        create_read_slice(game_state, |state: &tet::GameState| state.get_next_board());

    let main_board =
        create_read_slice(game_state, |state: &tet::GameState| state.main_board);

    let gameover = view! {
        <Show when=move || game_state.get().game_over fallback=|| view! {}>
            <div class="game_over_display" on:click=move |_| on_reset_game.call(())>
                you lose
            </div>
        </Show>
    };

    let pre_countdown = view! {
        <Show when=move || { pre_countdown_text.get().len() > 0 } fallback=|| view! {}>
            <div class="pre_game_countdown_display">{pre_countdown_text}</div>
        </Show>
    };

    // TODO PUT THIS ON SCREEN
    let _countdown_view = view! {
        <div class="gameover">{gameover}</div>
        <div class="pre_game_countdown">{pre_countdown}</div>
    };

    let user_profile = create_rw_signal(None);

    if let Ok(bot_name) = get_bot_from_id(player_id) {
        user_profile.set(Some(UserProfile { 
            display_name: format!("BOT {}", bot_name) }));
    } else {
        create_effect(move |_|{
            call_api_sync::<GetProfile>(player_id, move |r| {
                user_profile.set(Some(r));
            });
        });
    }

    let profile_view = {  move || view! {
        <div style="height: 20%; width: 10cqh;"></div>

        <p style="font-size: 15cqh; height: 20%;">
            {format!("{:?}", player_id)}
            <a href=format!("/user/{:?}", player_id)>(view)</a>
        </p>

        <h3 style="font-size: 35cqh; height: 40%;">
            {if user_profile.get().is_some() {
                user_profile.get().unwrap().display_name
            } else {
                "".to_string()
            }}

        </h3>
    }};

    view! {
        <div
            class=_style_name
            style="border:solid purple 1px;height:100%;flex-direction: column;display: flex;         container-type: size;
            --h-main-width:99.9cqw;   "
        >

            <div
                id="top-bar"
                style="width: 0px; height: 0px; margin: 0px; position: relative"
            >
                <div style="position: absolute; width: calc(var(--h-main-width)); height:  calc(var(--h-main-width)*0.5); container-type:size;">
                    {top_bar}
                </div>
            </div>

            <div style="border:solid purple 1px;height:15%;flex-direction: row;display: flex;"></div>

            <div style="border:solid red 1px;height: 75%;flex-direction: row;display: flex;">

                // HOLD
                <div style="width:25%;height:100%;flex-direction: column;display: flex;">
                    <div style="width:100%;height:10%; container-type: size;">
                        <h3 style="font-size:80cqh; text-align: center;">HOLD</h3>
                    </div>
                    <div style="width:100%;height:17%;flex-direction: row;display: flex;">
                        <div style="width:7%;height:100%;"></div>
                        // HOLD BORD
                        <div style="width:86%;height:100%;">
                            <BoardTable board=hold_board/>
                        </div>
                        <div style="width:7%;height:100%;"></div>

                    </div>
                    <div style="width:100%;height:30%;"></div>

                    // SCORE BOARD
                    <div style="width:100%;height:6%; container-type: size;">
                        <h3 style="font-size:80cqh; text-align: center;">
                            {move || { format!("{:?}", game_state.get().score) }}
                        </h3>
                    </div>
                    <div style="width:100%;height:37%;"></div>

                </div>

                // MAIN
                <div style="width:50%;height:100%;flex-direction: row;display: flex;">
                    <div style="width:3%;height:100%;flex-direction: column;display: flex;"></div>

                    <div
                        style="width:95%;height:100%;flex-direction: column;display: flex;"
                        class="calculate_table_width"
                    >

                        <div style="width: 0px; height: 0px; margin: 0px; position: relative;  z-index: 999;">
                            <div style="position: absolute; width: calc(var(--h-table-width)); height:  calc(var(--h-table-width)*2); container-type:size;   z-index: 999;           ">
                                {_countdown_view}
                            </div>
                        </div>

                        <BoardTable board=main_board on_click=on_main_cell_click/>
                    </div>

                    <div style="width:2%;height:100%;flex-direction: column;display: flex;"></div>
                </div>

                // NEXT
                <div style="width:25%;height:100%;flex-direction: column;display: flex;">
                    <div style="width:100%;height:10%;container-type: size;">
                        <h3 style="font-size:80cqh; text-align: center;">NEXT</h3>
                    </div>
                    <div style="width:100%;height:75%;flex-direction: row;display: flex;">
                        <div style="width:12%;height:100%;"></div>
                        // NEXT BORD
                        <div style="width:76%;height:100%;">
                            <BoardTable board=next_board/>
                        </div>
                        <div style="width:12%;height:100%;"></div>

                    </div>
                    <div style="width:100%;height:15%;"></div>
                </div>
            </div>
            // PLAYER ID
            <div style="border:solid yellow 1px;height:15%;flex-direction: column;display: flex;container-type: size;">
                {profile_view}
            </div>
        </div>
    }
}
