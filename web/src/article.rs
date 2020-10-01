use crate::{filters, render, WebError};
use askama::Template;
use conduit::{
    ArticleDto, ArticleService, CommentDto, CommentService, GetArticlesParams, PgPool, TagService,
    User,
};
use futures::try_join;
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

    let article = warp::path!("article" / String)
        .and(warp::get())
        .and(with_db(Arc::clone(&state)))
        .and(auth::optional(Arc::clone(&state)))
        .and_then(article_handler);

    home.or(article)
}

#[derive(Template)]
#[template(path = "home.html")]
struct HomeTemplate {
    tags: Vec<String>,
    user: Option<User>,
    articles: Vec<ArticleDto>,
}

#[derive(Template)]
#[template(path = "article.html")]
struct ArticleTemplate {
    user: Option<User>,
    article: ArticleDto,
    comments: Vec<CommentDto>,
}

async fn home_handler(
    params: GetArticlesParams,
    db_pool: PgPool,
    user: Option<User>,
) -> Result<impl Reply, Rejection> {
    let tag_service = TagService::new(&db_pool);
    let article_service = ArticleService::new(&db_pool);

    let (tags, articles) = try_join!(
        tag_service.get_tags(),
        article_service.get_articles(user.as_ref().map(|user| user.id), &params)
    )
    .map_err(WebError::from)?;

    render(&HomeTemplate {
        tags,
        user,
        articles,
    })
}

async fn article_handler(
    slug: String,
    db_pool: PgPool,
    user: Option<User>,
) -> Result<impl Reply, Rejection> {
    let current_user_id = user.as_ref().map(|user| user.id);
    let article_service = ArticleService::new(&db_pool);
    let comment_service = CommentService::new(&db_pool);

    let (article, comments) = try_join!(
        article_service.get_article(&slug, current_user_id),
        comment_service.get_comments(&slug, current_user_id)
    )
    .map_err(WebError::from)?;

    render(&ArticleTemplate {
        user,
        article,
        comments,
    })
}
