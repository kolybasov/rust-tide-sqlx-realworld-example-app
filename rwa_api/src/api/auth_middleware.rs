use crate::db::User;
use crate::State;
use sqlx::query_as;
use tide::http::headers;
use tide::{Error, Middleware, Next, Request, Result, StatusCode};

pub struct AuthMiddleware {
    auth_required: bool,
}

impl AuthMiddleware {
    pub fn required() -> Self {
        AuthMiddleware {
            auth_required: true,
        }
    }

    pub fn optional() -> Self {
        AuthMiddleware {
            auth_required: false,
        }
    }
}

#[async_trait::async_trait]
impl Middleware<State> for AuthMiddleware {
    async fn handle(&self, mut req: Request<State>, next: Next<'_, State>) -> Result {
        let state = req.state();

        if let Some(auth_header_values) = req.header(headers::AUTHORIZATION) {
            let auth_header = auth_header_values
                .as_str()
                .split_whitespace()
                .last()
                .ok_or(Error::from_str(StatusCode::Unauthorized, "No auth token"))?;

            if let Ok(claims) = state.jwt.verify(auth_header) {
                let user = query_as!(User, "SELECT * FROM users WHERE id = $1", claims.data.id)
                    .fetch_one(&state.db_pool)
                    .await?;

                req.set_ext(user);

                return Ok(next.run(req).await);
            }
        }

        if self.auth_required {
            Err(Error::from_str(StatusCode::Unauthorized, "Unauthorized"))
        } else {
            Ok(next.run(req).await)
        }
    }
}
