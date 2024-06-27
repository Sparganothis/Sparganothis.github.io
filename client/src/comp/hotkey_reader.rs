use std::collections::{hash_map, HashMap};

use game::tet::TetAction;
use leptos::*;
use leptos_use::{use_interval, use_interval_with_options, UseIntervalOptions, UseIntervalReturn};

use crate::hotkey_context::HotkeysContext;


const ARR_MS: u64= 33;
const DAS_MS: u64 = 167;


#[component]
pub fn HotkeyReader(#[prop(into)] on_action: Callback<TetAction>) -> impl IntoView {
    let mut control_mapping = HashMap::<String, TetAction>::new();
    control_mapping.insert("arrowup".to_string(),TetAction::RotateRight );
    control_mapping.insert("keyx".to_string(),TetAction::RotateRight );
    control_mapping.insert("controlleft".to_string(),TetAction::RotateRight );
    control_mapping.insert("controlright".to_string(),TetAction::RotateRight );

    control_mapping.insert("arrowdown".to_string(),TetAction::SoftDrop );
    control_mapping.insert("space".to_string(),TetAction::HardDrop );
    control_mapping.insert("keyz".to_string(),TetAction::RotateLeft );
    control_mapping.insert("arrowleft".to_string(),TetAction::MoveLeft );
    control_mapping.insert("arrowright".to_string(),TetAction::MoveRight );

    control_mapping.insert("keyc".to_string(),TetAction::Hold );
    control_mapping.insert("shiftleft".to_string(),TetAction::Hold );
    control_mapping.insert("shiftright".to_string(),TetAction::Hold );

    let hotkey_context = expect_context::<HotkeysContext>();
    let events = hotkey_context.key_events;


    let repeat_event = create_rw_signal(None);
    let UseIntervalReturn{ 
        counter: das_counter, reset: das_reset, is_active: das_is_active, pause: das_pause, resume: das_resume }    =  use_interval_with_options( DAS_MS , UseIntervalOptions::default().immediate(false));
    let UseIntervalReturn{ counter: arr_counter, reset: arr_reset, is_active: arr_is_active, pause: arr_pause, resume: arr_resume }    = use_interval_with_options(ARR_MS,UseIntervalOptions::default().immediate(false));

    // when das counter changes, reset and pause it and ccall fisrt repeat acction. Also start repeat arr counter
    let das_reset2 = das_reset.clone();
    let das_pause2 = das_pause.clone();
    let arr_resume2 = arr_resume.clone();
    create_effect(move |_| {
        let counter = das_counter.get();
        if counter > 0 {
            das_reset2();
            das_pause2();
            if let Some(repeat_event) =  repeat_event.get_untracked() {
                on_action.call(repeat_event);
                arr_resume2();
            }
        }
    });

    // every timem ARR hits, send ccallback again
    let das_pause2 = das_pause .clone();
    let das_reset2 = das_reset .clone();
    let arr_pause2 = arr_pause .clone();
    let arr_reset2 = arr_reset .clone();
    create_effect(move |_|{
        let counter = arr_counter.get();
        if counter > 0 { 
            if let Some(repeat_event) =  repeat_event.get_untracked() {
                on_action.call(repeat_event);
            }
        }
        if counter > 20 {
            repeat_event.set(None);
            das_pause2();
            das_reset2();
            arr_pause2();
            arr_reset2();
        }
    });


    let das_pause2 = das_pause .clone();
    let das_reset2 = das_reset .clone();
    let arr_pause2 = arr_pause .clone();
    let arr_reset2 = arr_reset .clone();
    let das_resume2 = das_resume.clone();
    create_effect(move |_| {
        events.with(|events| {
            for event in events {
                match event {
                    crate::hotkey_context::KeyPressEvent::KeyDown(key_id) => {
                        if let Some(tet_action) = control_mapping.get(key_id) {
                            on_action.call(*tet_action);
                            if tet_action.is_repeating() {
                                // magic
                                repeat_event.set(Some(*tet_action));
                                das_resume2();
                            }
                        }
                    },
                    crate::hotkey_context::KeyPressEvent::KeyUp(key_id) => {
                        if let Some(tet_action) = control_mapping.get(key_id) {
                            if tet_action.is_repeating() {
                                // more magic
                                repeat_event.set(None);
                                das_pause2();
                                das_reset2();
                                arr_pause2();
                                arr_reset2();
                            }
                        }
                    },
                }
            }
        })
    });
}
