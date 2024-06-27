use leptos::*;

#[component]
pub fn MenuGridView(views:Vec<View>) -> impl IntoView {
    view! {
        <div class="main_left">
            <div style="border:solid purple 1px;height:7%;flex-direction: row;display: flex;">
                <div style="border:solid red 1px;width:20%;height:100%;">
                    {views.get(0)}
                </div>
                <div style="border:solid yellow 1px;width:20%;height:100%;">
                    {views.get(1)}
                </div>
                <div style="border:solid green 1px;width:20%;height:100%;">
                    {views.get(2)}
                </div>
                <div style="border:solid blue 1px;width:20%;height:100%;">
                    {views.get(3)}
                </div>
                <div style="border:solid blue 1px;width:20%;height:100%;">
                    {views.get(4)}
                </div>
            </div>
            <div style="border:solid red 1px;height: 48%;flex-direction: row;display: flex;">
                <div style="border:solid blue 1px;width:50%;height:100%;flex-direction: column;display: flex;">
                    <div style="border:solid purple 1px;width:100%;height:50%;">
                        {views.get(5)}
                    </div>
                    <div style="border:solid green 1px;width:100%;height:25%;">
                        {views.get(6)}
                    </div>
                    <div style="border:solid red 1px;width:100%;height:25%;">
                        {views.get(7)}
                    </div>
                </div>
                <div style="border:solid green 1px;width:50%;height:100%;">
                    {views.get(8)}
                </div>
            </div>
            <div style="border:solid yellow 1px;height:10%;flex-direction: row;display: flex;">
                <div style="border:solid red 1px;width:25%;height:100%;">
                    {views.get(9)}
                </div>
                <div style="border:solid yellow 1px;width:25%;height:100%;">
                    {views.get(10)}
                </div>
                <div style="border:solid green 1px;width:25%;height:100%;">
                    {views.get(11)}
                </div>
                <div style="border:solid blue 1px;width:25%;height:100%;">
                    {views.get(12)}
                </div>

            </div>
            <div style="border:solid green 1px;height:35%;flex-direction: row;display: flex;">
                <div style="border:solid red 1px;width:25%;height:100%;">
                    {views.get(13)}
                </div>
                <div style="border:solid yellow 1px;width:25%;height:100%;">
                    {views.get(14)}
                </div>
                <div style="border:solid green 1px;width:25%;height:100%;">
                    {views.get(15)}
                </div>
                <div style="border:solid blue 1px;width:25%;height:100%;">
                    {views.get(16)}
                </div>

            </div>
        </div>
    }
}
