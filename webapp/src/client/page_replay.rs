use crate::game::random::GameSeed;

use super::game_board::RandomOpponentGameBoard;
use leptos::*;
 use leptonic::prelude::*;
 use icondata as i;
#[component]
pub fn GameReplayPage() -> impl IntoView {
    let seed: GameSeed = [0; 32];
    view! {
        <div class="main_left">
            <RandomOpponentGameBoard seed=seed/>
        </div>
        <div class="main_right">
            <Tabs mount=Mount::Once>
                <Tab name="tab-1" label="Tab 1".into_view()>
                    <div style="font-size: 8em; color: #8f39d3;">
                        <Icon icon=i::AiCarryOutTwotone style="color: green" />
                        <Icon icon=i::BiGraphql width="2em" height="2em" style="color: green"/>
                        <Icon icon=i::BiGraphql style="color: orange"/>
                        <Icon icon=i::Bs1Circle style="color: red"/>
                        <Icon icon=i::FaBarsSolid />
                        <Icon icon=i::ImPagebreak />
                        <Icon icon=i::ImPageBreak />
                        <Icon icon=i::OcAlertSm />
                        <Icon icon=i::OcAlertLg width="1em" height="2em" />
                    </div>
                </Tab>
                <Tab name="tab-2" label="Tab 2".into_view()>
                         "Content of tab 2"
                </Tab>
            </Tabs>
        </div>
    }
}
