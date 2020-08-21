use crate::db::{User, UserDto, UserResponse};
use crate::State;
use tide::{Body, Request, Response, Result, StatusCode};

pub async fn get_user(req: Request<State>) -> Result {
    let state = req.state();
    let user = req.ext::<User>().unwrap();

    let token = state.jwt.sign(user)?;
    let body = UserResponse {
        user: UserDto::with_token(user.clone(), token),
    };

    let mut res = Response::new(StatusCode::Ok);
    res.set_body(Body::from_json(&body)?);

    Ok(res)
}
