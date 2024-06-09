pub mod error_template;
pub mod errors;
#[cfg(feature = "ssr")]
pub mod fallback;
#[cfg(feature = "csr")]
pub mod client;
pub mod server;
pub mod game;


#[cfg(feature="csr")]
#[cfg_attr(feature = "csr", wasm_bindgen::prelude::wasm_bindgen)]
pub fn hydrate() {
    // use crate::todo::*;

    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    leptos::mount_to_body(crate::client::app_root::AppRoot);
}

#[cfg(not(feature="csr"))]
pub fn hydrate() {
}
