
use game::tet::GameState;
use leptos::*;

use crate::comp::game_board::GameBoard;

#[component]
pub fn GameBoardFlex() -> impl IntoView {
    view! {
        <div style="border:solid purple 1px;height:100%;flex-direction: column;display: flex;">
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
                    <div style="border:solid red 1px;width:100%;height:10%;">
                        <h3>HOLD</h3>
                    </div>
                    <div style="border:solid yellow 1px;width:100%;height:20%;flex-direction: row;display: flex;">
                        <div style="border:solid red 1px;width:7%;height:100%;"></div>
                        // HOLD BORD
                        <div style="border:solid yellow 1px;width:86%;height:100%;background-color: black;"></div>
                        <div style="border:solid green 1px;width:7%;height:100%;"></div>

                    </div>
                    <div style="border:solid green 1px;width:100%;height:25%;"></div>

                    <div style="border:solid green 1px;width:100%;height:15%;"></div>
                    <div style="border:solid green 1px;width:100%;height:30%;"></div>

                </div>
                <div style="border:solid green 1px;width:50%;height:100%;flex-direction: column;display: flex;background-color: black"></div>

                // NEXT
                <div style="border:solid green 1px;width:25%;height:100%;flex-direction: column;display: flex;">
                    <div style="border:solid red 1px;width:100%;height:10%;">
                        <h3>NEXT</h3>
                    </div>
                    <div style="border:solid yellow 1px;width:100%;height:75%;flex-direction: row;display: flex;">
                        <div style="border:solid red 1px;width:7%;height:100%;"></div>
                        // NEXT BORD
                        <div style="border:solid yellow 1px;width:86%;height:100%;background-color: black;"></div>
                        <div style="border:solid green 1px;width:7%;height:100%;"></div>

                    </div>
                    <div style="border:solid green 1px;width:100%;height:15%;"></div>
                </div>
            </div>
            <div style="border:solid yellow 1px;height:15%;flex-direction: row;display: flex;">

            </div>
        </div>
    }
}
