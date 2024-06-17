use game::api::websocket::{APIMethod, WebsocketAPIMessageRaw, WebsocketAPIMessageType, WhoAmI};
use leptos::*;
use leptos_use::core::ConnectionReadyState;
use leptos_use::{use_websocket, UseWebsocketReturn};

// async fn await_reply_message(msg_type: WebsocketAPIMessageType, msg_id: u32) -> WebsocketAPIMessageRaw {
//     let (tx, rx) = futures::channel::oneshot::channel::<WebsocketAPIMessageRaw>();

// }

// async fn who_am_i() ->  <game::api::websocket::WhoAmI as APIMethod>::Resp {

// }
use std::rc::Rc;

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct WsMessageKey(u32, WebsocketAPIMessageType);
#[derive(Clone)]
struct WsMessageCell(Rc<futures::channel::oneshot::Sender<WebsocketAPIMessageRaw>>);
#[derive(Clone)]
pub struct WebsocketAPI {
    map: RwSignal<std::collections::HashMap<WsMessageKey, WsMessageCell>>,
    sender: RwSignal<Rc<Box<dyn Fn(Vec<u8>)>>>,
}

pub fn call_websocket_api<T: APIMethod>(
    api: WebsocketAPI,
    arg: T::Req,
) -> anyhow::Result<impl std::future::Future<Output = Result<T::Resp, String>>> {
    use rand::Rng;
    let id = (&mut rand::thread_rng()).gen();
    let (tx, rx) = futures::channel::oneshot::channel::<WebsocketAPIMessageRaw>();

    let map_key = WsMessageKey(id, T::TYPE);
    api.map.update_untracked(|map| {
        map.insert(map_key.clone(), WsMessageCell(tx.into()));
    });

    let sender = api.sender.get_untracked();
    T::send(arg, move |x| sender(x), id)?;

    // let api = api.clone();
    Ok(async move {
        match rx.await {
            Ok(val) => match bincode::deserialize::<Result<T::Resp, String>>(&val.data) {
                Ok(val) => val,
                Err(e) => Err(format!("err websocket response deserialize: {:?}", e)),
            },
            Err(e) => Err(format!("err waiting on websocket oneshjot: {:?}", e)),
        }
    })
}

async fn accept_reply_message(api: &WebsocketAPI, msg: WebsocketAPIMessageRaw) {
    let key = WsMessageKey(msg.id, msg._type);

    log::info!("accepting websocket reploy for {:?}", &key);
    api.map.update_untracked(|map| {
        if let Some(cell) = map.remove(&key) {
            if let Ok(cell) = Rc::try_unwrap(cell.0) {
                if let Err(e) = cell.send(msg) {
                    log::warn!("failed to send message into oneshot: {:?}", e._type);
                }
            } else {
                log::warn!("failed to unwrap Rc that we just removed from map!");
            }
        } else {
            log::warn!("got message with key={:?} but not found in map!", key);
        }
    })
}

#[component]
pub fn WebsocketDemo2() -> impl IntoView {
    let UseWebsocketReturn {
        ready_state,
        // message,
        message_bytes,
        // send,
        send_bytes,
        open,
        close,
        ..
    } = use_websocket("ws://localhost:3000/api/ws");

    // let message =
    // let message = bincode::serialize(&message).unwrap();
    let api = WebsocketAPI {
        map: create_rw_signal(std::collections::HashMap::<_, _>::new()),
        sender: create_rw_signal(Rc::new(Box::new(send_bytes.clone()))),
    };
    provide_context(api.clone());

    let api2 = api.clone();
    let send_byte_message = move |_| {
        let api2 = api2.clone();
        let _res = create_resource(
            || (),
            move |_| {
                let api2 = api2.clone();
                async move {
                    log::info!("calling websocket api");
                    let r = call_websocket_api::<WhoAmI>(api2, ())
                        .expect("cannot obtain future")
                        .await;
                    log::info!("got back response: {:?}", r);
                    r
                }
            },
        );
    };
    let mut recv_bytes_stream = message_bytes.to_stream();

    log::info!("console init");
    let api_spawn = api.clone();
    spawn_local(async move {
        log::info!("spawn local init");
        use futures::stream::StreamExt;
        loop {
            while let Some(Some(c)) = recv_bytes_stream.next().await {
                match bincode::deserialize::<WebsocketAPIMessageRaw>(&c) {
                    Ok(msg) => {
                        log::info!("recv message type={:?} len={}", msg._type, c.len(),);
                        accept_reply_message(&api_spawn.clone(), msg).await;
                        // let ctx = expect_context::<RwSignal<WebsocketAPI>>();
                        // log::info!("successfully got global context size={}!", ctx.get_untracked().map.len());
                    }
                    Err(e) => {
                        log::warn!("websocket deserialize error {:?}", e);
                    }
                }
            }
            log::info!("websocket reciever died.");
            // thread::sleep(std::time::Duration::from_millis(3));
        }
    });

    let status = move || {
        let st = ready_state.get().to_string();
        log::info!("websocket status: {}", st);
        st
    };

    let connected = move || ready_state.get() == ConnectionReadyState::Open;

    let open_connection = move |_| {
        log::info!("websocket reopened.");
        open();
    };

    let close_connection = move |_| {
        log::info!("websocket closed intentionally.");
        close();
    };

    view! {
        <div>
            <p>"status: " {status}</p>

            <button on:click=send_byte_message disabled=move || !connected()>
                "Send bytes"
            </button>
            <button on:click=open_connection disabled=connected>
                "Open"
            </button>
            <button on:click=close_connection disabled=move || !connected()>
                "Close"
            </button>

            // <p>{sig}</p>
            <p>"Receive byte message: " {move || format!("{:?}", message_bytes.get())}</p>
        </div>
    }
}
