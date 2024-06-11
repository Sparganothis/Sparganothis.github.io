use leptos::*;

#[server]
pub async fn git_version() -> Result<String, ServerFnError> {
    use crate::server::backend::server_info::GIT_VERSION;

    Ok(GIT_VERSION.clone())
}
