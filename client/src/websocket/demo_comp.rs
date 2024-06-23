use game::api::websocket::{
    APIMethod, WebsocketAPIMessageRaw, WebsocketAPIMessageType,
};
use leptos::*;
use leptos_use::core::ConnectionReadyState;

// async fn await_reply_message(msg_type: WebsocketAPIMessageType, msg_id: u32) -> WebsocketAPIMessageRaw {
//     let (tx, rx) = futures::channel::oneshot::channel::<WebsocketAPIMessageRaw>();

// }

// async fn who_am_i() ->  <game::api::websocket::WhoAmI as APIMethod>::Resp {

// }
use std::rc::Rc;

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct WsMessageKey(u32, WebsocketAPIMessageType);
#[derive(Clone)]
pub struct WsMessageCell(Rc<futures::channel::oneshot::Sender<WebsocketAPIMessageRaw>>);

#[derive(Clone)]
pub struct WebsocketAPI {
    pub map: RwSignal<std::collections::HashMap<WsMessageKey, WsMessageCell>>,
    pub sender: RwSignal<Rc<Box<dyn Fn(Vec<u8>)>>>,
    pub ready_state_stream: async_broadcast::InactiveReceiver<ConnectionReadyState>,
    pub ready_signal: RwSignal<bool>,
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
    let ready_signal = api.ready_signal.clone();
    Ok(async move {
        if !ready_signal.get_untracked() {
            log::info!("waiting for ready state");

            loop {
                let mut stream = api.ready_state_stream.activate_cloned();
                if let Ok(current_state) = stream.recv().await {
                    match current_state {
                        ConnectionReadyState::Connecting => continue,
                        ConnectionReadyState::Open => break,
                        ConnectionReadyState::Closing => continue,
                        ConnectionReadyState::Closed => continue,
                    }
                }
                if ready_signal.get_untracked() {
                    break;
                }
            }
        }
        log::info!("Websocket Request: {:?}", T::TYPE);
        T::send(arg, move |x| sender(x), id)
            .map_err(|e| format!("send error: {:?}", e))?;

        match rx.await {
            Ok(val) => match bincode::deserialize::<Result<T::Resp, String>>(&val.data)
            {
                Ok(val) => val,
                Err(e) => Err(format!("err websocket response deserialize: {:?}", e)),
            },
            Err(e) => Err(format!("err waiting on websocket oneshjot: {:?}", e)),
        }
    })
}



pub async fn accept_subscribe_notification(_api: &WebsocketAPI, msg: WebsocketAPIMessageRaw) -> anyhow::Result<()> {
    match &msg._type {
        WebsocketAPIMessageType::SubscribedGameUpdateNotification => {
            let data  = bincode::deserialize::<<game::api::websocket::SubscribedGameUpdateNotification as game::api::websocket::APIMethod>::Req>(&msg.data)?;
            log::info!("GOT SUBSCRIBE MESSAGE FOR {} segments", data.len());
        },
        _x => {
            anyhow::bail!("unsupported message type for subscribe nmmotification:L {:?}", msg._type);
        }
    }
    Ok(())
}

pub async fn accept_reply_message(api: &WebsocketAPI, msg: WebsocketAPIMessageRaw) {
    let key = WsMessageKey(msg.id, msg._type);

    // log::info!("accepting websocket reploy for {:?}", &key);
    api.map.update_untracked(|map| {
        if let Some(cell) = map.remove(&key) {
            if let Ok(cell) = Rc::try_unwrap(cell.0) {
                if let Err(e) = cell.send(msg) {
                    log::warn!("failed to send message into oneshot: {:?}", e._type);
                } else {
                    log::info!("Websocket: {:?} SUCCESS", key.1);
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
pub fn WebsocketDemo2() -> impl IntoView {}
