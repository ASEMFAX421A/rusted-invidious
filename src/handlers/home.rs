use std::sync::Arc;
use askama::langid;
use tokio::sync::Mutex;
use axum::{Extension, response::Response, http::Request, body::Body};
use crate::{config::State, structs::template_context::TemplateContext};
use askama::Template;
askama::localization!(LOCALES);
use super::{templates::base::Base, utils::render};
/// Handler for the root path.
/// Ideally this should be redirect the user to the configured home path
/// e.g. /feed/popular or serve search page I guess but this could be changed
pub async fn index(Extension(state): Extension<Arc<Mutex<State>>>,request: Request<Body>)-> Response {
    let lock = state.lock().await;
    // TODO: implement
    let base = Base{
        loc: askama::Locale::new(langid!("en-US"), &LOCALES),
        context: TemplateContext::new(&request, None, &lock.config),
    };
    render(base.render())
}

/// Handler for the /feed/popular path.
pub async fn popular(Extension(state): Extension<Arc<Mutex<State>>>) -> Response {
    todo!()
}

/// Handler for the /feed/trending path.
pub async fn trending(Extension(state): Extension<Arc<Mutex<State>>>) -> Response {
    todo!()
}