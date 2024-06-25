
use game::tet::{self, GameState};
use leptos::*;

use crate::{comp::game_board::BoardTable, style::{flex_gameboard_style, GameBoardTetStyle}};



#[component]
pub fn GameBoardFlexDemoPage() -> impl IntoView {
    let game_state = create_rw_signal(GameState::empty());
    view! {
        <div class="main_left">
            <GameBoardFlex game_state/>
        </div>
    }
}

#[component]
pub fn GameBoardFlex(
    #[prop(into)] game_state: RwSignal<tet::GameState>,

    #[prop(default = Callback::<()>::new(move |_| {}))]
    #[prop(optional)]
    on_reset_game: Callback<()>,

    #[prop(default = Callback::<(i8, i8)>::new(move |_| {}))]
    #[prop(optional)]
    on_main_cell_click: Callback<(i8, i8)>,

    #[prop(into)]
    #[prop(default = create_rw_signal("".to_string()).read_only())]
    #[prop(optional)]
    pre_countdown_text: ReadSignal<String>,) -> impl IntoView {
    let tet_style = GameBoardTetStyle::new();

    let _style_name = flex_gameboard_style(tet_style, )
        .get_class_name()
        .to_owned();

    let hold_board =
        create_read_slice(game_state, |state: &tet::GameState| state.get_hold_board());

    let next_board =
        create_read_slice(game_state, |state: &tet::GameState| state.get_next_board());

    let main_board =
        create_read_slice(game_state, |state: &tet::GameState| state.main_board);

    let gameover = view! {
        <Show when=move || game_state.get().game_over fallback=|| view! {}>
            <div class="game_over_display" on:click=move |_| on_reset_game.call(())>
                you lose
            </div>
        </Show>
    };

    let pre_countdown = view! {
        <Show when=move || { pre_countdown_text.get().len() > 0 } fallback=|| view! {}>
            <div class="pre_game_countdown_display">{pre_countdown_text}</div>
        </Show>
    };

    // TODO PUT THIS ON SCREEN
    let _countdown_view = view  ! {
                        <div class="gameover">{gameover}</div>
        <div class="pre_game_countdown">{pre_countdown}</div>
    };

    view! {
        <div class=_style_name style="border:solid purple 1px;height:100%;flex-direction: column;display: flex;">
            <div style="border:solid purple 1px;height:5%;flex-direction: row;display: flex;">
                <div style="border:solid red 1px;width:15%;height:100%;"></div>
                <div style="border:solid yellow 1px;width:20%;height:100%;"></div>
                <div style="border:solid green 1px;width:20%;height:100%;"></div>
                <div style="border:solid blue 1px;width:20%;height:100%;"></div>
                <div style="border:solid blue 1px;width:20%;height:100%;"></div>
            </div>
            <div style="border:solid purple 1px;height:10%;flex-direction: row;display: flex;">
                <div style="border:solid red 1px;width:30%;height:100%;"></div>
                <div style="border:solid yellow 1px;width:40%;height:100%;"></div>
                <div style="border:solid green 1px;width:30%;height:100%;"></div>
            </div>
            <div style="border:solid red 1px;height: 75%;flex-direction: row;display: flex;">


                // HOLD
                <div style="border:solid green 1px;width:25%;height:100%;flex-direction: column;display: flex;">
                    <div style="border:solid red 1px;width:100%;height:10%; container-type: size;">
                        <h3 style="font-size:80cqh; text-align: center;">HOLD</h3>
                    </div>
                    <div style="border:solid yellow 1px;width:100%;height:17%;flex-direction: row;display: flex;">
                        <div style="border:solid red 1px;width:7%;height:100%;"></div>
                        // HOLD BORD
                        <div style="border:solid yellow 1px;width:86%;height:100%;">
                            <BoardTable board=hold_board/>
                        </div>
                        <div style="border:solid green 1px;width:7%;height:100%;"></div>

                    </div>
                    <div style="border:solid green 1px;width:100%;height:30%;"></div>

                    // SCORE BOARD
                    <div style="border:solid green 1px;width:100%;height:6%; container-type: size;">
                        <h3 style="font-size:80cqh; text-align: center;">
                            {move || { format!("{:?}", game_state.get().score) }}
                        </h3>
                    </div>
                    <div style="border:solid green 1px;width:100%;height:37%;"></div>

                </div>
                // MAIN
                <div style="border:solid green 1px;width:50%;height:100%;flex-direction: column;display: flex;">
                    <BoardTable board=main_board on_click=on_main_cell_click />
                </div>

                // NEXT
                <div style="border:solid green 1px;width:25%;height:100%;flex-direction: column;display: flex;">
                    <div style="border:solid red 1px;width:100%;height:10%;container-type: size;">
                        <h3 style="font-size:80cqh; text-align: center;">NEXT</h3>
                    </div>
                    <div style="border:solid yellow 1px;width:100%;height:75%;flex-direction: row;display: flex;">
                        <div style="border:solid red 1px;width:12%;height:100%;"></div>
                        // NEXT BORD
                        <div style="border:solid yellow 1px;width:76%;height:100%;">
                            <BoardTable board=next_board/>
                        </div>
                        <div style="border:solid green 1px;width:12%;height:100%;"></div>

                    </div>
                    <div style="border:solid green 1px;width:100%;height:15%;"></div>
                </div>
            </div>
            <div style="border:solid yellow 1px;height:15%;flex-direction: row;display: flex;"></div>
        </div>
    }
}
