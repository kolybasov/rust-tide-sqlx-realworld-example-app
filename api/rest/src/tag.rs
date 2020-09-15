use crate::{filters::state::with_db, WarpState};
use conduit::{PgPool, TagService};
use serde::Serialize;
use warp::{Filter, Rejection, Reply};

pub fn routes(
    state: WarpState,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    // GET /tags
    warp::path!("tags")
        .and(warp::get())
        .and(with_db(state))
        .and_then(get_tags_handler)
}

pub async fn get_tags_handler(db_pool: PgPool) -> Result<impl Reply, Rejection> {
    let tags = TagService::new(&db_pool).get_tags().await?;
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
