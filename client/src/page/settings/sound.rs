use game::api::user::{self, GuestInfo};
use leptonic::{slider::Slider, toggle::Toggle};
use leptos::*;

use crate::audio3::{change_global_volume_js, change_sound_volume_js, play_sound_js, stop_all_sound_js, stop_sound_js};

use super::server_api::UserSettingSignals;


#[allow(unused_variables)]
#[component]
pub fn SoundSettingsTab(user_profile: user::UserProfile, guest_id: GuestInfo) -> impl IntoView {

    let user_setting_signal = expect_context::<UserSettingSignals>();
    create_volume_control_to_js_reactor(user_setting_signal);
    
    view! {
        <table>

            // DISBLAE ALL SOUNDS
            <ToggleSettingRow signal= user_setting_signal.all_sound_enabled title="Sound".to_string() on_label="ON".to_string() off_label="OFF".to_string()/>
            // VOLUMME ALL SOUNDS
            <Show when=move || user_setting_signal.all_sound_enabled.get()>
                <SliderSettingRow signal=user_setting_signal.sound_all_sounds_volume title="Volume".to_string()/>
                // DISABLE_MENU_MUSIC
                <ToggleSettingRow signal= user_setting_signal.sound_menu_music_enabled title="Menu Music".to_string() on_label="ON".to_string() off_label="OFF".to_string()/>
                // MMENU MMUSIC VOLUME
                <Show when=move || user_setting_signal.sound_menu_music_enabled.get()>
                    <SliderSettingRow signal=user_setting_signal.sound_menu_music_volume title="Music Volume".to_string()/>
                </Show>        
            </Show>
        </table>
    }
}


#[component]
pub fn SliderSettingRow(signal: RwSignal<f64>, title: String)-> impl IntoView{

    view!{

        <tr>
            <td style="width:20vmin;">
                <Slider
                    min=1.0
                    max=100.0
                    step=1.0
                    value=signal
                        .read_only()
                    set_value=signal
                        .write_only()
                    value_display=move |v| format!("{v:.0}")
                />
            </td>
            <td>
                <h3>{title}</h3>
            </td>
            <td>
                {move || {
                    format!(
                        "{}",
                        signal.get(),
                    )
                }}

            </td>
        </tr>


    }


}



//TOGGLE COMP
#[component]
pub fn ToggleSettingRow(signal: RwSignal<bool>, title: String, on_label: String, off_label: String)-> impl IntoView{


    view!{
        <tr>
            <td style="width:20vmin;">
                <Toggle
                    state=signal.read_only()
                    set_state=signal.write_only()
                />
            </td>
            <td style="width:30vmin;">
                <h3>{title}</h3>
            </td>
            <td style="width:20vmin;">
                <h3>
                    {
                        move || {
                        (if signal.get() {
                        on_label.clone()
                        } else {
                        off_label.clone()
                        })
                    }}

                </h3>
            </td>
        </tr>
    }


}



pub fn create_volume_control_to_js_reactor(user_setting_signal:UserSettingSignals){
    
     // all sound on/of      
    let _ = leptos::watch(
        move || (
            user_setting_signal.all_sound_enabled.get()
        ),
        move |enabled, _, _| {
            if !enabled{
                stop_all_sound_js();
            }  
            if *enabled && user_setting_signal.sound_menu_music_enabled.get_untracked(){
                play_sound_js(
                    "mmenu_mmusicc".to_string(), 
                
                    user_setting_signal.sound_menu_music_volume.get_untracked()
                );
            }

        }, 
        false
    );

    // all sound volume      
    let _ = leptos::watch(
        move || (
            
            user_setting_signal.sound_all_sounds_volume.get()
        ),
        move |volume, _, _| {
            change_global_volume_js(*volume);
            
        }, 
        false
    );

    // mmenu musicc volumme     
    let _ = leptos::watch(
        move || (
            
            user_setting_signal.sound_menu_music_volume.get()
        ),
        move |volume, _, _| {
            change_sound_volume_js("mmenu_mmusicc".to_string(),*volume);
            
        }, 
        false
    );

    // mmenu musicc enabled      
    let _ = leptos::watch(
        move || (
            user_setting_signal.sound_menu_music_enabled.get()
        ),
        move |enabled, _, _| {
            if !enabled{
                stop_sound_js("mmenu_mmusicc".to_string());
            }
            else {
                play_sound_js(
                    "mmenu_mmusicc".to_string(), 
                
                    user_setting_signal.sound_menu_music_volume.get_untracked()
                );
            }
            
        }, 
        false
    ); 
}


