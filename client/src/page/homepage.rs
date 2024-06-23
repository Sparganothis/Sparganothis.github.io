
use leptos::*;
#[component]
pub fn Homepage() -> impl IntoView {
    view! {
        <div class="main_left">
            <div style="border:solid purple 1px;height:7%;flex-direction: row;display: flex;">
                <div style="border:solid red 1px;width:20%;height:100%;">
                </div>
                <div style="border:solid yellow 1px;width:20%;height:100%;">
                </div>
                <div style="border:solid green 1px;width:20%;height:100%;">
                </div>
                <div style="border:solid blue 1px;width:20%;height:100%;">
                </div>
                <div style="border:solid blue 1px;width:20%;height:100%;">
                </div>
            </div>
            <div style="border:solid red 1px;height: 48%;flex-direction: row;display: flex;">
                <div style="border:solid blue 1px;width:50%;height:100%;flex-direction: column;display: flex;">
                    <div style="border:solid purple 1px;width:100%;height:50%;">
                    </div>
                    <div style="border:solid green 1px;width:100%;height:25%;">
                    </div>
                    <div style="border:solid red 1px;width:100%;height:25%;">
                    </div>
                </div>
                <div style="border:solid green 1px;width:50%;height:100%;">
                </div>
            </div>
            <div style="border:solid yellow 1px;height:10%;flex-direction: row;display: flex;">
                <div style="border:solid red 1px;width:25%;height:100%;">
                </div>
                <div style="border:solid yellow 1px;width:25%;height:100%;">
                </div>
                <div style="border:solid green 1px;width:25%;height:100%;">
                </div>
                <div style="border:solid blue 1px;width:25%;height:100%;">
                </div>

            </div>
            <div style="border:solid green 1px;height:35%;flex-direction: row;display: flex;">
                <div style="border:solid red 1px;width:25%;height:100%;">
                </div>
                <div style="border:solid yellow 1px;width:25%;height:100%;">
                </div>
                <div style="border:solid green 1px;width:25%;height:100%;">
                </div>
                <div style="border:solid blue 1px;width:25%;height:100%;">
                </div>

            </div>
        </div>
    }
}

