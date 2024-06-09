use leptos::*;

#[server]
pub async fn git_version() -> Result<String, ServerFnError> {
    // this is just an example of how to access server context injected in the handlers
    // let req_parts = use_context::<http::request::Parts>();

    // if let Some(req_parts) = req_parts {
    //     println!("git_version() uri={:?} ver={:?}", req_parts.uri, GIT_VERSION.clone());
    // }
    use crate::server::backend::server_info::GIT_VERSION;

    Ok(GIT_VERSION.clone())
}
