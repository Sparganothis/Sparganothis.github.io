use leptos_hotkeys::use_hotkeys;
use leptos::*;
use crate::tet::TetAction;
#[component]
pub fn HotkeyReader() -> impl IntoView {
    let (get_act, set_act) = create_signal(TetAction::Nothing);

    use_hotkeys!(("arrowup,keyx,ControlLeft,ControlRight") => move |_| {
        logging::log!("up has been pressed");
        set_act.update(|c| *c=TetAction::RotateRight);
    });

    use_hotkeys!(("arrowdown") => move |_| {
        logging::log!("down has been pressed");
        set_act.update(|c| *c = TetAction::SoftDrop);
    });

    use_hotkeys!(("Space") => move |_| {
        logging::log!("space has been pressed");
        set_act.update(|c| *c = TetAction::HardDrop);
    });

    use_hotkeys!(("KeyC,ShiftLeft,ShiftRight") => move |_| {
        logging::log!("C has been pressed");
        set_act.update(|c| *c = TetAction::Hold);
    });

    use_hotkeys!(("KeyZ") => move |_| {
        logging::log!("Z has been pressed");
        set_act.update(|c| *c = TetAction::RotateLeft);
    });

    use_hotkeys!(("ArrowLeft") => move |_| {
        logging::log!("Left has been pressed");
        set_act.update(|c| *c = TetAction::MoveLeft);
    });

    use_hotkeys!(("ArrowRight") => move |_| {
        logging::log!("Right has been pressed");
        set_act.update(|c| *c = TetAction::MoveRight);
    });

    view! { <p>Num Respects: {move || format!("{:?}", get_act())}</p> }
}