// #![deny(unused_crate_dependencies)]

use leptos::*;

pub mod app_root;
mod error_template;
pub mod style;
// pub mod comp;
// pub mod page;
pub mod websocket;

fn main() {
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default_with_config(
        tracing_wasm::WASMLayerConfigBuilder::default()
            .set_max_level(tracing::Level::DEBUG)
            .build(),
    );
    console_log::init_with_level(log::Level::Debug).expect("cannot register log");
    mount_to_body(|| {
        view! { <app_root::AppRoot/> }
    });
}
