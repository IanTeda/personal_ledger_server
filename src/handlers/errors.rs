use actix_web::{http::StatusCode, ResponseError};

/// Handlers module errors
#[derive(thiserror::Error, Debug, PartialEq)]
pub enum Error {
	#[error("There is no Thing associated with the provided id.")]
    ThingUnknownId,

	#[error("There is no Thing associated with the provided name.")]
    ThingUnknownName,

	#[error("Parameter missing from query.")]
	ParameterMissing,
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        let status_code = match self {
			Self::ThingUnknownName => StatusCode::BAD_REQUEST,
            Self::ThingUnknownId => StatusCode::BAD_REQUEST,
            Self::ParameterMissing => StatusCode::BAD_REQUEST,
        };
        status_code
    }
}