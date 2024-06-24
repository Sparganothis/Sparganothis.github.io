use leptos::*;

use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToggleSize {
    Small,
    Normal,
    Big,
}

impl ToggleSize {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Small => "small",
            Self::Normal => "normal",
            Self::Big => "big",
        }
    }
}

impl std::fmt::Display for ToggleSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl Default for ToggleSize {
    fn default() -> Self {
        Self::Normal
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ToggleIcons {
    pub off: icondata::Icon,
    pub on: icondata::Icon,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum ToggleVariant {
    #[default]
    Sliding,
    Stationary,
}

impl ToggleVariant {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Sliding => "sliding",
            Self::Stationary => "stationary",
        }
    }
}

#[component]
pub fn Toggle(
    #[prop(into)] state: MaybeSignal<bool>,
    #[prop(into, optional)] set_state: Option<Out<bool>>,
    #[prop(into, optional)] active: OptionalMaybeSignal<bool>,
    #[prop(into, optional)] disabled: OptionalMaybeSignal<bool>,
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: Option<AttributeValue>,
    #[prop(into, optional)] style: Option<AttributeValue>,
    #[prop(optional)] size: ToggleSize,
    #[prop(optional)] variant: ToggleVariant,
    #[prop(into, optional)] icons: Option<ToggleIcons>,
) -> impl IntoView {
    view! {
        <leptonic-toggle-wrapper class=class style=style>
            <leptonic-toggle
                id=id
                class:active=move || active.0.as_ref().map(SignalGet::get).unwrap_or(true)
                class:disabled=move || disabled.0.as_ref().map(SignalGet::get).unwrap_or(false)
                data-size=size.as_str()
                data-variant=variant.as_str()
                on:click=move |_| { if let Some(set) = &set_state { set.set(!state.get_untracked()) } }
            >
                <span class="slider round" class:on=move || state.get()>
                    {
                        move || icons.as_ref().map(|icons| {
                            let off_icon = icons.off;
                            let on_icon = icons.on;
                            view! {
                                <span class="icon-positioner">
                                    <Icon icon=off_icon style=move || match state.get() {
                                        true => "display: none",
                                        false => "display: inherit",
                                    } />
                                    <Icon icon=on_icon style=move || match state.get() {
                                        true => "display: inherit",
                                        false => "display: none",
                                    } />
                                </span>
                            }
                        })
                    }
                </span>
            </leptonic-toggle>
        </leptonic-toggle-wrapper>
    }
}
