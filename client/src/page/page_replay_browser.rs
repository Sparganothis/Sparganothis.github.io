use game::random::GameSeed;

use crate::comp::{
    game_board::RandomOpponentGameBoard, table_replay_games::AllGamesTable,
};
use game::api::websocket::GetAllGamesArg;
use icondata as i;
use leptonic::prelude::*;
use leptos::*;

#[component]
pub fn GameReplayBrowserPage() -> impl IntoView {
    let seed: GameSeed = [0; 32];
    view! {
        <div class="main_right">
            <RandomOpponentGameBoard seed=seed/>
        </div>
        <div class="main_left">

            <Tabs mount=Mount::Once>

                <Tab name="tab-best-games" label="Best Games".into_view()>
                    <AllGamesTable list_type=GetAllGamesArg::BestGames/>
                </Tab>

                <Tab name="tab-recent-games" label="Recent Games".into_view()>
                    <AllGamesTable list_type=GetAllGamesArg::RecentGames/>
                </Tab>

                <Tab name="tab-my-best-games" label="My Best Games".into_view()>
                    <AllGamesTable list_type=GetAllGamesArg::MyBestGames/>
                </Tab>

                <Tab name="tab-my-recent-games" label="My Recent Games".into_view()>
                    <AllGamesTable list_type=GetAllGamesArg::MyRecentGames/>
                </Tab>

                <Tab name="tab-1" label="Tab 1".into_view()>
                    <div style="font-size: 8em; color: #8f39d3;">
                        <Icon icon=i::AiCarryOutTwotone style="color: green"/>
                        <Icon
                            icon=i::BiGraphql
                            width="2em"
                            height="2em"
                            style="color: green"
                        />
                        <Icon icon=i::BiGraphql style="color: orange"/>
                        <Icon icon=i::Bs1Circle style="color: red"/>
                        <Icon icon=i::FaBarsSolid/>
                        <Icon icon=i::ImPagebreak/>
                        <Icon icon=i::ImPageBreak/>
                        <Icon icon=i::OcAlertSm/>
                        <Icon icon=i::OcAlertLg width="1em" height="2em"/>

                    </div>
                </Tab>

            </Tabs>
        </div>
    }
}
