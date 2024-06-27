use game::timestamp::get_timestamp_now_ms;
use leptos::html::ElementDescriptor;
use leptos::*;
use std::collections::HashSet;
use wasm_bindgen::JsCast;

#[derive(Clone, Copy)]
pub struct HotkeysContext {
    pub key_events: RwSignal<Vec<KeyPressEvent>>,
}

#[derive(Clone, Debug)]
pub enum KeyPressEvent {
    KeyDown(String),
    KeyUp(String)
}

pub fn provide_hotkeys_context2<T>(
    node_ref: NodeRef<T>,
) -> HotkeysContext
where
    T: ElementDescriptor + 'static + Clone,
{
    let raw_pressed_keys: RwSignal<std::collections::HashMap<String, web_sys::KeyboardEvent>> =
        RwSignal::new(std::collections::HashMap::new());

    let dedup_pressed_keys :  RwSignal<std::collections::HashMap<String, web_sys::KeyboardEvent>> =
    RwSignal::new(std::collections::HashMap::new());

    let last_pressed_keys = create_rw_signal(vec![]);
    create_effect(move |_| {
        let v_pressed_keys = raw_pressed_keys.get().keys().cloned().collect::<Vec<String>>();
        if v_pressed_keys != last_pressed_keys.get_untracked() {
            last_pressed_keys.set_untracked(v_pressed_keys);
            dedup_pressed_keys.set(raw_pressed_keys.get());
        }
    });

    let _last_pressed_time = create_rw_signal(get_timestamp_now_ms());

    let last_dedup_keys : RwSignal<std::collections::HashMap<String, web_sys::KeyboardEvent>> =
    RwSignal::new(std::collections::HashMap::new());
    

    let events_signal : RwSignal<Vec<KeyPressEvent>> = create_rw_signal(vec![]);
    create_effect(move|_|{

        let old_keys  = last_dedup_keys.get_untracked(); 
        let new_keys  = dedup_pressed_keys.get();

        let mut v = vec![];
        for key in old_keys.keys() {
            if !new_keys.contains_key(key) {
                // remomve key (key up )
                v.push(KeyPressEvent::KeyUp(key.clone()));
            }
        }
        for key in new_keys.keys() {
           if !old_keys.contains_key(key) {
            // down key (add key)
                v.push(KeyPressEvent::KeyDown(key.clone()));
           } 
        }
        events_signal.set(v.clone());

        log::info!("keys: {:?} time={:?}ms", v, get_timestamp_now_ms() - _last_pressed_time.get_untracked());
        _last_pressed_time.set_untracked(get_timestamp_now_ms());
        last_dedup_keys.set_untracked(new_keys);
    });

    node_ref.on_load(move |_| {
        let keydown_listener =
            wasm_bindgen::closure::Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
                raw_pressed_keys.update(|keys| {
                    keys.insert(event.code().to_lowercase(), event);
                });
            }) as Box<dyn Fn(_)>);
        let keyup_listener =
            wasm_bindgen::closure::Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
                raw_pressed_keys.update(|keys| {
                    keys.remove(&event.code().to_lowercase());
                });
            }) as Box<dyn Fn(_)>);


        document()
            .add_event_listener_with_callback_and_bool("keydown", keydown_listener.as_ref().unchecked_ref(), false)
            .expect("Failed to add keydown event listener");

        document()
            .add_event_listener_with_callback_and_bool("keyup", keyup_listener.as_ref().unchecked_ref(), false)
            .expect("Failed to add keyup event listener");

        on_cleanup(move || {
            document()
                .remove_event_listener_with_callback(
                    "keydown",
                    keydown_listener.as_ref().unchecked_ref(),
                )
                .expect("Failed to remove keydown event listener");
            document()
                .remove_event_listener_with_callback(
                    "keyup",
                    keyup_listener.as_ref().unchecked_ref(),
                )
                .expect("Failed to remove keyup event listener");
            keydown_listener.forget();
            keyup_listener.forget();
        });
    });

    let hotkeys_context = HotkeysContext {
        key_events: events_signal,
    };

    provide_context(hotkeys_context);
    hotkeys_context
}

pub fn use_hotkeys_context() -> HotkeysContext {
    use_context::<HotkeysContext>().expect("expected hotkeys context")
}