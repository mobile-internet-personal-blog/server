use axum::{http::StatusCode, response::IntoResponse};

#[derive(Debug)]
pub enum ApiError {
    NOT_FOUND,
}


#[derive(Debug)]
pub enum ModelError {
    UnexpectedData
}

#[derive(Debug)]
pub enum Error {
    DbError(anyhow::Error),
    ApiError(ApiError),
    Model(ModelError),
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

impl<E> From<E> for Error
where 
    E: Into<anyhow::Error>
{
    fn from(e: E) -> Error {
        Error::DbError(e.into())
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