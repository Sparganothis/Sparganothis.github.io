use leptos::{expect_context, queue_microtask, SignalGetUntracked};
//use leptos::{provide_context, queue_microtask};
use wasm_bindgen::prelude::*;

use crate::page::settings::server_api::UserSettingSignals;

// create rust functions from the javascript functions
#[wasm_bindgen(module = "/public/js/audio.js")]
extern "C" {
    // pub fn init_audio_js() -> JsValue;
    pub fn play_sound_js(sound_name:String, volume: f64) -> JsValue;
    pub fn stop_sound_js(sound_name:String) -> JsValue;
    pub fn change_global_volume_js(volume: f64) -> JsValue;
    pub fn stop_all_sound_js() -> JsValue;
    pub fn change_sound_volume_js(sound_name: String, volume: f64) -> JsValue;

}

#[derive(Debug, Clone, PartialEq)]
pub struct Audio3Context {
    // audio_sounds_js: JsValue,
}

pub fn provide_audio_context() {
    // init_audio_js();
    leptos::provide_context(Audio3Context{});
}

pub fn play_sound_effect(audio_key: &str) {
    if audio_key.len() == 0 {
        return;
    }
    let user_setting_signal = expect_context::<UserSettingSignals>();

    let is_enabled = if audio_key != "mmenu_mmusicc" {
        user_setting_signal.all_sound_enabled.get_untracked()
    } else {
        user_setting_signal.sound_menu_music_enabled.get_untracked()
    };
    if !is_enabled {
        return;
    }
    let audio_key = audio_key.to_string();
    let _context : Audio3Context= leptos::expect_context();
    queue_microtask(move || {
        log::info!("play soundn js sound js");
        play_sound_js( audio_key, 100.0);
    });
}

pub fn stop_sound(audio_key: &str) {
    let audio_key = audio_key.to_string();
    let _context : Audio3Context= leptos::expect_context();
    queue_microtask(move || {
        log::info!("stop sound js");
        stop_sound_js(audio_key);
    });
}