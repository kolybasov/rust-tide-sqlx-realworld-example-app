use crate::api::{get_user, login, register, update_user, AuthMiddleware};
use sqlx::{Pool, Postgres};
use tide::security::CorsMiddleware;

#[derive(Clone)]
pub struct State {
    pub db_pool: Pool<Postgres>,
    pub config: crate::Config,
    pub jwt: crate::jwt::JWT,
}

pub struct Server;

impl Server {
    pub fn new(state: State) -> tide::Server<State> {
        tide::log::start();

        let mut app = tide::with_state(state);

        // Middlewares
        app.with(CorsMiddleware::new());

        // Routes
        app.at("/users").post(register);
        app.at("/users/login").post(login);
        app.at("/user")
            .with(AuthMiddleware)
            .get(get_user)
            .put(update_user);

        app
    }
}
