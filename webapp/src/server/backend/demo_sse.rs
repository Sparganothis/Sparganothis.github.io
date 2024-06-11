use axum::{
    response::sse::{Event, Sse},
    routing::get,
    Router,
};
use axum_extra::TypedHeader;
use futures::stream::{self, Stream};
use std::{convert::Infallible, path::PathBuf, time::Duration};
use tokio_stream::StreamExt as _;
use tower_http::{services::ServeDir};
use axum_extra::headers;
// #[get("/api/events")]
// async fn counter_events() -> impl Responder {
//     use crate::counters::ssr_imports::*;
//     use futures::StreamExt;

//     let stream = futures::stream::once(async {
//         crate::counters::get_server_count().await.unwrap_or(0)
//     })
//     .chain(COUNT_CHANNEL.clone())
//     .map(|value| {
//         Ok(web::Bytes::from(format!(
//             "event: message\ndata: {value}\n\n"
//         ))) as Result<web::Bytes>
//     });
//     HttpResponse::Ok()
//         .insert_header(("Content-Type", "text/event-stream"))
//         .streaming(stream)
// }
use crate::game::tet::*;
use crate::game::timestamp::get_timestamp_now;

pub async fn handle_sse_game_stream(
    TypedHeader(user_agent): TypedHeader<headers::UserAgent>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    println!("`{}` connected", user_agent.as_str());

    let seed = [0; 32];
    let mut state1 = GameState::new(&seed, get_timestamp_now());
    let stream = stream::repeat_with(move || {
                    let action = TetAction::random();
                    let t2 = get_timestamp_now();
                    let _ = state1.apply_action_if_works(action, t2);
        
                    if state1.game_over {
                        state1 = GameState::new(&seed, get_timestamp_now());
                    }
                    Event::default().json_data(state1.replay.clone()).unwrap()
                })
        .map(Ok)
        .throttle(Duration::from_secs(1));

    // let stream = stream::repeat_with(|| {
    //     Event::default().json_data("hi!").unwrap()
    // })
    // .map(Ok)
    // .throttle(Duration::from_secs(1));

    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(1))
            .text("keep-alive-text"),
    )
}


// pub async fn handle_sse_game_stream() -> Sse<impl Stream<Item = Result<Event, axum::BoxError>>> {
//     use futures::stream;
//     use leptos_sse::ServerSentEvents;
//     use std::time::Duration;
//     use tokio_stream::StreamExt as _;


//     let stream = ServerSentEvents::new(
//         "game_replay",
//         stream::repeat_with()
//         .throttle(Duration::from_secs(1)),
//     )
//     .unwrap();
//     Sse::new(stream).keep_alive(KeepAlive::default())
// }
