use crate::{
    error_template::{AppError, ErrorTemplate},
    pages::editor::ThemeEditor,
};

use leptos::*;
use leptos_meta::{provide_meta_context, Link as MetaLink, Meta, Stylesheet, Title};
use leptos_router::*;
use leptos_use::use_media_query;

use leptonic::prelude::*;

use crate::pages::{documentation::doc_root::DocRoutes, err404::PageErr404, welcome::PageWelcome};

#[derive(Debug, Copy, Clone)]
pub enum AppRoutes {
    Welcome,
    Doc,
    ThemeEditor,
    NotFound,
}

impl AppRoutes {
    pub const fn route(self) -> &'static str {
        match self {
            Self::Welcome => "",
            Self::Doc => "doc",
            Self::ThemeEditor => "theme-editor",
            Self::NotFound => "not-found", // Leptos requires this to be be named "*"!
        }
    }
}

/// Required so that `Routes` variants can be used in `<Route path=Routes::Foo ...>` definitions.
impl std::fmt::Display for AppRoutes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.route())
    }
}

/// Required so that `Routes` variants can be used in `<Link href=Routes::Foo ...>` definitions.
impl ToHref for AppRoutes {
    fn to_href(&self) -> Box<dyn Fn() -> String + '_> {
        Box::new(move || format!("/{}", self.route()))
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Meta name="description" content="Leptonic"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>
        <Meta name="theme-color" content="#e66956"/>

        <Stylesheet id="leptos" href="/pkg/book-ssr.css"/>
        <Stylesheet href="https://fonts.googleapis.com/css?family=Roboto&display=swap"/>

        <MetaLink rel="icon" href="/res/icon/leptonic_x64.png"/>
        <MetaLink rel="apple-touch-icon" href="/res/icon/maskable_icon_x192.png"/>

        <Title text="Leptonic"/>

        <Root default_theme=LeptonicTheme::default()>
            <Router fallback=|| {
                let mut outside_errors = Errors::default();
                outside_errors.insert_with_default_key(AppError::NotFound);
                view! {
                    <Layout>
                        <ErrorTemplate outside_errors/>
                    </Layout>
                }
                .into_view()
            }>
                <Routes>
                    <Route path="" view=|| view! { <Layout/> }>
                        <Route path=AppRoutes::Welcome view=|| view! { <PageWelcome/> }/>
                        <DocRoutes path=AppRoutes::Doc/>
                        <Route path=AppRoutes::ThemeEditor view=|| view! { <ThemeEditor/> }/>
                        <Route path=AppRoutes::NotFound view=|| view! { <PageErr404 /> }/>
                    </Route>
                </Routes>
            </Router>
        </Root>
    }
}

pub const APP_BAR_HEIGHT: Height = Height::Em(3.5);

#[derive(Debug, Clone, Copy)]
pub struct AppLayoutContext {
    pub is_small: Signal<bool>,
    pub main_drawer_closed: Signal<bool>,
    set_main_drawer_closed: WriteSignal<bool>,
    pub doc_drawer_closed: Signal<bool>,
    set_doc_drawer_closed: WriteSignal<bool>,
}

impl AppLayoutContext {
    #[allow(unused)]
    pub fn close_main_drawer(&self) {
        self.set_main_drawer_closed.set(true);
    }

    pub fn close_doc_drawer(&self) {
        self.set_doc_drawer_closed.set(true);
    }

    pub fn toggle_main_drawer(&self) {
        let currently_closed = self.main_drawer_closed.get_untracked();
        self.set_main_drawer_closed.set(!currently_closed);
        if currently_closed {
            self.close_doc_drawer();
        }
    }

    pub fn toggle_doc_drawer(&self) {
        let currently_closed = self.doc_drawer_closed.get_untracked();
        self.set_doc_drawer_closed.set(!currently_closed);
        if currently_closed {
            self.close_main_drawer();
        }
    }
}

#[component]
#[allow(clippy::too_many_lines)]
pub fn Layout(#[prop(optional)] children: Option<Children>) -> impl IntoView {
    let is_small = use_media_query("(max-width: 800px)");

    let router_context = use_context::<RouterContext>();
    let is_doc = create_memo(move |_| {
        router_context
            .as_ref()
            .map(|router| router.pathname().get().starts_with("/doc"))
            .unwrap_or(false)
    });

    // The main drawer is only used on mobile / small screens!.
    let (main_drawer_closed, set_main_drawer_closed) = create_signal(true);
    let (doc_drawer_closed, set_doc_drawer_closed) = create_signal(false);

    // Always close the doc-drawer when the application is now small.
    // Always open the doc-drawer when the application is no longer small.
    create_effect(move |_| {
        if is_small.get() {
            set_doc_drawer_closed.set(true);
        } else {
            set_doc_drawer_closed.set(false);
        }
    });

    // Always close the main-drawer when the application is no longer small.
    create_effect(move |_| {
        if !is_small.get() {
            set_main_drawer_closed.set(true);
        }
    });

    let ctx = AppLayoutContext {
        is_small,
        main_drawer_closed: main_drawer_closed.into(),
        set_main_drawer_closed,
        doc_drawer_closed: doc_drawer_closed.into(),
        set_doc_drawer_closed,
    };

    provide_context(ctx);

    let search_options = vec![
        create_search_option(DocRoutes::Overview, "Overview"),
        create_search_option(DocRoutes::Installation, "Installation"),
        create_search_option(DocRoutes::Themes, "Themes"),
        create_search_option(DocRoutes::Changelog, "Changelog"),
        create_search_option(DocRoutes::Stack, "Grid"),
        create_search_option(DocRoutes::Separator, "Separator"),
        create_search_option(DocRoutes::Skeleton, "App Bar"),
        create_search_option(DocRoutes::Drawer, "Drawer"),
        create_search_option(DocRoutes::Tab, "Tabs"),
        create_search_option(DocRoutes::Table, "Table"),
        create_search_option(DocRoutes::Collapsible, "Collapsible"),
        create_search_option(DocRoutes::Button, "Button"),
        create_search_option(DocRoutes::Input, "Input"),
        create_search_option(DocRoutes::TiptapEditor, "Tiptap Editor"),
        create_search_option(DocRoutes::DateTime, "Date & Time"),
        create_search_option(DocRoutes::Slider, "Slider"),
        create_search_option(DocRoutes::Select, "Select"),
        create_search_option(DocRoutes::ColorPicker, "Color Picker"),
        create_search_option(DocRoutes::Alert, "Alert"),
        create_search_option(DocRoutes::Toast, "Toast"),
        create_search_option(DocRoutes::Modal, "Modal"),
        create_search_option(DocRoutes::Progress, "Progress"),
        create_search_option(DocRoutes::Popover, "Popover"),
        create_search_option(DocRoutes::Chip, "Chip"),
        create_search_option(DocRoutes::Kbd, "Keyboard"),
        create_search_option(DocRoutes::Typography, "Typography"),
        create_search_option(DocRoutes::Icon, "Icon"),
        create_search_option(DocRoutes::Link, "Link"),
        create_search_option(DocRoutes::Anchor, "Anchor"),
        create_search_option(DocRoutes::Callback, "Callback"),
        //create_search_option(DocRoutes::Transition, "Transition"),
    ];

    let logo = move || {
        view! {
            <Link href="">
                <img src="/res/leptonic.svg" id="logo" alt="Leptonic logo"/>
            </Link>
        }
    };

    view! {
        <AppBar id="app-bar" height=APP_BAR_HEIGHT>
            <div id="app-bar-content">
                <Stack id="left" orientation=StackOrientation::Horizontal spacing=Size::Zero>
                    { move || match (is_doc.get(), is_small.get()) {
                        (false, true) => logo().into_view(),
                        (true, true) => view! {
                            <Icon id="mobile-menu-trigger" icon=icondata::BsList on:click=move |_| ctx.toggle_doc_drawer()/>
                            { logo }
                        }.into_view(),
                        (_, false) => view! {
                            { logo }
                            <Link href=AppRoutes::Doc>
                                <H3 style="margin: 0 0 0 0.5em">
                                    "Docs"
                                </H3>
                            </Link>
                            //<Link href=AppRoutes::ThemeEditor>
                            //    <H3 style="margin: 0 0 0 0.5em">
                            //        "Theme Editor"
                            //    </H3>
                            //</Link>
                        }.into_view(),
                    } }
                </Stack>

                <Stack id="center" orientation=StackOrientation::Horizontal spacing=Size::Em(1.0)>
                    <Quicksearch
                        id="quicksearch"
                        trigger=move |set_quicksearch| view! {
                            <QuicksearchTrigger id="quicksearch-trigger" set_quicksearch=set_quicksearch>
                                { move || match is_small.get() {
                                    true => view! { <Icon icon=icondata::BsSearch />}.into_view(),
                                    false => view! { "Search"}.into_view(),
                                } }
                            </QuicksearchTrigger>
                        }
                        query=move |search: String| {
                            if search.is_empty() {
                                return vec![];
                            }
                            let lower_search = search.to_lowercase();
                            search_options.iter()
                                .filter(|it| it.label.to_lowercase().contains(&lower_search))
                                .cloned()
                                .collect::<Vec<_>>()
                        }
                    />
                </Stack>

                <Stack id="right" orientation=StackOrientation::Horizontal spacing=Size::Em(1.0)>
                    { move || match is_small.get() {
                        true => view! {
                            <Icon id="mobile-menu-trigger" icon=icondata::BsThreeDots on:click=move |_| ctx.toggle_main_drawer()/>
                        }.into_view(),
                        false => view! {
                            <Link href=DocRoutes::Changelog>"v0.5.0 (main)"</Link>

                            <LinkExt href="https://github.com/lpotthast/leptonic" target=LinkExtTarget::Blank>
                                <Icon id="github-icon" icon=icondata::BsGithub aria_label="GitHub icon"/>
                            </LinkExt>

                            <ThemeToggle off=LeptonicTheme::Light on=LeptonicTheme::Dark style="margin-right: 1em"/>
                        }.into_view(),
                    } }
                </Stack>
            </div>
        </AppBar>

        <Box id="content" style=format!("height: calc(var(--leptonic-vh, 100vh) - {APP_BAR_HEIGHT}); max-height: calc(var(--leptonic-vh, 100vh) - {APP_BAR_HEIGHT});")>
            {
                match children {
                    Some(children) => {
                        let children = children();
                        view! {{children}}.into_view()
                    },
                    None => view! {
                        // <Outlet/> will show nested child routes.
                        <Outlet/>
                    }.into_view(),
                }
            }

            <Drawer id="main-drawer" shown=Signal::derive(move || !main_drawer_closed.get()) side=DrawerSide::Right style=format!("top: {APP_BAR_HEIGHT}")>
                <Stack orientation=StackOrientation::Vertical spacing=Size::Em(2.0) class="menu">

                    <LinkExt href="https://github.com/lpotthast/leptonic" target=LinkExtTarget::Blank style="font-size: 3em;">
                        <Icon id="github-icon" icon=icondata::BsGithub/>
                    </LinkExt>

                    <ThemeToggle off=LeptonicTheme::Light on=LeptonicTheme::Dark style="margin-right: 1em"/>

                    "Currently - v0.5.0 (main)"
                </Stack>
            </Drawer>
        </Box>
    }
}

fn create_search_option(route: DocRoutes, label: &'static str) -> QuicksearchOption {
    QuicksearchOption {
        label: label.into(),
        view: ViewProducer::new(move || {
            view! {
                <Link href=route class="search-link">
                    {label}
                </Link>
            }
        }),
        on_select: Producer::new(move || {}),
    }
}
