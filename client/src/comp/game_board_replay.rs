use crate::comp::game_board::GameBoard;
use crate::websocket::demo_comp::{call_websocket_api, WebsocketAPI};
use game::api::game_replay::GameId;
use game::api::websocket::GetAllSegments;
use game::tet::{GameReplaySegment, GameState};
use game::timestamp::get_timestamp_now_ms;
use leptonic::prelude::*;
use leptonic::slider::Slider;
use leptos::*;

#[component]
pub fn ReplayGameBoard(game_id: GameId) -> impl IntoView {
    let state_signal = create_rw_signal(GameState::new(&[0; 32], 0));
    let api: WebsocketAPI = expect_context();

    let all_segments = create_resource(
        || (),
        move |_| {
            let api3 = api.clone();
            async move {
                // log::info!("calling websocket api");
                let r = call_websocket_api::<GetAllSegments>(api3, game_id)
                    .expect("cannot obtain future")
                    .await;
                // log::info!("got back response: {:?}", r);
                r
            }
        },
    );
    let (get_slider, set_slider) = create_signal(6.0);
    let status_message = create_rw_signal(String::from("downloading..."));

    // let api2: WebsocketAPI = expect_context();
    let all_states = create_memo(move |_| {
        if let Some(Ok(all_segments)) = all_segments.get() {
            let t0 = get_timestamp_now_ms();
            status_message.set("simulating...".to_string());
            let mut current_state = match all_segments.get(0) {
                Some(GameReplaySegment::Init(_replay)) => {
                    GameState::new(&_replay.init_seed, _replay.start_time)
                }
                _ => {
                    log::error!("got no init segment");
                    return vec![];
                }
            };
            let mut all_states = vec![];
            all_states.push(current_state.clone());
            for segment in &all_segments[1..] {
                match segment {
                    GameReplaySegment::Init(_) => {
                        log::error!("got two init segments");
                        return vec![];
                    }
                    GameReplaySegment::Update(_slice) => {
                        if let Err(e) = current_state.accept_replay_slice(_slice) {
                            log::error!("failed to accept replay slice: {:#?}", e);
                            return vec![];
                        }
                    }
                    GameReplaySegment::GameOver => {
                        if !current_state.game_over {
                            log::error!("expected to see game over in simulation!");
                        }
                    }
                }
                all_states.push(current_state.clone());
            }
            let t1 = get_timestamp_now_ms();
            status_message.set(format!("done {}ms", t1 - t0));
            all_states
        } else {
            vec![]
        }
    });

    let update_state_on_slider_change = move || {
        let slider_val = get_slider.get() as usize;
        all_states.with(|all_states| {
            if all_states.is_empty() {
                return view! { <p>"no data..."</p> }.into_view();
            }
            if slider_val >= all_states.len() {
                return view! { <p>"simulating..."</p> }.into_view();
            }
            state_signal.set(all_states[slider_val].clone());
            view! { <p>{status_message.get_untracked()}</p> }.into_view()
        })
    };

    let slider = move || {
        if let Some(Ok(all_segments)) = all_segments.get() {
            let maxval = all_segments.len() as f64;
            view! {
                <Slider
                    min=0.0
                    max=maxval
                    step=1.0
                    value=get_slider
                    set_value=set_slider
                    value_display=move |v| format!("{v:.0}/{maxval:.0}")
                />
            }
            .into_view()
        } else {
            view! { <p>loading...</p> }.into_view()
        }
    };

    let control_icons = {
        let is_backwards = create_rw_signal(false);
        let tick = create_rw_signal(0);
        let do_every_tick = create_rw_signal(4);

        let leptos_use::utils::Pausable { pause, resume, .. } =
            leptos_use::use_interval_fn(
                move || {
                    tick.set(tick.get_untracked() + 1);
                    if tick.get_untracked() % do_every_tick.get_untracked() == 0 {
                        let old_slider = get_slider.get_untracked();

                        let diff_slider = if is_backwards.get_untracked() {
                            -1.0
                        } else {
                            1.0
                        };
                        let mut new_slider = old_slider + diff_slider;
                        new_slider = new_slider
                            .max(0.0)
                            .min(all_states.with_untracked(|w| w.len() as f64));

                        set_slider.set(new_slider);
                    }
                },
                16,
            );

        let resume1 = resume.clone();
        let on_click_play = move |_| {
            log::info!("click on_click_play");
            is_backwards.set(false);
            do_every_tick.set(4);
            resume1();
        };

        let pause1 = pause.clone();
        let on_click_pause = move |_| {
            pause1();
        };

        let pause1 = pause.clone();
        let on_click_stop = move |_| {
            pause1();
            set_slider.set(0.0);
        };

        let resume1 = resume.clone();
        let on_click_rewind = move |_| {
            is_backwards.set(true);
            do_every_tick.set(1);
            resume1();
        };

        let resume1 = resume.clone();
        let on_click_fast = move |_| {
            is_backwards.set(false);
            do_every_tick.set(1);
            resume1();
        };

        let resume1 = resume.clone();
        let on_click_skip_to_end = move |_| {
            is_backwards.set(false);
            let len = all_states.with_untracked(|s| s.len());
            set_slider.set(len as f64);
            is_backwards.set(true);
            do_every_tick.set(1);
            resume1();
        };

        let resume1 = resume.clone();
        let on_click_skip_to_start = move |_| {
            is_backwards.set(false);
            set_slider.set(0.0 as f64);
            do_every_tick.set(1);
            is_backwards.set(false);
            resume1();
        };

        view! {
            <div class="control_icon_parent">

                <div class="control_icon_container">
                    <Icon
                        class="control_icon"
                        icon=icondata::BiSkipPreviousCircleRegular
                        on:click=on_click_skip_to_start
                        width="5vmin"
                        height="5vmin"
                    />
                </div>

                <div class="control_icon_container">
                    <Icon
                        class="control_icon"
                        icon=icondata::BiRewindCircleRegular
                        style="color: black"
                        on:click=on_click_rewind
                        width="5vmin"
                        height="5vmin"
                    />
                </div>

                <div class="control_icon_container">
                    <Icon
                        class="control_icon"
                        icon=icondata::BiPlayCircleRegular
                        on:click=on_click_play
                        width="5vmin"
                        height="5vmin"
                    />
                </div>

                <div class="control_icon_container">
                    <Icon
                        class="control_icon"
                        icon=icondata::BiPauseCircleRegular
                        on:click=on_click_pause
                        width="5vmin"
                        height="5vmin"
                    />
                </div>

                <div class="control_icon_container">
                    <Icon
                        class="control_icon"
                        icon=icondata::BiStopCircleRegular
                        on:click=on_click_stop
                        width="5vmin"
                        height="5vmin"
                    />
                </div>

                <div class="control_icon_container">
                    <Icon
                        class="control_icon"
                        icon=icondata::BiFastForwardCircleRegular
                        on:click=on_click_fast
                        width="5vmin"
                        height="5vmin"
                    />
                </div>

                <div class="control_icon_container">
                    <Icon
                        class="control_icon"
                        icon=icondata::BiSkipNextCircleRegular
                        on:click=on_click_skip_to_end
                        width="5vmin"
                        height="5vmin"
                    />
                </div>

            </div>
        }
    };

    let on_reset: Callback<()> = Callback::<()>::new(move |_| {});
    view! {
        {slider}
        {update_state_on_slider_change}
        {control_icons}

        <GameBoard on_reset_game=on_reset game_state=state_signal/>
    }
}
