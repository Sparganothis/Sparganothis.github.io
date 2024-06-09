use crate::game::tet::TetAction;
use leptos::*;
use leptos_hotkeys::use_hotkeys;
#[component]
pub fn HotkeyReader(#[prop(into)] on_action: Callback<TetAction>) -> impl IntoView {
    use_hotkeys!(("arrowup,keyx,ControlLeft,ControlRight") => move |_| {
        logging::log!("up has been pressed");
        on_action(TetAction::RotateRight);
    });

    use_hotkeys!(("arrowdown") => move |_| {
        logging::log!("down has been pressed");
        on_action(TetAction::SoftDrop);
    });

    use_hotkeys!(("Space") => move |_| {
        logging::log!("space has been pressed");
        on_action(TetAction::HardDrop);
    });

    use_hotkeys!(("KeyC,ShiftLeft,ShiftRight") => move |_| {
        logging::log!("C has been pressed");
        on_action(TetAction::Hold);
    });

    use_hotkeys!(("KeyZ") => move |_| {
        logging::log!("Z has been pressed");
        on_action(TetAction::RotateLeft);
    });

    use_hotkeys!(("ArrowLeft") => move |_| {
        logging::log!("Left has been pressed");
        on_action(TetAction::MoveLeft);
    });

    use_hotkeys!(("ArrowRight") => move |_| {
        logging::log!("Right has been pressed");
        on_action(TetAction::MoveRight);
    });
}
