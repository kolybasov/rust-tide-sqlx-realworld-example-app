use conduit::ConduitError;
use serde::Serialize;
use server::{warp, ServerError};
use std::collections::HashMap;
use thiserror::Error;
use warp::{http::StatusCode, Rejection, Reply};

#[derive(Error, Debug)]
pub enum RestError {
    #[error(transparent)]
    ConduitError(#[from] ConduitError),
    #[error(transparent)]
    ServerError(#[from] ServerError),
}

impl warp::reject::Reject for RestError {}

impl From<RestError> for warp::Rejection {
    fn from(err: RestError) -> Self {
        warp::reject::custom(err)
    }
}

#[derive(Debug, Serialize)]
pub struct ErrorMessage<'a> {
    code: u16,
    message: &'a str,
    errors: Option<HashMap<&'static str, Vec<String>>>,
}

pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, std::convert::Infallible> {
    let mut code = StatusCode::INTERNAL_SERVER_ERROR;
    let mut message = "INTERNAL_SERVER_ERROR";
    let mut errors = None;
    let error_string;

    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = "NOT_FOUND";
    } else if let Some(rest_err) = err.find::<RestError>() {
        match rest_err {
            RestError::ConduitError(conduit_err) => match conduit_err {
                ConduitError::ValidationError(validation_err) => {
                    code = StatusCode::BAD_REQUEST;
                    message = "BAD_REQUEST";
                    errors = Some(
                        validation_err
                            .field_errors()
                            .iter()
                            .map(|(field, errors)| {
                                (
                                    *field,
                                    errors
                                        .into_iter()
                                        .map(|err| {
                                            err.message
                                                .as_ref()
                                                .map(|err| err.to_string())
                                                .unwrap_or(format!("Invalid {}", err.code))
                                        })
                                        .collect(),
                                )
                            })
                            .collect(),
                    );
                }
                ConduitError::InvalidPassword => {
                    code = StatusCode::UNAUTHORIZED;
                    message = "UNAUTHORIZED";
                }
                _ => {
                    error_string = conduit_err.to_string();
                    message = error_string.as_str();
                }
            },
            RestError::ServerError(server_err) => match server_err {
                ServerError::Unauthorized => {
                    code = StatusCode::UNAUTHORIZED;
                    message = "UNAUTHORIZED";
                }
                _ => {
                    error_string = server_err.to_string();
                    message = error_string.as_str();
                }
            },
        }
    }

    let body = warp::reply::json(&ErrorMessage {
        code: code.as_u16(),
        message,
        errors,
    });

    Ok(warp::reply::with_status(body, code))
}
