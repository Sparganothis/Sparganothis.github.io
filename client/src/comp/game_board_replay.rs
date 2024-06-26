use game::tet::{GameReplaySegment, GameState};
use game::timestamp::get_timestamp_now_ms;
use leptonic::prelude::*;
use leptonic::slider::Slider;
use leptos::*;

#[component]
pub fn ReplayGameBoardFromSegmments(
    all_segments: Signal<Vec<GameReplaySegment>>,
    slider: RwSignal<f64>,
    game_state: RwSignal<GameState>,

    #[prop(default = false)]
    hide_controller: bool,

) -> impl IntoView {
    let status_message = create_rw_signal(String::from("downloading..."));

    // let api2: WebsocketAPI = expect_context();
    let all_states = create_memo(move |_| {
        let all_segments = all_segments.get();
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
                    current_state.game_over = true;
                }
            }
            all_states.push(current_state.clone());
        }
        let t1 = get_timestamp_now_ms();
        status_message.set(format!("done {}ms", t1 - t0));
        all_states
    });

    create_effect(move |_| {
        let slider_val = slider.get() as usize;
        all_states.with(|all_states| {
            if all_states.is_empty() {
                return view! { <p>"no data..."</p> }.into_view();
            }
            if slider_val >= all_states.len() {
                return view! { <p>"simulating..."</p> }.into_view();
            }
            game_state.set(all_states[slider_val].clone());
            view! { <p>{status_message.get_untracked()}</p> }.into_view()
        })
    });

    let make_slider = move || {
        if hide_controller {view!{}.into_view()} else {

            let all_segments = all_segments.get();
            let maxval = (all_segments.len() as f64 - 1.0).max(1.0);
            view! {
                <Slider
                    min=0.0
                    max=maxval
                    step=1.0
                    value=slider.read_only()
                    set_value=slider.write_only()
                    value_display=move |v| format!("{v:.0}/{maxval:.0}")
                />
            }
            .into_view()
        }
    };

    let is_backwards = create_rw_signal(false);
    let tick = create_rw_signal(0);
    let do_every_tick = create_rw_signal(4);

    let leptos_use::utils::Pausable { pause, resume, .. } =
        leptos_use::use_interval_fn(
            move || {
                tick.set(tick.get_untracked() + 1);
                if tick.get_untracked() % do_every_tick.get_untracked() == 0 {
                    let old_slider = slider.get_untracked();

                    let diff_slider = if is_backwards.get_untracked() {
                        -1.0
                    } else {
                        1.0
                    };
                    let mut new_slider = old_slider + diff_slider;
                    new_slider = new_slider
                        .max(0.0)
                        .min(all_states.with_untracked(|w| w.len() as f64 - 1.0));

                    slider.set(new_slider);
                }
            },
            25,
        );

    let control_icons = if hide_controller {view!{}.into_view()} else {
        let pause1 = pause.clone();
        create_effect(move |_| {
            let _count = all_states.with(|w| w.len());
            let _sl = slider.get();
            if _count > 10 {
                if _sl >(( _count  as f64)-1.1) {
                    pause1();
                }
            }
        });

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
            slider.set(0.0);
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

        let pause1 = pause.clone();
        let on_click_one_right = move |_| {
            is_backwards.set(false);
            let len = all_states.with_untracked(|s| s.len());
            slider.update(|s| *s = (*s + 1.0).min(len as f64 - 1.0));
            do_every_tick.set(4);
            pause1();
            log::info!("right one");
        };

        let pause1 = pause.clone();
        let on_click_left_one = move |_| {
            is_backwards.set(false);
            slider.update(|s| *s = (*s - 1.0).max(0.0));
            do_every_tick.set(4);
            pause1();
            log::info!("left one");
        };

        view! {
            <div class="control_icon_parent">

                <div class="control_icon_container">
                    <Icon
                        class="control_icon"
                        icon=icondata::AiCaretLeftFilled
                        on:click=on_click_left_one
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
                        icon=icondata::AiCaretRightFilled
                        on:click=on_click_one_right
                        width="5vmin"
                        height="5vmin"
                    />
                </div>

            </div>
        }.into_view()
    };

    let on_reset: Callback<()> = Callback::<()>::new(move |_| {});
    view! {
        <GameBoardFlex
            on_reset_game=on_reset
            game_state=game_state
            top_bar=view! {
                {make_slider}
                {control_icons}
            }
                .into_view()
        />
    }
}
use crate::comp::game_board_flex::GameBoardFlex;