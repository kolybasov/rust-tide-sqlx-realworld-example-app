use crate::render;
use askama::Template;
use conduit::User;
use server::{auth, warp, Either, ServerState};
use std::sync::Arc;
use warp::{http, Filter, Rejection, Reply};

pub fn routes(state: ServerState) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    // GET /login
    let login = warp::path!("login")
        .and(warp::get())
        .and(auth::optional(Arc::clone(&state)))
        .and_then(login_handler);

    login
}

#[derive(Template, Default)]
#[template(path = "login.html")]
struct LoginTemplate {
    user: Option<User>,
}

pub async fn login_handler(user: Option<User>) -> Result<impl Reply, Rejection> {
    Ok(if let Some(_) = user {
        Either::Left(warp::redirect::temporary(http::Uri::from_static("/")))
    } else {
        Either::Right(render(&LoginTemplate::default())?)
    })
}
