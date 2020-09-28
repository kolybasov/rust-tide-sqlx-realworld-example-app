pub use askama::Error;
use askama::Template;
use conduit::{ArticleDto, ArticleService, GetArticlesParams, PgPool, TagService, User};
use server::{auth, warp, with_db, ServerState};
use std::sync::Arc;
use warp::{Filter, Rejection, Reply};

#[derive(Template)]
#[template(path = "home.html")]
struct HelloTemplate<'a> {
    tags: &'a Vec<String>,
    user: Option<&'a User>,
    articles: &'a Vec<ArticleDto>,
}

pub struct Web;

impl Web {
    pub fn new(state: ServerState) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
        let static_dir = warp::path("static").and(warp::fs::dir("web/static"));

        let home = warp::get()
            .and(warp::query())
            .and(with_db(Arc::clone(&state)))
            .and(auth::optional(state))
            .and_then(
                |params: GetArticlesParams, db_pool: PgPool, user: Option<User>| async move {
                    let tags = TagService::new(&db_pool).get_tags().await.unwrap();
                    let articles = ArticleService::new(&db_pool)
                        .get_articles(user.as_ref().map(|user| user.id), &params)
                        .await
                        .unwrap();

                    let html = HelloTemplate {
                        tags: &tags,
                        user: user.as_ref(),
                        articles: &articles,
                    }
                    .render()
                    .unwrap();
                    Ok::<_, Rejection>(warp::reply::html(html))
                },
            );

        static_dir.or(home)
    }
}
