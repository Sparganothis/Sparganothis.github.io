//use leptos::{provide_context, queue_microtask};
use wasm_bindgen::prelude::*;

// create rust functions from the javascript functions
#[wasm_bindgen(module = "/public/js/audio.js")]
extern "C" {
    pub fn init_audio_js() -> JsValue;
    pub fn play_sound_js(sound_items: JsValue, sound_name:String) -> JsValue;
    pub fn stop_sound_js(sound_items: JsValue, sound_name:String) -> JsValue;
}

#[derive(Debug, Clone, PartialEq)]
pub struct Audio3Context {
    audio_sounds_js: JsValue,
}

pub fn provide_audio_context() {
    leptos::provide_context(Audio3Context{audio_sounds_js: init_audio_js()});
}

pub fn play_sound(audio_key: &str) {
    let audio_key = audio_key.to_string();
    let context : Audio3Context= leptos::expect_context();
    // queue_microtask(move || {
        play_sound_js(context.audio_sounds_js, audio_key);
    // });
}
pub fn stop_sound(audio_key: &str) {
    let audio_key = audio_key.to_string();
    let context : Audio3Context= leptos::expect_context();
    // queue_microtask(move || {
        stop_sound_js(context.audio_sounds_js, audio_key);
    // });
}