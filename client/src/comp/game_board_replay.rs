use game::tet::{segments_to_states, GameReplaySegment, GameState};
use game::timestamp::get_timestamp_now_ms;
use leptonic::prelude::*;
use leptonic::slider::Slider;
use leptos::*;
use wasm_bindgen::JsValue;
use crate::comp::game_board_flex::GameBoardFlex;


fn _bytes_to_array(bytes: &[u8]) -> JsValue {
    let array: js_sys::Uint8Array =
        js_sys::Uint8Array::new_with_length(bytes.len().try_into().expect("ccreate js int8 array"));

    array.copy_from(bytes);

    array.into()
}

pub fn _bytes_to_blob(bytes: &[u8], content_type: Option<&str>) -> web_sys::Blob {
    let array = _bytes_to_array(bytes);

    let blob_parts_array = js_sys::Array::new();

    blob_parts_array.push(&array);

    let mut options = web_sys::BlobPropertyBag::new();

    match content_type {
        Some(content_type) => {
            options.type_(content_type);
        }
        None => {}
    };

    web_sys::Blob::new_with_u8_array_sequence_and_options(&blob_parts_array, &options)
        .expect("create blob")
}


#[component]
pub fn ReplayGameBoardFromSegmments(
    all_segments: Signal<Vec<GameReplaySegment>>,
    slider: RwSignal<f64>,
    game_state: RwSignal<GameState>,

    #[prop(default = false)]
    hide_controller: bool,

    player_id: uuid::Uuid,

    top_bar_override: Option<View>,

) -> impl IntoView {
    let status_message = create_rw_signal(String::from("downloading..."));

    // let api2: WebsocketAPI = expect_context();
    let all_states = create_memo(move |_| {
        let all_segments = all_segments.get();
        let t0 = get_timestamp_now_ms();
        status_message.set("simulating...".to_string());
        log::info!("segments_to_states: {}", all_segments.len());
        let mut all_segemnts_2 = all_segments.clone();
        all_segemnts_2.truncate(6666);
        let all_states = segments_to_states(&all_segemnts_2);
        log::info!("segmments to states OK!");
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

        let download_href = move || {
            let seg = all_segments.get();
            let bytes = bincode::serialize(&seg).expect("must serialize");
            log::info!("download size: {} bytes", bytes.len());
            let b = _bytes_to_blob(&bytes, Some("application/octet-stream"));
            web_sys::Url::create_object_url_with_blob(&b).expect("create url")
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

                <div class="control_icon_container">
                    <a href=download_href download="Sparganothis.replay.bin">
                        <Icon
                            class="control_icon"
                            icon=icondata::BsDownload
                            width="5vmin"
                            height="5vmin"
                        />
                    </a>
                </div>

            </div>
        }.into_view()
    };

    let on_reset: Callback<()> = Callback::<()>::new(move |_| {});
    view! {
        <GameBoardFlex
            on_reset_game=on_reset
            game_state=game_state
            top_bar=top_bar_override
                .unwrap_or(
                    view! {
                        {make_slider}
                        {control_icons}
                    }
                        .into_view(),
                )

            player_id
        />
    }
}