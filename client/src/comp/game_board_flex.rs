
use game::{api::{user::UserProfile, websocket::GetProfile}, bot::get_bot_from_id, tet};
use leptos::*;
use leptos_use::{use_interval_with_options, UseIntervalOptions, UseIntervalReturn};

use crate::{audio3::play_sound_effect, comp::game_board::BoardTable, style::{flex_gameboard_style, GameBoardTetStyle}, websocket::demo_comp::call_api_sync, };
#[component]
pub fn GameBoardTimer( game_state: RwSignal<tet::GameState>, pre_countdown_text: ReadSignal<String>,)->impl IntoView{

    let UseIntervalReturn {counter:counter_timer,pause:pause_timer,resume:resume_timer, reset, is_active }  = use_interval_with_options( 1000, UseIntervalOptions::default().immediate(true) );

    let timer_str = move || { 
        counter_timer.track();
        if let Some(s) = game_state.try_get_untracked() {
            let pre = pre_countdown_text.get();
            if s.game_over || !pre.is_empty() {
                pause_timer();
                reset();
            } else {
                if !is_active.get() {
                    resume_timer();
                }
            }
            format!("{}", s.current_time_string()) 
        } else {
            "".to_string()
        }
        
    };
    timer_str
}

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
                    if current.replay.replay_slices.len() != _prev.replay.replay_slices.len() && current.replay.replay_slices.len()>=1 {
                        let last_slice = current.replay.replay_slices.last().expect("last of vec longer than 1");
                        let sound = match last_slice.event.action {
                            tet::TetAction::HardDrop    => "hard_drop",
                            tet::TetAction::MoveLeft    => "move_left", 
                            tet::TetAction::MoveRight   => "move_right", 
                            tet::TetAction::SoftDrop    => "soft_drop",
                            tet::TetAction::Hold        =>"hold",
                            tet::TetAction::RotateLeft  => "rotate_left",
                            tet::TetAction::RotateRight => "rotate_right",
                            tet::TetAction::Nothing       => "",
                        };
                        crate::audio3::play_sound_effect(sound);                        
                        if current.game_over{
                            crate::audio3::play_sound_effect("game_over");
                        }
                        if current.total_lines != _prev.total_lines {
                            let clr_line_effect = match current.total_lines - _prev.total_lines {
                                1 => "clear_line_1",
                                2 => "clear_line_2",
                                3 => "clear_line_3",
                                4 => "clear_line_4",
                                _ => ""
                            };
                            crate::audio3::play_sound_effect(clr_line_effect);
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
                    let pre_effect = match ccurrent.as_str() {
                        "3" => "pre_123_1",
                        "2" => "pre_123_2",
                        "1" => "pre_123_3",
                        "Go" => "pre_123_4",
                        _=> ""
                    };
                    play_sound_effect(pre_effect)
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

    let _countdown_view = view! {
        <Show when=move || {
            game_state.get().game_over || pre_countdown_text.get().len() > 0
        }>
            <div style="width: 0px; height: 0px; margin: 0px; position: relative;  z-index: 999;">
                <div style="position: absolute; width: calc(var(--h-table-width)); height:  calc(var(--h-table-width)*2); container-type:size;   z-index: 999;           ">
                    <Show when=move || game_state.get().game_over>
                        <div class="gameover">
                            <div
                                class="game_over_display"
                                on:click=move |_| on_reset_game.call(())
                            >
                                you lose
                            </div>
                        </div>
                    </Show>
                    <Show when=move || { pre_countdown_text.get().len() > 0 }>
                        <div class="pre_game_countdown">
                            <div class="pre_game_countdown_display">
                                {pre_countdown_text}
                            </div>
                        </div>
                    </Show>
                </div>
            </div>
        </Show>
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
                user_profile.get().expect("is_some").display_name
            } else {
                "".to_string()
            }}

        </h3>
    }};

    view! {
        <div
            class=_style_name
            style="height:100%;flex-direction: column;display: flex;         container-type: size;
            aspect-ratio: 0.66;  margin:auto;
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

            <div style="height:15%;flex-direction: row;display: flex;"></div>

            <div style="height: 75%;flex-direction: row;display: flex;">

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
                    <div style="width:100%;height:37%;">
                        {move || { format!("{}", game_state.get().get_debug_info()) }}
                    </div>
                    <div style="width:100%;height:37%;">
                        <GameBoardTimer game_state pre_countdown_text/>
                    </div>

                </div>

                // MAIN
                <div style="width:50%;height:100%;flex-direction: row;display: flex;">
                    <div style="width:3%;height:100%;flex-direction: column;display: flex;"></div>

                    <div
                        style="width:95%;height:100%;flex-direction: column;display: flex;"
                        class="calculate_table_width"
                    >

                        {_countdown_view}

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
            <div style="height:15%;flex-direction: column;display: flex;container-type: size;">
                {profile_view}
            </div>
        </div>
    }
}


#[component]
pub fn FlexText(
    #[prop(into)]
    text: MaybeSignal<String>,
    
    #[prop(into)]
    #[prop(default=80.0)]
    size_cqh: f32,
) -> impl IntoView {
    view!{
        <div style="width:100%;height:100%; container-type: size; margin:0px; padding:0px;">
            <p style={format!("font-size:{size_cqh}cqh;text-align: center;margin:0px; padding:0px;")} >
                {text}
            </p>
        </div>
    }
}