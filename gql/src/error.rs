use conduit::ConduitError;
use juniper::{DefaultScalarValue, FieldError, IntoFieldError, Object, Value};
use server::{warp, ServerError};
use thiserror::Error;
use warp::http::StatusCode;

#[derive(Error, Debug)]
pub enum GqlError {
    #[error(transparent)]
    ConduitError(#[from] ConduitError),
    #[error(transparent)]
    ServerError(#[from] ServerError),
}

impl warp::reject::Reject for GqlError {}

pub type Result<T> = std::result::Result<T, GqlError>;

impl IntoFieldError for GqlError {
    fn into_field_error(self) -> FieldError {
        let mut extensions = Object::with_capacity(2);
        let mut code = StatusCode::INTERNAL_SERVER_ERROR;
        let mut errors: Option<Object<DefaultScalarValue>> = None;
        let message;
        let error_string;

        match self {
            GqlError::ConduitError(conduit_err) => match conduit_err {
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
                                    Value::list(
                                        errors
                                            .iter()
                                            .map(|err| {
                                                Value::scalar(
                                                    err.message
                                                        .as_ref()
                                                        .map(|err| err.to_string())
                                                        .unwrap_or(format!("Invalid {}", err.code)),
                                                )
                                            })
                                            .collect(),
                                    ),
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
            GqlError::ServerError(server_err) => match server_err {
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

        extensions.add_field("code", Value::scalar(code.as_u16() as i32));
        if let Some(errors) = errors {
            extensions.add_field("errors", errors.into());
        }

        FieldError::new(message, extensions.into())
    }
}
