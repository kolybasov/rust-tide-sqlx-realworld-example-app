use crate::{render, WebError};
use askama::Template;
use conduit::{ArticleDto, ArticleService, GetArticlesParams, PgPool, TagService, User};
use server::{auth, warp, with_db, ServerState};
use std::sync::Arc;
use warp::{Filter, Rejection, Reply};

pub fn routes(state: ServerState) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    // GET /
    let home = warp::path::end()
        .and(warp::get())
        .and(warp::query())
        .and(with_db(Arc::clone(&state)))
        .and(auth::optional(Arc::clone(&state)))
        .and_then(home_handler);

    home
}

#[derive(Template)]
#[template(path = "home.html")]
struct HomeTemplate {
    tags: Vec<String>,
    user: Option<User>,
    articles: Vec<ArticleDto>,
}

async fn home_handler(
    params: GetArticlesParams,
    db_pool: PgPool,
    user: Option<User>,
) -> Result<impl Reply, Rejection> {
    let tags = TagService::new(&db_pool)
        .get_tags()
        .await
        .map_err(WebError::from)?;
    let articles = ArticleService::new(&db_pool)
        .get_articles(user.as_ref().map(|user| user.id), &params)
        .await
        .map_err(WebError::from)?;

    render(&HomeTemplate {
        tags,
        user,
        articles,
    })
}
