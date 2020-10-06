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

    // POST /logout
    let logout = warp::path!("logout")
        .and(warp::post())
        .and(auth(Arc::clone(&state)))
        .and_then(logout_handler);

    // GET /register
    let register = warp::path!("register")
        .and(warp::get())
        .and(auth::optional(Arc::clone(&state)))
        .and_then(register_handler);

    // GET /settings
    let settings = warp::path!("settings")
        .and(warp::get())
        .and(auth::optional(Arc::clone(&state)))
        .and_then(settings_handler);

    login.or(register).or(settings).or(logout)
}

#[derive(Template, Default)]
#[template(path = "login.html")]
struct LoginTemplate {
    user: Option<User>,
}

pub async fn login_handler(user: Option<User>) -> Result<impl Reply, Rejection> {
    Ok(if user.is_some() {
        Either::Left(warp::redirect::temporary(http::Uri::from_static("/")))
    } else {
        Either::Right(render(&LoginTemplate::default())?)
    })
}

pub async fn logout_handler(_user: User) -> Result<impl Reply, Rejection> {
    let set_cookie = auth::delete_cookie_token();
    Ok(warp::reply::with_header(
        warp::reply::with_header(
            warp::reply::with_status(warp::reply(), warp::http::StatusCode::SEE_OTHER),
            warp::http::header::LOCATION,
            "/",
        ),
        warp::http::header::SET_COOKIE,
        set_cookie,
    ))
}

#[derive(Template, Default)]
#[template(path = "register.html")]
struct RegisterTemplate {
    user: Option<User>,
}

pub async fn register_handler(user: Option<User>) -> Result<impl Reply, Rejection> {
    Ok(if user.is_some() {
        Either::Left(warp::redirect::temporary(http::Uri::from_static("/")))
    } else {
        Either::Right(render(&RegisterTemplate::default())?)
    })
}

#[derive(Template)]
#[template(path = "settings.html")]
struct SettingsTemplate {
    user: Option<User>,
}

impl SettingsTemplate {
    fn image(&self) -> &str {
        self.user
            .as_ref()
            .and_then(|u| u.image.as_deref())
            .unwrap_or_default()
    }
    fn bio(&self) -> &str {
        self.user
            .as_ref()
            .and_then(|u| u.bio.as_deref())
            .unwrap_or_default()
    }
    fn username(&self) -> &str {
        self.user
            .as_ref()
            .map(|u| u.username.as_ref())
            .unwrap_or_default()
    }
    fn email(&self) -> &str {
        self.user
            .as_ref()
            .map(|u| u.email.as_ref())
            .unwrap_or_default()
    }
}

pub async fn settings_handler(user: Option<User>) -> Result<impl Reply, Rejection> {
    Ok(if user.is_some() {
        Either::Left(render(&SettingsTemplate { user })?)
    } else {
        Either::Right(warp::redirect::temporary(http::Uri::from_static("/")))
    })
}
