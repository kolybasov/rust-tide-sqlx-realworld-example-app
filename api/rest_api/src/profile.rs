use crate::State;
use conduit::{ProfileDto, ProfileService, User};
use serde::Serialize;
use tide::{Body, Request, Result};

#[derive(Serialize, Debug)]
pub struct ProfileResponse {
    pub profile: ProfileDto,
}

impl From<ProfileDto> for ProfileResponse {
    fn from(profile: ProfileDto) -> Self {
        ProfileResponse { profile }
    }
}

pub async fn get_profile(req: Request<State>) -> Result {
    let state = req.state();
    let username: String = req.param("username")?;
    let current_user_id = req.ext::<User>().map(|user| user.id).or(None);

    let profile = ProfileService::new(&state.db_pool)
        .get_profile(&username, current_user_id)
        .await?;

    let res = ProfileResponse::from(profile);
    Ok(Body::from_json(&res)?.into())
}

pub async fn follow_profile(req: Request<State>) -> Result {
    let state = req.state();
    let username: String = req.param("username")?;
    let current_user_id = req.ext::<User>().unwrap().id;

    let profile = ProfileService::new(&state.db_pool)
        .follow_profile(&username, current_user_id)
        .await?;

    let res = ProfileResponse::from(profile);
    Ok(Body::from_json(&res)?.into())
}

pub async fn unfollow_profile(req: Request<State>) -> Result {
    let state = req.state();
    let username: String = req.param("username")?;
    let current_user_id = req.ext::<User>().unwrap().id;

    let profile = ProfileService::new(&state.db_pool)
        .unfollow_profile(&username, current_user_id)
        .await?;

    let res = ProfileResponse::from(profile);
    Ok(Body::from_json(&res)?.into())
}
