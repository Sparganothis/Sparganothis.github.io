
use leptos::*;
#[component]
pub fn Homepage() -> impl IntoView {
    view! {
        <div class="main_left">
            <div style="border:solid purple 1px;height:7%;flex-direction: row;display: flex;">
                <div style="border:solid red 1px;width:20%;height:100%;"><h1>1</h1></div>
                <div style="border:solid yellow 1px;width:20%;height:100%;"><h1>2</h1></div>
                <div style="border:solid green 1px;width:20%;height:100%;"><h1>3</h1></div>
                <div style="border:solid blue 1px;width:20%;height:100%;"><h1>4</h1></div>
                <div style="border:solid blue 1px;width:20%;height:100%;"><h1>5</h1></div>
            </div>
            <div style="border:solid red 1px;height: 48%;flex-direction: row;display: flex;">
                <div style="border:solid blue 1px;width:50%;height:100%;flex-direction: column;display: flex;">
                    <div style="border:solid purple 1px;width:100%;height:50%;"><h1>6</h1></div>
                    <div style="border:solid green 1px;width:100%;height:25%;"><h1>7</h1></div>
                    <div style="border:solid red 1px;width:100%;height:25%;"><h1>8</h1></div>
                </div>
                <div style="border:solid green 1px;width:50%;height:100%;"><h1>17</h1></div>
            </div>
            <div style="border:solid yellow 1px;height:10%;flex-direction: row;display: flex;">
                <div style="border:solid red 1px;width:25%;height:100%;"><h1>9</h1></div>
                <div style="border:solid yellow 1px;width:25%;height:100%;"><h1>10</h1></div>
                <div style="border:solid green 1px;width:25%;height:100%;"><h1>11</h1></div>
                <div style="border:solid blue 1px;width:25%;height:100%;"><h1>12</h1></div>

            </div>
            <div style="border:solid green 1px;height:35%;flex-direction: row;display: flex;">
                <div style="border:solid red 1px;width:25%;height:100%;"><h1>13</h1></div>
                <div style="border:solid yellow 1px;width:25%;height:100%;"><h1>14</h1></div>
                <div style="border:solid green 1px;width:25%;height:100%;"><h1>15</h1></div>
                <div style="border:solid blue 1px;width:25%;height:100%;"><h1>16</h1></div>

            </div>
        </div>
    }
}

