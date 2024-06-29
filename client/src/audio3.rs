use leptos::{expect_context, queue_microtask, SignalGetUntracked};
//use leptos::{provide_context, queue_microtask};
use wasm_bindgen::prelude::*;

use crate::page::settings::server_api::UserSettingSignals;

// create rust functions from the javascript functions
#[wasm_bindgen(module = "/public/js/audio.js")]
extern "C" {
    // pub fn init_audio_js() -> JsValue;
    pub fn play_sound_js(sound_name:String) -> JsValue;
    pub fn stop_sound_js(sound_name:String) -> JsValue;
}

#[derive(Debug, Clone, PartialEq)]
pub struct Audio3Context {
    // audio_sounds_js: JsValue,
}

pub fn provide_audio_context() {
    // init_audio_js();
    leptos::provide_context(Audio3Context{});
}

pub fn play_sound(audio_key: &str) {
    let user_setting_signal = expect_context::<UserSettingSignals>();

    let is_disabled = user_setting_signal.sound_disabled.get_untracked();
    if is_disabled {
        return;
    }

    let audio_key = audio_key.to_string();
    let _context : Audio3Context= leptos::expect_context();
    queue_microtask(move || {
        play_sound_js( audio_key);
    });
}

pub fn stop_sound(audio_key: &str) {
    let audio_key = audio_key.to_string();
    let _context : Audio3Context= leptos::expect_context();
    queue_microtask(move || {
        stop_sound_js(audio_key);
    });
}