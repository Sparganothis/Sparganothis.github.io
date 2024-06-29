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
            <tr>
                <td>
                    <Toggle state=user_setting_signal.sound_disabled.read_only() set_state=user_setting_signal.sound_disabled.write_only()/>
                </td>
                <td>
                    <h3>Menu Music</h3>
                </td>
                <td>
                    <h3>
                        {move || {
                            (if user_setting_signal.sound_disabled.get() { "ON" } else { "OFF" }).to_string()
                        }}

                    </h3>
                </td>
            </tr>
            <tr>
                <td style="width:20vmin;">
                    <Slider
                        min=0.0
                        max=100.0
                        step=1.0
                        value=user_setting_signal.sound_volume.read_only()
                        set_value=user_setting_signal.sound_volume.write_only()
                        value_display=move |v| format!("{v:.0}")
                    />
                </td>
                <td>
                    <h3>Menu Music Volume</h3>
                </td>
                <td>{move || format!("{}", user_setting_signal.sound_volume.get())}</td>
            </tr>
        </table>
    }
}


