
use leptos::*;

fn main() {
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default_with_config(
        tracing_wasm::WASMLayerConfigBuilder::default()
            .set_max_level(tracing::Level::DEBUG)
            .build(),
    );
    console_log::init_with_level(log::Level::Debug).expect("cannot register log");
    
    // TODO P2P
    // wasm_bindgen_futures::spawn_local(demo_matchbox::async_main());

    mount_to_body(|| {
        view! { <client::app_root::AppRoot></client::app_root::AppRoot> }
    });
}

#[cfg(test)]
pub mod tests {
    use wasm_bindgen_test::*;

    #[test]
    #[wasm_bindgen_test]
    pub fn test_2_plus_2() {
        assert_eq!(2 + 2, 4)
    }
}
