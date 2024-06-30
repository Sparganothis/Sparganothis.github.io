use game::api::user::{self, GuestInfo};
use leptonic::{slider::Slider, toggle::Toggle};
use leptos::*;

use super::server_api::UserSettingSignals;


#[allow(unused_variables)]
#[component]
pub fn SoundSettingsTab(user_profile: user::UserProfile, guest_id: GuestInfo) -> impl IntoView {

    let user_setting_signal = expect_context::<UserSettingSignals>();
    let has_cchanged_menu = create_rw_signal(false);

    let _ = leptos::watch(move || (user_setting_signal.sound_menu_music_enabled.track(),user_setting_signal.sound_menu_music_volume.track()), move |_, _, _| {
        has_cchanged_menu.set(true);
    }, false);
    
    view! {
        <table>

            // DISBLAE ALL SOUNDS
            <tr>
                <td style="width:20vmin;">
                    <Toggle
                        state=user_setting_signal.sound_enabled.read_only()
                        set_state=user_setting_signal.sound_enabled.write_only()
                    />
                </td>
                <td style="width:30vmin;">
                    <h3>Sound</h3>
                </td>
                <td style="width:20vmin;">
                    <h3>
                        {move || {
                            (if user_setting_signal.sound_enabled.get() {
                                "ON"
                            } else {
                                "OFF"
                            })
                                .to_string()
                        }}

                    </h3>
                </td>
            </tr>

            <Show when=move || user_setting_signal.sound_enabled.get()>

                // GLOBAL VOLUMEM
                <tr>
                    <td>
                        {move || {
                            if user_setting_signal.sound_enabled.get() {
                                view! {
                                    <Slider
                                        disabled=true
                                        min=1.0
                                        max=100.0
                                        step=1.0
                                        value=user_setting_signal
                                            .sound_all_sounds_volume
                                            .read_only()
                                        set_value=user_setting_signal
                                            .sound_all_sounds_volume
                                            .write_only()
                                        value_display=move |v| format!("{v:.0}")
                                    />
                                }
                                    .into_view()
                            } else {
                                view! {}.into_view()
                            }
                        }}

                    </td>
                    <td>
                        <h3>Volume</h3>
                    </td>
                    <td>
                        {move || {
                            format!(
                                "{}",
                                user_setting_signal.sound_all_sounds_volume.get(),
                            )
                        }}

                    </td>
                </tr>

                // DISABLE_MENU_MUSIC
                <tr>
                    <td>
                        <Toggle
                            state=user_setting_signal
                                .sound_menu_music_enabled
                                .read_only()
                            set_state=user_setting_signal
                                .sound_menu_music_enabled
                                .write_only()
                        />
                    </td>
                    <td>
                        <h3>Menu Music</h3>
                    </td>
                    <td>
                        <h3>
                            {move || {
                                (if user_setting_signal.sound_menu_music_enabled.get() {
                                    "ON"
                                } else {
                                    "OFF"
                                })
                                    .to_string()
                            }}

                        </h3>
                    </td>
                </tr>

                // MMENU MMUSIC VOLUME
                <Show when=move || user_setting_signal.sound_menu_music_enabled.get()>
                    <tr>
                        <td style="width:20vmin;">
                            <Slider
                                min=1.0
                                max=100.0
                                step=1.0
                                value=user_setting_signal
                                    .sound_menu_music_volume
                                    .read_only()
                                set_value=user_setting_signal
                                    .sound_menu_music_volume
                                    .write_only()
                                value_display=move |v| format!("{v:.0}")
                            />
                        </td>
                        <td>
                            <h3>Music Volume</h3>
                        </td>
                        <td>
                            {move || {
                                format!(
                                    "{}",
                                    user_setting_signal.sound_menu_music_volume.get(),
                                )
                            }}

                        </td>
                    </tr>
                </Show>

                <Show when=move || has_cchanged_menu.get()>

                    <tr>
                        <td colspan="100%">
                            <h5 style="color:red; font-weight: bold;">
                                This setting may require refresh
                            </h5>
                        </td>
                    </tr>
                </Show>

            </Show>

        </table>
    }
}

