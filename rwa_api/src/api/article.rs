use crate::db::{ArticleResponse, ArticlesResponse, User};
use crate::services::{
    ArticleService, CreateArticleParams, GetArticlesParams, PageOptions, UpdateArticleParams,
};
use crate::State;
use serde::Deserialize;
use tide::{Body, Error, Request, Response, Result, StatusCode};

#[derive(Debug, Deserialize)]
struct CreateArticlePayload {
    article: CreateArticleParams,
}

pub async fn create_article(mut req: Request<State>) -> Result {
    let payload: CreateArticlePayload = req.body_json().await?;
    let state = req.state();
    let author = req.ext::<User>().ok_or(Error::from_str(
        StatusCode::Unauthorized,
        "No user provided",
    ))?;

    let article = ArticleService::new(&state.db_pool)
        .create_article(&payload.article, author.id)
        .await?;

    let body = Body::from_json(&ArticleResponse::from(article))?;
    Ok(Response::builder(StatusCode::Ok).body(body).build())
}

pub async fn delete_article(req: Request<State>) -> Result {
    let state = req.state();
    let current_user_id = req.ext::<User>().unwrap().id;
    let slug: String = req.param("slug")?;

    ArticleService::new(&state.db_pool)
        .delete_article(&slug, current_user_id)
        .await?;

    Ok(Response::new(StatusCode::NoContent))
}

pub async fn favorite_article(req: Request<State>) -> Result {
    let state = req.state();
    let slug: String = req.param("slug")?;
    let current_user_id = req.ext::<User>().unwrap().id;

    let article = ArticleService::new(&state.db_pool)
        .favorite_article(&slug, current_user_id)
        .await?;

    let body = Body::from_json(&ArticleResponse::from(article))?;
    Ok(Response::builder(StatusCode::Ok).body(body).build())
}

pub async fn feed(req: Request<State>) -> Result {
    let state = req.state();
    let current_user_id = req.ext::<User>().map(|user| user.id).or(None);
    let params: PageOptions = req.query()?;

    let articles = ArticleService::new(&state.db_pool)
        .get_articles(
            current_user_id,
            &GetArticlesParams::default()
                .limit(params.limit)
                .offset(params.offset),
        )
        .await?;

    let body = Body::from_json(&ArticlesResponse::from(articles))?;
    Ok(Response::builder(StatusCode::Ok).body(body).build())
}

pub async fn get_article(req: Request<State>) -> Result {
    let state = req.state();
    let current_user_id = req.ext::<User>().map(|user| user.id).or(None);
    let slug: String = req.param("slug")?;

    let article = ArticleService::new(&state.db_pool)
        .get_article(&slug, current_user_id)
        .await?;
    let body = Body::from_json(&ArticleResponse::from(article))?;
    Ok(Response::builder(StatusCode::Ok).body(body).build())
}

pub async fn get_articles(req: Request<State>) -> Result {
    let state = req.state();
    let current_user_id = req.ext::<User>().map(|user| user.id).or(None);
    let params: GetArticlesParams = req.query()?;

    let articles = ArticleService::new(&state.db_pool)
        .get_articles(current_user_id, &params)
        .await?;

    let body = Body::from_json(&ArticlesResponse::from(articles))?;
    Ok(Response::builder(StatusCode::Ok).body(body).build())
}

pub async fn unfavorite_article(req: Request<State>) -> Result {
    let state = req.state();
    let slug: String = req.param("slug")?;
    let current_user_id = req.ext::<User>().unwrap().id;

    let article = ArticleService::new(&state.db_pool)
        .unfavorite_article(&slug, current_user_id)
        .await?;

    let body = Body::from_json(&ArticleResponse::from(article))?;
    Ok(Response::builder(StatusCode::Ok).body(body).build())
}

#[derive(Debug, Deserialize)]
struct UpdateArticlePayload {
    article: UpdateArticleParams,
}

pub async fn update_article(mut req: Request<State>) -> Result {
    let payload: UpdateArticlePayload = req.body_json().await?;
    let state = req.state();
    let slug: String = req.param("slug")?;
    let author_id = req.ext::<User>().unwrap().id;

    let article = ArticleService::new(&state.db_pool)
        .update_article(&slug, author_id, &payload.article)
        .await?;

    let body = Body::from_json(&ArticleResponse::from(article))?;
    Ok(Response::builder(StatusCode::Ok).body(body).build())
}
