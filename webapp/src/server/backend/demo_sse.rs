use axum::
    response::sse::{Event, Sse}
;
use axum_extra::headers;
use axum_extra::TypedHeader;
use futures::stream::{self, Stream};
use std::{convert::Infallible,time::Duration};
use tokio_stream::StreamExt as _;

use crate::game::tet::*;
use crate::game::timestamp::get_timestamp_now;
use std::collections::VecDeque;


pub async fn handle_sse_game_stream(
    TypedHeader(user_agent): TypedHeader<headers::UserAgent>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    println!("`{}` connected", user_agent.as_str());

    let mut maybe_state: Option<GameState> = None;
    let mut new_segments = VecDeque::<GameReplaySegment>::new();
    let stream = stream::repeat_with(move || {
        if !new_segments.is_empty() {
            new_segments.pop_front().unwrap()
        } else {
            if let  Some(mut state)  = maybe_state.clone() {
                let  old_slice_no = state.replay.replay_slices.len();
                while  state.replay.replay_slices.len() == old_slice_no {
                    let action = TetAction::random();
                    let t2 = get_timestamp_now();
                    let _ = state.apply_action_if_works(action, t2);
                }
                let new_slice_no = state.replay.replay_slices.len();
                maybe_state = Some(state);
                
                for i in (old_slice_no+1)..new_slice_no  {
                    let new_state = maybe_state.as_ref().unwrap().replay.replay_slices[i].clone();
                    new_segments.push_back(GameReplaySegment::Update(new_state));
                }
    
                GameReplaySegment::Update(maybe_state.as_ref().unwrap().replay.replay_slices[old_slice_no].clone())
    
            } else  {
                let seed: [u8; 32] = [0; 32];
                maybe_state = Some(GameState::new(&seed, get_timestamp_now()));
                GameReplaySegment::Init(maybe_state.as_ref().unwrap().replay.clone())
            }
        }

    }).take_while(|v| {
        !v.is_game_over()
    })
    .map(|segment| {   
        let str = serde_json::to_string(&segment).unwrap();
        Event::default().data(str)
    })
    .map(Ok)
    .throttle(Duration::from_millis(100));

    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(1))
            .text("keep-alive-text"),
    )
}
