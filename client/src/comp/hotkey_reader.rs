use std::collections::{hash_map, HashMap};

use game::tet::TetAction;
use leptos::*;

use crate::hotkey_context::HotkeysContext;

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

    create_effect(move |_| {
        events.with(|events| {
            for event in events {
                match event {
                    crate::hotkey_context::KeyPressEvent::KeyDown(key_id) => {
                        if let Some(tet_action) = control_mapping.get(key_id) {
                            on_action.call(*tet_action);
                            if tet_action.is_repeating() {
                                // magic
                            }
                        }
                    },
                    crate::hotkey_context::KeyPressEvent::KeyUp(key_id) => {
                        if let Some(tet_action) = control_mapping.get(key_id) {
                            if tet_action.is_repeating() {
                                // more magic
                            }
                        }
                    },
                }
            }
        })
    });

    

}
