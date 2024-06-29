use game::api::user::{self, GuestInfo};
use leptonic::{slider::Slider, toggle::Toggle};
use leptos::*;

use super::server_api::UserSettingSignals;


#[allow(unused_variables)]
#[component]
pub fn SoundSettingsTab(user_profile: user::UserProfile, guest_id: GuestInfo) -> impl IntoView {

    let user_setting_signal = expect_context::<UserSettingSignals>();
    view! {
        <table>

            // DISBLAE ALL SOUNDS
            <tr>
                <td>
                    <Toggle state=user_setting_signal.sound_disabled.read_only() set_state=user_setting_signal.sound_disabled.write_only()/>
                </td>
                <td>
                    <h3>Disable All Sounds</h3>
                </td>
                <td>
                    <h3>
                        {move || {
                            (if user_setting_signal.sound_disabled.get() { "Sound OFF" } else { "Sound ON" }).to_string()
                        }}

                    </h3>
                </td>
            </tr>


            //GLOBAL VOLUMEM
            <tr>
                <td style="width:20vmin;">
                    <Slider
                        min=1.0
                        max=100.0
                        step=1.0
                        value=user_setting_signal.sound_all_sounds_volume.read_only()
                        set_value=user_setting_signal.sound_all_sounds_volume.write_only()
                        value_display=move |v| format!("{v:.0}")
                    />
                </td>
                <td>
                    <h3>Global Volume</h3>
                </td>
                <td>{move || format!("{}", user_setting_signal.sound_all_sounds_volume.get())}</td>
            </tr>

            
            // DISABLE_MENU_MUSIC
            <tr>
                <td>
                    <Toggle state=user_setting_signal.sound_menu_music_disabled.read_only() set_state=user_setting_signal.sound_menu_music_disabled.write_only()/>
                </td>
                <td>
                    <h3>Disable Menu MMusic</h3>
                </td>
                <td>
                    <h3>
                        {move || {
                            (if user_setting_signal.sound_menu_music_disabled.get() { "Sound OFF" } else { "Sound ON" }).to_string()
                        }}

                    </h3>
                </td>
            </tr>

            // MMENU MMUSIC VOLUME
            <tr>
                <td style="width:20vmin;">
                    <Slider
                        min=1.0
                        max=100.0
                        step=1.0
                        value=user_setting_signal.sound_menu_music_volume.read_only()
                        set_value=user_setting_signal.sound_menu_music_volume.write_only()
                        value_display=move |v| format!("{v:.0}")
                    />
                </td>
                <td>
                    <h3>Menu Music Volumme</h3>
                </td>
                <td>{move || format!("{}", user_setting_signal.sound_menu_music_volume.get())}</td>
            </tr>


        </table>
    }
}


