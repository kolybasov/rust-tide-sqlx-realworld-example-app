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

    // GET /register
    let register = warp::path!("register")
        .and(warp::get())
        .and(auth::optional(Arc::clone(&state)))
        .and_then(register_handler);

    login.or(register)
}

#[derive(Template, Default)]
#[template(path = "login.html")]
struct LoginTemplate {
    user: Option<User>,
}

#[derive(Template, Default)]
#[template(path = "register.html")]
struct RegisterTemplate {
    user: Option<User>,
}

pub async fn login_handler(user: Option<User>) -> Result<impl Reply, Rejection> {
    Ok(if user.is_some() {
        Either::Left(warp::redirect::temporary(http::Uri::from_static("/")))
    } else {
        Either::Right(render(&LoginTemplate::default())?)
    })
}

pub async fn register_handler(user: Option<User>) -> Result<impl Reply, Rejection> {
    Ok(if user.is_some() {
        Either::Left(warp::redirect::temporary(http::Uri::from_static("/")))
    } else {
        Either::Right(render(&RegisterTemplate::default())?)
    })
}
