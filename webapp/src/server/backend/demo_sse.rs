use {
    crate::game::{
        tet::{GameState, TetAction},
        timestamp::get_timestamp_now,
    },
    axum::response::sse::{Event, KeepAlive, Sse},
    futures::stream::Stream,
};

pub async fn handle_sse_game_stream() -> Sse<impl Stream<Item = Result<Event, axum::BoxError>>> {
    use futures::stream;
    use leptos_sse::ServerSentEvents;
    use std::time::Duration;
    use tokio_stream::StreamExt as _;

    let seed = [0; 32];
    let mut state1 = GameState::new(&seed, get_timestamp_now());

    let stream = ServerSentEvents::new(
        "game_replay",
        stream::repeat_with(move || {
            let action = TetAction::random();
            let t2 = get_timestamp_now();
            let _ = state1.apply_action_if_works(action, t2);

            if state1.game_over {
                state1 = GameState::new(&seed, get_timestamp_now());
            }
            Ok(state1.replay.clone())
        })
        .throttle(Duration::from_secs(1)),
    )
    .unwrap();
    Sse::new(stream).keep_alive(KeepAlive::default())
}
