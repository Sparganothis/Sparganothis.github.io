use wasm_bindgen::prelude::*;

// create rust functions from the javascript functions
#[wasm_bindgen(module = "/public/js/mobile_check.js")]
extern "C" {
    pub fn detect_mobile_js() -> JsValue;
}

pub fn is_mobile_phone()->bool{
    let v = detect_mobile_js();
    log::info!("{v:?}");
    v.as_bool().expect("Mobil Check JS")
}