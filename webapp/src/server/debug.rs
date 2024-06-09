use leptos::*;

#[server]
pub async fn git_version() -> Result<String, ServerFnError> {
    // this is just an example of how to access server context injected in the handlers
    let req_parts = use_context::<http::request::Parts>();

    if let Some(req_parts) = req_parts {
        println!("git_version() uri={:?}", req_parts.uri);
    }

    Ok("something2".to_string())
}