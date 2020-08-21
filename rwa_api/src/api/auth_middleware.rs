use crate::db::User;
use crate::State;
use sqlx::query_as;
use tide::http::headers;
use tide::{Middleware, Next, Request, Response, Result, StatusCode};

pub struct AuthMiddleware;

#[async_trait::async_trait]
impl Middleware<State> for AuthMiddleware {
    async fn handle(&self, mut req: Request<State>, next: Next<'_, State>) -> Result {
        let state = req.state();

        if let Some(auth_header_values) = req.header(headers::AUTHORIZATION) {
            let auth_header = auth_header_values
                .as_str()
                .split_whitespace()
                .last()
                .unwrap();
            if let Ok(claims) = state.jwt.verify(auth_header) {
                let user = query_as!(User, "SELECT * FROM users WHERE id = $1", claims.data.id)
                    .fetch_one(&state.db_pool)
                    .await?;

                req.set_ext(user);

                let res = next.run(req).await;
                return Ok(res);
            }
        }

        Ok(Response::new(StatusCode::Unauthorized))
    }
}
