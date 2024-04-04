use std::{io, result};

use axum::{http::StatusCode, response::IntoResponse};

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum ApiError {
    NotFound,
}


#[derive(Debug)]
pub enum ModelError {
    UnexpectedData
}

#[derive(Debug)]
pub enum Error {
    DbError(sqlx::Error),
    ApiError(ApiError),
    Model(ModelError),
    SerdeError(serde_json::Error),
    OtherError(anyhow::Error),
    IOError(io::Error),
    RequestError(reqwest::Error),
}

impl From<ModelError> for Error {
    fn from(e: ModelError) -> Error {
        Error::Model(e)
    }
}

impl From<ApiError> for Error {
    fn from(e: ApiError) -> Error {
        Error::ApiError(e)
    }
}

impl From<sqlx::Error> for Error {
    fn from(e: sqlx::Error) -> Error {
        Error::DbError(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Error {
        Error::SerdeError(e)
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Error {
        Error::IOError(e)
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Error {
        Error::RequestError(e)
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLE_CLIENT_ERROR").into_response()
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::INTERNAL_SERVER_ERROR, "Unexpected").into_response()
    }
}