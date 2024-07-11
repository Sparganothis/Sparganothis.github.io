use game::{api::{game_replay::{GameId, GameSegmentId}, websocket::{
    APIMethod, SubscribeGamePlz, SubscribeGamePlzArgument, WebsocketAPIMessageRaw, WebsocketAPIMessageType
}}, tet::GameReplaySegment};
use leptos::*;
use leptos_use::core::ConnectionReadyState;

// async fn await_reply_message(msg_type: WebsocketAPIMessageType, msg_id: u32) -> WebsocketAPIMessageRaw {
//     let (tx, rx) = futures::channel::oneshot::channel::<WebsocketAPIMessageRaw>();

// }

// async fn who_am_i() ->  <game::api::websocket::WhoAmI as APIMethod>::Resp {

// }
use std::{collections::HashMap, rc::Rc};

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

    pub subscribe_game_callbacks: RwSignal<HashMap<GameId, SubscribeSegmentCallback>>,
    pub error_msgs: RwSignal<Vec<String>>,
}

type SubscribeSegmentCallback = Callback<Vec<(GameSegmentId, GameReplaySegment)>>;

impl WebsocketAPI {
    pub fn subscribe_to_game(&self, game_id: &GameId, cb: SubscribeSegmentCallback) {
        self.subscribe_game_callbacks.update_untracked(|map| {
            log::warn!("START SUBSCRIBE TO GAME: {:?}", game_id);
            map.insert(*game_id, cb);
            let game_id = *game_id;
            let arg = SubscribeGamePlzArgument { game_id, command: game::api::websocket::SubscribeGamePlzCommmand::StartStreaming };
            call_api_sync::<SubscribeGamePlz>(arg, move |_x| {
                log::warn!("subscribe OK");
            });
        });
    }

   pub fn stop_subscribe_to_game(&self, game_id: &GameId) {
    self.subscribe_game_callbacks.update_untracked(|map| {
        log::warn!("STOP SUBSCRIBE TO GAME: {:?}", game_id);
        map.remove(game_id);
        let game_id = *game_id;
        let arg = SubscribeGamePlzArgument { game_id, command: game::api::websocket::SubscribeGamePlzCommmand::StopStreaming };
        call_api_sync::<SubscribeGamePlz>(arg, move |_x| {
            log::warn!("unsubscribe OK");
        });
    })
   }

   pub fn stop_all_subscribe_to_game(&self) {
        self.subscribe_game_callbacks.update_untracked(|map| {
            for item in map.keys() {
                self.stop_subscribe_to_game(item);
            }
        })
   } 
}


pub fn call_api_sync<T: APIMethod>(arg: T::Req, f: impl Fn(T::Resp) + Clone+'static) -> () {
    let api2: WebsocketAPI = expect_context();
    call_api_sync_or_error::<T>(arg, f, move |err|{api2.error_msgs.update(|x| x.push(err.clone()));})
}

pub fn call_api_sync_or_error<T: APIMethod>(arg: T::Req, f: impl Fn(T::Resp) + Clone+'static, ferr: impl Fn(String) + Clone+'static) -> () {
    let api2: WebsocketAPI = expect_context();
    let res = create_resource(|| (), move |_| {
        let api2 = api2.clone();
        let arg2 = arg.clone();
        async move {
            _call_websocket_api::<T>(api2, arg2)
            .expect("cannot obtain future")
            .await
        }
    });

    create_effect(move |_| {
        match res.get() {
            Some(Ok(res)) => {
                f(res);
            },
            Some(Err(err)) => {
                ferr(err);
            },
            None => return,
        }
    });
}

pub fn _call_websocket_api<T: APIMethod>(
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
        // log::info!("Websocket Request: {:?}", T::TYPE);
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

            let first = (&data).first().cloned();
            if let Some(first) = first.clone() {
                _api.subscribe_game_callbacks.with_untracked(move |map| {
                    if let  Some(cb) = map.get(&first.0.game_id) {
                        cb.call(data.clone());
                    }
            })};
            
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
                    // log::info!("Websocket: {:?} SUCCESS", key.1);
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
