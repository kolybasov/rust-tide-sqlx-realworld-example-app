use crate::RestError;
use conduit::{PgPool, TagService};
use serde::Serialize;
use server::{warp, with_db, ServerState};
use warp::{Filter, Rejection, Reply};

pub fn routes(
    state: ServerState,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    // GET /tags
    let get_tags = warp::path!("tags")
        .and(warp::get())
        .and(with_db(state))
        .and_then(get_tags_handler)
        .boxed();

    get_tags
}

pub async fn get_tags_handler(db_pool: PgPool) -> Result<impl Reply, Rejection> {
    let tags = TagService::new(&db_pool)
        .get_tags()
        .await
        .map_err(RestError::from)?;
    Ok(warp::reply::json(&TagsResponse::from(tags)))
}

#[derive(Debug, Serialize)]
pub struct TagsResponse {
    pub tags: Vec<String>,
}

impl From<Vec<String>> for TagsResponse {
    fn from(tags: Vec<String>) -> Self {
        TagsResponse { tags }
    }
}
