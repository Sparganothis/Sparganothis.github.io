use std::collections::HashMap;

use game::tet::TetAction;
use leptos::*;
use leptos_use::{ use_interval_with_options, UseIntervalOptions, UseIntervalReturn};

use crate::hotkey_context::HotkeysContext;


const ARR_MS: u64= 33;
const DAS_MS: u64 = 167;

pub struct HotkeyRepeaterFunctions<StartFn, StopFn>
where
    StartFn: Fn() + Clone,
    StopFn: Fn() + Clone,
{
    pub start: StartFn,
    pub stop: StopFn,
}

fn create_hotkey_repeater(action: TetAction, on_action: impl Fn(TetAction) + Clone + 'static) -> HotkeyRepeaterFunctions<impl Fn() + Clone, impl Fn() + Clone> {

    let UseIntervalReturn{ 
        counter: das_counter, reset: das_reset, is_active: das_is_active, pause: das_pause, resume: das_resume }    =  use_interval_with_options( DAS_MS , UseIntervalOptions::default().immediate(false));
    let UseIntervalReturn{ counter: arr_counter, reset: arr_reset, is_active: arr_is_active, pause: arr_pause, resume: arr_resume }    = use_interval_with_options(ARR_MS,UseIntervalOptions::default().immediate(false));

    // when das counter changes, reset and pause it and ccall fisrt repeat acction. Also start repeat arr counter
    let das_reset2 = das_reset.clone();
    let das_pause2 = das_pause.clone();
    let arr_resume2 = arr_resume.clone();
    let on_action2 = on_action.clone();
    create_effect(move |_| {
        let counter = das_counter.get();
        if counter > 0 {
            das_reset2();
            das_pause2();
            on_action2(action);
            arr_resume2();
        }
    });

    // every timem ARR hits, send ccallback again
    let das_pause2 = das_pause .clone();
    let das_reset2 = das_reset .clone();
    let arr_pause2 = arr_pause .clone();
    let arr_reset2 = arr_reset .clone();
    let on_action2 = on_action.clone();
    create_effect(move |_|{
        let counter = arr_counter.get();
        if counter > 0 { 
            on_action2(action);
        }
        if counter > 18461 { // world record of softdrop repeated for 5min
            das_pause2();
            das_reset2();
            arr_pause2();
            arr_reset2();
        }
    });
    
    let das_resume2 = das_resume.clone();
    let on_start = move || {
        das_resume2();
    };

    let das_pause2 = das_pause .clone();
    let das_reset2 = das_reset .clone();
    let arr_pause2 = arr_pause .clone();
    let arr_reset2 = arr_reset .clone();
    let on_stop = move || {
        das_pause2();
        das_reset2();
        arr_pause2();
        arr_reset2();
    };

    HotkeyRepeaterFunctions{
        start: on_start,
        stop: on_stop,
    }
}


pub fn create_hotkey_reader(on_action: impl Fn(TetAction) + Clone+'static) {
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

    let HotkeyRepeaterFunctions {
        start: start_left,
        stop: stop_left,
    } = create_hotkey_repeater(TetAction::MoveLeft, on_action.clone());
    
    let HotkeyRepeaterFunctions {
        start: start_right,
        stop: stop_right,
    } = create_hotkey_repeater(TetAction::MoveRight, on_action.clone());

    let HotkeyRepeaterFunctions {
        start: start_softdrop,
        stop: stop_softdrop,
    } = create_hotkey_repeater(TetAction::SoftDrop, on_action.clone());

    let start_left2 = start_left.clone();
    let start_right2 = start_right.clone();
    let start_softdrop2 = start_softdrop.clone();
    let stop_left2 =     stop_left.clone();
    let stop_right2 =    stop_right.clone();
    let stop_softdrop2 = stop_softdrop.clone();

    let last_events_sig = create_rw_signal(vec![]);

    create_effect(move |_| {
        let current_events = events.get();
        let last_events = last_events_sig.get_untracked();
        if last_events == current_events {
            return;
        }
        last_events_sig.set_untracked(current_events);
        events.with(|events| {
            log::info!("NEW EVENTS LISTING FOR....");
            for event in events {
                
                log::info!("REACTING TO NEW EVENT: {:?}", event);
                match event {
                    crate::hotkey_context::KeyPressEvent::KeyDown(key_id) => {
                        if let Some(tet_action) = control_mapping.get(key_id) {
                            on_action(*tet_action);
                            if tet_action.is_repeating() {
                                // magic
                                match tet_action {
                                    TetAction::SoftDrop => start_softdrop2(),
                                    TetAction::MoveLeft => start_left2(),
                                    TetAction::MoveRight => start_right2(),
                                    _ => {},
                                }
                            }
                        }
                    },
                    crate::hotkey_context::KeyPressEvent::KeyUp(key_id) => {
                        if let Some(tet_action) = control_mapping.get(key_id) {
                            if tet_action.is_repeating() {
                                // more magic   
                                match tet_action {
                                    TetAction::SoftDrop => stop_softdrop2(),
                                    TetAction::MoveLeft => stop_left2(),
                                    TetAction::MoveRight => stop_right2(),
                                    _ => {},
                                }
                            }
                        }
                    },
                }
            }
        })
    });
}
