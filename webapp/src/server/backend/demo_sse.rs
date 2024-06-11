use {
    axum::response::sse::{Event, KeepAlive, Sse},
    futures::stream::Stream,
};
use serde::{Serialize,Deserialize};

use crate::server::api::Count;


pub async fn handle_sse() -> Sse<impl Stream<Item = Result<Event, axum::BoxError>>> {
    use futures::stream;
    use leptos_sse::ServerSentEvents;
    use std::time::Duration;
    use tokio_stream::StreamExt as _;

    let mut value = 0;
    let stream = ServerSentEvents::new(
        "counter",
        stream::repeat_with(move || {
            let curr = value;
            value += 1;
            Ok(Count { value: curr })
        })
        .throttle(Duration::from_secs(1)),
    )
    .unwrap();
    Sse::new(stream).keep_alive(KeepAlive::default())
}