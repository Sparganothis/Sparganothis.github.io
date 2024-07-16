pub use axum::{
    body::Body as AxumBody,
    extract::{Path, State},
    http::Request,
    response::{Html, IntoResponse, Response},
    routing::{get, post},
    Router,
};
// pub use leptos::*;
// pub use leptos_axum::{generate_route_list, LeptosRoutes};
use std::net::SocketAddr;
use tower_http::trace::{DefaultMakeSpan, TraceLayer};

pub async fn server_main() {
    log::info!("server init...");
    let addr = "0.0.0.0:3000".to_string();

    // let _conn = db().await.expect("couldn't connect to DB");

    // Setting this to None means we'll be using cargo-leptos and its env vars
    // let conf = get_configuration(None).await.unwrap();
    // let leptos_options = conf.leptos_options;
    // let addr = leptos_options.site_addr;

    // build our application with a route
    let app = Router::new()
        // server function handlers are normally set up by .leptos_routes()
        // here, we're not actually doing server side rendering, so we set up a manual
        // handler for the server fns
        // this should include a get() handler if you have any GetUrl-based server fns
        // .route("/api/*fn_name", post(leptos_axum::handle_server_fns))
        .route("/api/ws", get(crate::backend::websocket::ws_handler))
        // .fallback(file_or_index_handler)
        // .with_state(leptos_options)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::default().include_headers(true)),
        )
        .layer(super::session::make_session_layer());

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("couldn't bind to address");
    log::info!("===>> server listening on http://{}", &addr);
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}
