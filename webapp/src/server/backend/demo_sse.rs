use axum::{
    response::sse::{Event, Sse},
};
use axum_extra::headers;
use axum_extra::TypedHeader;
use futures::stream::{self, Stream};
use std::{convert::Infallible,time::Duration};
use tokio_stream::StreamExt as _;

use crate::game::tet::*;
use crate::game::timestamp::get_timestamp_now;

pub async fn handle_sse_game_stream(
    TypedHeader(user_agent): TypedHeader<headers::UserAgent>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    println!("`{}` connected", user_agent.as_str());

    let mut maybe_state: Option<GameState> = None;
    let stream = stream::repeat_with(move || {

        let segment = if let  Some(mut state)  = maybe_state.clone() {
            let  old_slice_no = state.replay.replay_slices.len();
            while  state.replay.replay_slices.len() == old_slice_no {
                let action = TetAction::random();
                let t2 = get_timestamp_now();
                let _ = state.apply_action_if_works(action, t2);
            }
            let new_slice_no = state.replay.replay_slices.len();
            maybe_state = Some(state);
            assert!(new_slice_no == old_slice_no + 1);

            GameReplaySegment::Update(maybe_state.as_ref().unwrap().replay.replay_slices.last().unwrap().clone())

        } else  {
            let seed: [u8; 32] = [0; 32];
            maybe_state = Some(GameState::new(&seed, get_timestamp_now()));
            GameReplaySegment::Init(maybe_state.as_ref().unwrap().replay.clone())
        };
   
        let str = serde_json::to_string(&segment).unwrap();
        Event::default().data(str)
    })
    .map(Ok)
    .throttle(Duration::from_secs(1));

    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(1))
            .text("keep-alive-text"),
    )
}
