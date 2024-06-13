use leptos::*;

#[component]
pub fn WebsocketDemo() -> impl IntoView {
    log::info!("init");
    let leptos_use::UseWebsocketReturn {
        ready_state,
        message,
        message_bytes,
        send,
        send_bytes,
        open,
        close,
        ..
    } = leptos_use::use_websocket("/api/ws");

    log::info!("instanced");
    
let send_message = move |_| {
    send("Hello, world!");
};

let send_byte_message = move |_| {
    send_bytes(b"Hello, world!\r\n".to_vec());
};

let status = move || ready_state.get().to_string();

let connected = move || ready_state.get() == leptos_use::core::ConnectionReadyState::Open;

let open_connection = move |_| {
    open();
};

let close_connection = move |_| {
    close();
};

view! {
    <div>
        <p>"status: " {status}</p>

        <button on:click=send_message disabled=move || !connected()>
            "Send"
        </button>
        <button on:click=send_byte_message disabled=move || !connected()>
            "Send bytes"
        </button>
        <button on:click=open_connection disabled=connected>
            "Open"
        </button>
        <button on:click=close_connection disabled=move || !connected()>
            "Close"
        </button>

        <p>"Receive message: " {move || format!("{:?}", message.get())}</p>
        <p>"Receive byte message: " {move || format!("{:?}", message_bytes.get())}</p>
    </div>
}
}
