
use leptos::*;
use leptos_meta::*;
use leptos_router::*;


#[cfg(feature = "ssr")]
pub mod ssr_imports {
    pub use broadcaster::BroadcastChannel;
    pub use once_cell::sync::OnceCell;
    pub use std::sync::atomic::{AtomicI32, Ordering};

    pub static COUNT: AtomicI32 = AtomicI32::new(0);

    lazy_static::lazy_static! {
        pub static ref COUNT_CHANNEL: BroadcastChannel<i32> = BroadcastChannel::new();
    }

    static LOG_INIT: OnceCell<()> = OnceCell::new();

    pub fn init_logging() {
        LOG_INIT.get_or_init(|| {
            simple_logger::SimpleLogger::new().env().init().unwrap();
        });
    }
}


#[server]
pub async fn adjust_server_count(
    delta: i32,
    msg: String,
) -> Result<i32, ServerFnError> {
    use ssr_imports::*;

    let new = COUNT.load(Ordering::Relaxed) + delta;
    COUNT.store(new, Ordering::Relaxed);
    _ = COUNT_CHANNEL.send(&new).await;
    println!("message = {:?}", msg);
    Ok(new)
}
use crate::server::api::Count;
use leptos::*;
use leptos_sse::create_sse_signal;
use serde::{Deserialize, Serialize};

#[component]
pub fn SseDeom() -> impl IntoView {
    leptos_sse::provide_sse("/api/events").unwrap();

    // Create sse signal
    let count = create_sse_signal::<Count>("counter");

    view! {
        <h1>"Count: " {move || count.get().value.to_string()}</h1>
    }
}