use crate::services::{ProfileDto, ProfileService, User};
use crate::State;
use serde::Serialize;
use std::convert::TryFrom;
use tide::{Body, Request, Response, Result, StatusCode};

#[derive(Serialize, Debug)]
pub struct ProfileResponse {
    pub profile: ProfileDto,
}

impl TryFrom<ProfileDto> for Body {
    type Error = tide::Error;

    fn try_from(profile: ProfileDto) -> Result<Self> {
        let res = ProfileResponse { profile };
        Body::from_json(&res)
    }
}

pub async fn get_profile(req: Request<State>) -> Result {
    let state = req.state();
    let username: String = req.param("username")?;
    let current_user_id = req.ext::<User>().map(|user| user.id).or(None);

    let profile = ProfileService::new(&state.db_pool)
        .get_profile(&username, current_user_id)
        .await?;

    Ok(Response::builder(StatusCode::Ok)
        .body(Body::try_from(profile)?)
        .build())
}

pub async fn follow_profile(req: Request<State>) -> Result {
    let state = req.state();
    let username: String = req.param("username")?;
    let current_user_id = req.ext::<User>().unwrap().id;

    let profile = ProfileService::new(&state.db_pool)
        .follow_profile(&username, current_user_id)
        .await?;

    Ok(Response::builder(StatusCode::Ok)
        .body(Body::try_from(profile)?)
        .build())
}

pub async fn unfollow_profile(req: Request<State>) -> Result {
    let state = req.state();
    let username: String = req.param("username")?;
    let current_user_id = req.ext::<User>().unwrap().id;

    let profile = ProfileService::new(&state.db_pool)
        .unfollow_profile(&username, current_user_id)
        .await?;

    Ok(Response::builder(StatusCode::Ok)
        .body(Body::try_from(profile)?)
        .build())
}
