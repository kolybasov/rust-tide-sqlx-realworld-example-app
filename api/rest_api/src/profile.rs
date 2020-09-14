use crate::{
    filters,
    filters::state::{with_db, PgPool, WarpState},
};
use conduit::{ProfileDto, ProfileService, User};
use serde::Serialize;
use warp::{Filter, Rejection, Reply};

pub fn routes(state: WarpState) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    // GET /profiles/:username
    let get_profile = warp::path!("profiles" / String)
        .and(warp::get())
        .and(filters::auth::optional(state.clone()))
        .and(with_db(state.clone()))
        .and_then(get_profile_handler);

    // POST /profiles/:username/follow
    let follow_profile = warp::path!("profiles" / String / "follow")
        .and(warp::post())
        .and(filters::auth(state.clone()))
        .and(with_db(state.clone()))
        .and_then(follow_profile_handler);

    // DELETE /profiles/:username/follow
    let unfollow_profile = warp::path!("profiles" / String / "follow")
        .and(warp::delete())
        .and(filters::auth(state.clone()))
        .and(with_db(state.clone()))
        .and_then(unfollow_profile_handler);

    get_profile.or(follow_profile).or(unfollow_profile)
}

#[derive(Serialize, Debug)]
pub struct ProfileResponse {
    pub profile: ProfileDto,
}

impl From<ProfileDto> for ProfileResponse {
    fn from(profile: ProfileDto) -> Self {
        ProfileResponse { profile }
    }
}

async fn get_profile_handler(
    username: String,
    user: Option<User>,
    db_pool: PgPool,
) -> Result<impl Reply, Rejection> {
    let current_user_id = user.map(|user| user.id).or(None);

    let profile = ProfileService::new(&db_pool)
        .get_profile(&username, current_user_id)
        .await?;

    Ok(warp::reply::json(&ProfileResponse::from(profile)))
}

async fn follow_profile_handler(
    username: String,
    user: User,
    db_pool: PgPool,
) -> Result<impl Reply, Rejection> {
    let profile = ProfileService::new(&db_pool)
        .follow_profile(&username, user.id)
        .await?;

    Ok(warp::reply::json(&ProfileResponse::from(profile)))
}

async fn unfollow_profile_handler(
    username: String,
    user: User,
    db_pool: PgPool,
) -> Result<impl Reply, Rejection> {
    let profile = ProfileService::new(&db_pool)
        .unfollow_profile(&username, user.id)
        .await?;

    Ok(warp::reply::json(&ProfileResponse::from(profile)))
}
