use conduit::{
    ArticleDto, ArticleService, CreateArticleParams, GetArticlesParams, PageOptions, PgPool,
    UpdateArticleParams, User,
};
use serde::{Deserialize, Serialize};
use server::{auth, warp, with_db, ServerState};
use warp::{Filter, Rejection, Reply};

pub fn routes(state: ServerState) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    // GET /articles
    let get_articles = warp::path!("articles")
        .and(warp::get())
        .and(warp::query())
        .and(auth::optional(state.clone()))
        .and(with_db(state.clone()))
        .and_then(get_articles_handler)
        .boxed();

    // POST /articles
    let create_article = warp::path!("articles")
        .and(warp::post())
        .and(warp::body::json())
        .and(auth(state.clone()))
        .and(with_db(state.clone()))
        .and_then(create_article_handler)
        .boxed();

    // GET /articles/feed
    let feed = warp::path!("articles" / "feed")
        .and(warp::get())
        .and(warp::query())
        .and(auth(state.clone()))
        .and(with_db(state.clone()))
        .and_then(feed_handler)
        .boxed();

    // GET /articles/:slug
    let get_article = warp::path!("articles" / String)
        .and(warp::get())
        .and(auth::optional(state.clone()))
        .and(with_db(state.clone()))
        .and_then(get_article_handler)
        .boxed();

    // PUT /articles/:slug
    let update_article = warp::path!("articles" / String)
        .and(warp::put())
        .and(warp::body::json())
        .and(auth(state.clone()))
        .and(with_db(state.clone()))
        .and_then(update_article_handler)
        .boxed();

    // DELETE /articles/:slug
    let delete_article = warp::path!("articles" / String)
        .and(warp::delete())
        .and(auth(state.clone()))
        .and(with_db(state.clone()))
        .and_then(delete_article_handler)
        .boxed();

    // POST /articles/:slug/favorite
    let favorite_article = warp::path!("articles" / String / "favorite")
        .and(warp::post())
        .and(auth(state.clone()))
        .and(with_db(state.clone()))
        .and_then(favorite_article_handler)
        .boxed();

    // DELETE /articles/:slug/favorite
    let unfavorite_article = warp::path!("articles" / String / "favorite")
        .and(warp::delete())
        .and(auth(state.clone()))
        .and(with_db(state.clone()))
        .and_then(unfavorite_article_handler)
        .boxed();

    get_articles
        .or(create_article)
        .or(feed)
        .or(get_article)
        .or(update_article)
        .or(delete_article)
        .or(favorite_article)
        .or(unfavorite_article)
        .boxed()
}

#[derive(Serialize, Debug)]
pub struct ArticleResponse {
    pub article: ArticleDto,
}

impl From<ArticleDto> for ArticleResponse {
    fn from(article: ArticleDto) -> Self {
        ArticleResponse { article }
    }
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ArticlesResponse {
    pub articles: Vec<ArticleDto>,
    pub articles_count: usize,
}

impl From<Vec<ArticleDto>> for ArticlesResponse {
    fn from(articles: Vec<ArticleDto>) -> Self {
        ArticlesResponse {
            articles_count: articles.len(),
            articles,
        }
    }
}

#[derive(Debug, Deserialize)]
struct CreateArticlePayload {
    article: CreateArticleParams,
}

async fn create_article_handler(
    payload: CreateArticlePayload,
    author: User,
    db_pool: PgPool,
) -> Result<impl Reply, Rejection> {
    let article = ArticleService::new(&db_pool)
        .create_article(&payload.article, author.id)
        .await?;

    Ok(warp::reply::with_status(
        warp::reply::json(&ArticleResponse::from(article)),
        warp::http::StatusCode::CREATED,
    ))
}

async fn delete_article_handler(
    slug: String,
    user: User,
    db_pool: PgPool,
) -> Result<impl Reply, Rejection> {
    ArticleService::new(&db_pool)
        .delete_article(&slug, user.id)
        .await?;

    Ok(warp::reply::with_status(
        warp::reply(),
        warp::http::StatusCode::NO_CONTENT,
    ))
}

async fn favorite_article_handler(
    slug: String,
    user: User,
    db_pool: PgPool,
) -> Result<impl Reply, Rejection> {
    let article = ArticleService::new(&db_pool)
        .favorite_article(&slug, user.id)
        .await?;

    Ok(warp::reply::json(&ArticleResponse::from(article)))
}

async fn feed_handler(
    params: PageOptions,
    user: User,
    db_pool: PgPool,
) -> Result<impl Reply, Rejection> {
    let articles = ArticleService::new(&db_pool)
        .get_articles(
            Some(user.id),
            &GetArticlesParams::default()
                .limit(params.limit)
                .offset(params.offset)
                .feed(Some(true)),
        )
        .await?;

    Ok(warp::reply::json(&ArticlesResponse::from(articles)))
}

async fn get_article_handler(
    slug: String,
    user: Option<User>,
    db_pool: PgPool,
) -> Result<impl Reply, Rejection> {
    let current_user_id = user.map(|user| user.id).or(None);

    let article = ArticleService::new(&db_pool)
        .get_article(&slug, current_user_id)
        .await?;

    Ok(warp::reply::json(&ArticleResponse::from(article)))
}

async fn get_articles_handler(
    params: GetArticlesParams,
    user: Option<User>,
    db_pool: PgPool,
) -> Result<impl Reply, Rejection> {
    let current_user_id = user.map(|user| user.id).or(None);

    let articles = ArticleService::new(&db_pool)
        .get_articles(current_user_id, &params)
        .await?;

    Ok(warp::reply::json(&ArticlesResponse::from(articles)))
}

async fn unfavorite_article_handler(
    slug: String,
    user: User,
    db_pool: PgPool,
) -> Result<impl Reply, Rejection> {
    let article = ArticleService::new(&db_pool)
        .unfavorite_article(&slug, user.id)
        .await?;

    Ok(warp::reply::json(&ArticleResponse::from(article)))
}

#[derive(Debug, Deserialize)]
struct UpdateArticlePayload {
    article: UpdateArticleParams,
}

async fn update_article_handler(
    slug: String,
    payload: UpdateArticlePayload,
    author: User,
    db_pool: PgPool,
) -> Result<impl Reply, Rejection> {
    let article = ArticleService::new(&db_pool)
        .update_article(&slug, author.id, &payload.article)
        .await?;

    Ok(warp::reply::json(&ArticleResponse::from(article)))
}
