use crate::api::{
    create_article, create_comment, delete_article, delete_comment, favorite_article, feed,
    follow_profile, get_article, get_articles, get_comments, get_profile, get_tags, get_user,
    login, register, unfavorite_article, unfollow_profile, update_article, update_user,
    AuthMiddleware,
};
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
        app.at("/users").post(register);

        app.at("/users/login").post(login);

        app.at("/user")
            .with(AuthMiddleware::required())
            .get(get_user)
            .put(update_user);

        // Profiles
        app.at("/profiles/:username")
            .with(AuthMiddleware::optional())
            .get(get_profile);

        app.at("/profiles/:username/follow")
            .with(AuthMiddleware::required())
            .post(follow_profile)
            .delete(unfollow_profile);

        // Articles
        app.at("/articles")
            .with(AuthMiddleware::optional())
            .get(get_articles);

        app.at("/articles")
            .with(AuthMiddleware::required())
            .post(create_article);

        app.at("/articles/feed")
            .with(AuthMiddleware::required())
            .get(feed);

        app.at("/articles/:slug")
            .with(AuthMiddleware::optional())
            .get(get_article);

        app.at("/articles/:slug")
            .with(AuthMiddleware::required())
            .put(update_article)
            .delete(delete_article);

        app.at("/articles/:slug/favorite")
            .with(AuthMiddleware::required())
            .post(favorite_article)
            .delete(unfavorite_article);

        // Comments
        app.at("/articles/:slug/comments")
            .with(AuthMiddleware::required())
            .post(create_comment);

        app.at("/articles/:slug/comments")
            .with(AuthMiddleware::optional())
            .get(get_comments);

        app.at("/articles/:slug/comments/:id")
            .with(AuthMiddleware::required())
            .delete(delete_comment);

        // Tags
        app.at("/tags").get(get_tags);

        app
    }
}
