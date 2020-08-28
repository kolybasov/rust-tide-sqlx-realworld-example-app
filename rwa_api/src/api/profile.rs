use crate::db::{ProfileResponse, User};
use crate::services::ProfileService;
use crate::State;
use tide::{Body, Request, Response, Result, StatusCode};

pub async fn get_profile(req: Request<State>) -> Result {
    let state = req.state();
    let username: String = req.param("username")?;
    let current_user_id = req.ext::<User>().map(|user| user.id).or(None);

    let profile = ProfileService::new(&state.db_pool)
        .get_profile(&username, current_user_id)
        .await?;

    let body = Body::from_json(&ProfileResponse::from(profile))?;
    Ok(Response::builder(StatusCode::Ok).body(body).build())
}

pub async fn follow_profile(req: Request<State>) -> Result {
    let state = req.state();
    let username: String = req.param("username")?;
    let current_user_id = req.ext::<User>().unwrap().id;

    let profile = ProfileService::new(&state.db_pool)
        .follow_profile(&username, current_user_id)
        .await?;

    let body = Body::from_json(&ProfileResponse::from(profile))?;
    Ok(Response::builder(StatusCode::Ok).body(body).build())
}

pub async fn unfollow_profile(req: Request<State>) -> Result {
    let state = req.state();
    let username: String = req.param("username")?;
    let current_user_id = req.ext::<User>().unwrap().id;

    let profile = ProfileService::new(&state.db_pool)
        .unfollow_profile(&username, current_user_id)
        .await?;

    let body = Body::from_json(&ProfileResponse::from(profile))?;
    Ok(Response::builder(StatusCode::Ok).body(body).build())
}
