pub use super::fallback::file_or_index_handler;
pub use axum::{
    body::Body as AxumBody,
    extract::{Path, State},
    http::Request,
    response::{Html, IntoResponse, Response},
    routing::{get, post},
    Router,
};
pub use leptos::*;
pub use leptos_axum::{generate_route_list, LeptosRoutes};

pub async fn server_main() {
    simple_logger::init_with_level(log::Level::Debug).expect("couldn't initialize logging");

    // let _conn = db().await.expect("couldn't connect to DB");

    // Setting this to None means we'll be using cargo-leptos and its env vars
    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;

    // build our application with a route
    let app = Router::new()
        // server function handlers are normally set up by .leptos_routes()
        // here, we're not actually doing server side rendering, so we set up a manual
        // handler for the server fns
        // this should include a get() handler if you have any GetUrl-based server fns
        .route("/api/*fn_name", post(leptos_axum::handle_server_fns))
        .fallback(file_or_index_handler)
        .with_state(leptos_options)
        .layer(super::session::make_session_layer());

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    logging::log!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("couldn't bind to address");
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
