use crate::{filters, render, WebError};
use askama::Template;
use conduit::{
    ArticleDto, ArticleService, CommentDto, CommentService, GetArticlesParams, PgPool, TagService,
    User,
};
use futures::try_join;
use server::{auth, warp, with_db, Either, ServerState};
use std::sync::Arc;
use warp::{http, Filter, Rejection, Reply};

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

    let editor_new = warp::path!("editor")
        .and(warp::get())
        .and(auth::optional(Arc::clone(&state)))
        .and_then(editor_new_handler);

    let editor_existing = warp::path!("editor" / String)
        .and(warp::get())
        .and(with_db(Arc::clone(&state)))
        .and(auth::optional(Arc::clone(&state)))
        .and_then(editor_existing_handler);

    home.or(article).or(editor_new).or(editor_existing)
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

#[derive(Template)]
#[template(path = "article.html")]
struct ArticleTemplate {
    user: Option<User>,
    article: ArticleDto,
    comments: Vec<CommentDto>,
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

#[derive(Template)]
#[template(path = "editor.html")]
struct EditorTemplate {
    user: Option<User>,
    article: Option<ArticleDto>,
}

impl EditorTemplate {
    fn title(&self) -> &str {
        self.article
            .as_ref()
            .map(|a| a.title.as_ref())
            .unwrap_or("")
    }
    fn description(&self) -> &str {
        self.article
            .as_ref()
            .map(|a| a.description.as_ref())
            .unwrap_or("")
    }
    fn body(&self) -> &str {
        self.article.as_ref().map(|a| a.body.as_ref()).unwrap_or("")
    }
    fn slug(&self) -> String {
        self.article
            .as_ref()
            .map(|a| format!("/{}", a.slug))
            .unwrap_or_default()
    }
    fn method(&self) -> &str {
        self.article.as_ref().map(|_| "PUT").unwrap_or("POST")
    }
}

async fn editor_new_handler(user: Option<User>) -> Result<impl Reply, Rejection> {
    Ok(if let Some(user) = user {
        Either::Left(render(&EditorTemplate {
            user: Some(user),
            article: None,
        })?)
    } else {
        Either::Right(warp::redirect::temporary(http::Uri::from_static("/")))
    })
}

async fn editor_existing_handler(
    slug: String,
    db_pool: PgPool,
    user: Option<User>,
) -> Result<impl Reply, Rejection> {
    let article = ArticleService::new(&db_pool)
        .get_article(&slug, None)
        .await
        .map_err(WebError::from)?;

    let is_author = user
        .as_ref()
        .filter(|user| user.username == article.author.username)
        .is_some();
    Ok(if is_author {
        Either::Left(render(&EditorTemplate {
            user,
            article: Some(article),
        })?)
    } else {
        Either::Right(warp::redirect::temporary(http::Uri::from_static("/editor")))
    })
}
