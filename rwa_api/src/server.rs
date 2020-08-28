use crate::api::{article, comment, profile, tag, user, AuthMiddleware};
use sqlx::{Pool, Postgres};
use tide::{http::headers::HeaderValue, security::CorsMiddleware};

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

        /* Middlewares */
        app.with(CorsMiddleware::new().allow_methods("*".parse::<HeaderValue>().unwrap()));

        /* Routes */
        // Users
        app.at("/users").post(user::register);

        app.at("/users/login").post(user::login);

        app.at("/user")
            .with(AuthMiddleware::required())
            .get(user::get_user)
            .put(user::update_user);

        // Profiles
        app.at("/profiles/:username")
            .with(AuthMiddleware::optional())
            .get(profile::get_profile);

        app.at("/profiles/:username/follow")
            .with(AuthMiddleware::required())
            .post(profile::follow_profile)
            .delete(profile::unfollow_profile);

        // Articles
        app.at("/articles")
            .with(AuthMiddleware::optional())
            .get(article::get_articles);

        app.at("/articles")
            .with(AuthMiddleware::required())
            .post(article::create_article);

        app.at("/articles/feed")
            .with(AuthMiddleware::required())
            .get(article::feed);

        app.at("/articles/:slug")
            .with(AuthMiddleware::optional())
            .get(article::get_article);

        app.at("/articles/:slug")
            .with(AuthMiddleware::required())
            .put(article::update_article)
            .delete(article::delete_article);

        app.at("/articles/:slug/favorite")
            .with(AuthMiddleware::required())
            .post(article::favorite_article)
            .delete(article::unfavorite_article);

        // Comments
        app.at("/articles/:slug/comments")
            .with(AuthMiddleware::required())
            .post(comment::create_comment);

        app.at("/articles/:slug/comments")
            .with(AuthMiddleware::optional())
            .get(comment::get_comments);

        app.at("/articles/:slug/comments/:id")
            .with(AuthMiddleware::required())
            .delete(comment::delete_comment);

        // Tags
        app.at("/tags").get(tag::get_tags);

        app
    }
}
