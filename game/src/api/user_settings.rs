use serde::{Deserialize, Serialize};

#[derive(
    Copy, Debug, Serialize, Deserialize, Clone, PartialEq, PartialOrd, Eq, Ord, Hash,
)]
pub enum UserSettingType {
    SoundSetting(SoundSettingType),
    ControlSetting(ControlSettingType),
    ThemeSetting(ThemeSettingType),
}

#[derive(
    Copy, Debug, Serialize, Deserialize, Clone, PartialEq, PartialOrd, Eq, Ord, Hash,
)]
pub enum SoundSettingType {
    DisableAllSounds,
    DisableMenuMusic,
    MenuMusicVolume,
    MenuSoundsVolume,
}

#[derive(
    Copy, Debug, Serialize, Deserialize, Clone, PartialEq, PartialOrd, Eq, Ord, Hash,
)]
pub enum ControlSettingType {
    IHaveADHD,
}

#[derive(
    Copy, Debug, Serialize, Deserialize, Clone, PartialEq, PartialOrd, Eq, Ord, Hash,
)]
pub enum ThemeSettingType {
    BackgroundColor,
}