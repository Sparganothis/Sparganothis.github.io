use std::rc::Rc;

use leptos::*;
use uuid::Uuid;

use crate::{tab::TabData, Mount};

#[derive(Debug, Clone)]
pub struct TabHistory {
    active: Option<Oco<'static, str>>,
    previous: Option<Oco<'static, str>>,
}

impl TabHistory {
    pub const fn new() -> Self {
        Self {
            active: None,
            previous: None,
        }
    }

    pub const fn get_active(&self) -> Option<&Oco<'static, str>> {
        self.active.as_ref()
    }

    pub const fn get_previous(&self) -> Option<&Oco<'static, str>> {
        self.previous.as_ref()
    }

    pub fn push(&mut self, active: Oco<'static, str>) {
        self.previous = self.active.take();
        self.active = Some(active);
    }

    pub fn pop(&mut self) {
        self.active = self.previous.take();
    }
}

impl Default for TabHistory {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Copy, Clone)]
pub struct TabsContext {
    pub tabs: ReadSignal<Vec<TabData>>,
    pub set_tabs: WriteSignal<Vec<TabData>>,

    pub history: ReadSignal<TabHistory>,
    pub set_history: WriteSignal<TabHistory>,

    /// Default mount option when not otherwise specified for an individual tab.
    pub default_mount_type: Option<Mount>,
}

impl TabsContext {
    /// Register a tab with the given label.
    /// Automatically set this to be the active tab when no other tab is currently active.
    pub(crate) fn register(&self, tab: TabData) {
        let name = tab.name.clone();

        self.set_tabs.update(|tabs| {
            tabs.push(tab);
        });

        if self.history.get_untracked().get_active().is_none() {
            self.set_history.update(|history| {
                history.push(name);
            });
        }
    }

    pub(crate) fn deregister(&self, tab_id: Uuid) {
        self.set_tabs.update(|labels| {
            if let Some(idx) = labels.iter().position(|tab| tab.id == tab_id) {
                labels.remove(idx);
            }
        });

        if self.history.get_untracked().get_active().is_none() {
            self.set_history.update(|history| {
                history.pop();
            });
        }
    }
}

pub fn use_tabs() -> TabsContext {
    expect_context::<TabsContext>()
}

#[component]
pub fn Tabs(#[prop(optional)] mount: Option<Mount>, children: Children) -> impl IntoView {
    let (history, set_history) = create_signal(TabHistory::new());
    let (tabs, set_tabs) = create_signal(Vec::new());
    
    view! {
        <leptonic-tabs>
            <Provider value=TabsContext {
                history,
                set_history,
                tabs,
                set_tabs,
                default_mount_type: mount,
            }>
                <TabsContent children />
            </Provider>
        </leptonic-tabs>
    }
}

#[component]
pub fn TabsContent(children: Children) -> impl IntoView {    
    let ctx = use_tabs();

    // Note: Rendering out the children first is important for reliable SSR.
    // Children are `Tab`s, which register themselves in the previously constructed `TabsContext`.
    // Rendering the children inline in the `view!` macro would send down an empty `TabSelectors` 
    // which would then result in hydration errors!
    let children = children();
    
    view! {
        <TabSelectors tabs=ctx.tabs history=ctx.history set_history=ctx.set_history/>
        { children }
    }
}

#[component]
pub fn TabSelectors(
    tabs: ReadSignal<Vec<TabData>>,
    history: ReadSignal<TabHistory>,
    set_history: WriteSignal<TabHistory>,
) -> impl IntoView {
    let navigate = leptos_router::use_navigate();
    let location = leptos_router::use_location();
    let current_path = location.pathname.get_untracked();

    let update_tabs = move |current_hash| {
        tabs.with_untracked(|tabs| {
            let mut found_tab = false;
            for tab in tabs.iter() {
                if current_hash == tab.name.clone() {
                    set_history.update(|history| history.push(tab.name.clone()));
                    found_tab = true;
                    break;
                }
            }
            if !found_tab {
                if let Some(first_tab) = tabs.first() {
                    set_history.update(|history| history.push(first_tab.name.clone()));
                }
            }
        });
    };

    let stop_watch =    leptos::watch (
            move || location.hash.get(),
            move |current_hash, _prev_hash, _| {
                let current_hash = if current_hash.len() > 1 {(&current_hash[1..]).to_string()} else {"".to_string()};
    
                update_tabs(current_hash);
            },
            false,
        );
    leptos::on_cleanup(move || {
        stop_watch();
    });

    // do immmediate: false on watch() becacuse panicc bug
    let current_hash = location.hash.get_untracked();
    let current_hash = if current_hash.len() > 1 {(&current_hash[1..]).to_string()} else {"".to_string()};
    update_tabs(current_hash);

    view! {
        <leptonic-tab-selectors role="tablist">
            <For
                each=move || tabs.get()
                key=|tab| tab.id
                children=move |tab| {
                    let n1 = tab.name.clone();
                    let n2 = tab.name.clone();
                    let n3 = tab.name.clone();
                    let navigate2 = navigate.clone();
                    let current_path = current_path.clone();
                    view! {
                        <TabSelector
                            is_active=move || history.get().get_active() == Some(&n1.clone())
                            set_active=move || {
                                set_history.update(|history| history.push(n2.clone()));

                                let new_url = format!("{}#{}", current_path, n3);
                                navigate2(
                                    &new_url,
                                    leptos_router::NavigateOptions::default() 
                                );
                            }
                            name=tab.name.clone()
                            label=tab.label.clone() />
                    }
                }
            />
        </leptonic-tab-selectors>
    }
}

#[component]
fn TabSelector<A, S>(
    is_active: A,
    set_active: S,
    name: Oco<'static, str>,
    label: Rc<View>,
) -> impl IntoView
where
    A: Fn() -> bool + 'static,
    S: Fn() + 'static,
{
    view! {
        <leptonic-tab-selector
            data:for-name=name
            class:active=is_active
            on:click=move |_event| set_active()
            role="tab"
        >
            { (*label).clone() }
        </leptonic-tab-selector>
    }
}
