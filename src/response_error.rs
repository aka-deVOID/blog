use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use sea_orm::error::{DbErr as SeaORMError, RuntimeErr};
use serde_json::{json, Value as JsonVal};
use sqlx_core::error::Error as SqlxErr;
use thiserror::Error;

pub type ResponseResult = std::result::Result<HttpResponse, ResponseErr>;

#[derive(Error, Debug)]
pub enum ResponseErr {
    #[error("Unauthorized: {}", _0)]
    Unauthorized(JsonVal),

    #[error("Forbidden: {}", _0)]
    Forbidden(JsonVal),

    #[error("Not Found: {}", _0)]
    NotFound(JsonVal),

    #[error("Unprocessable Entity: {}", _0)]
    UnprocessableEntity(JsonVal),

    #[error("Internet Server Error")]
    InternetServerError,
}

impl ResponseError for ResponseErr {
    fn status_code(&self) -> StatusCode {
        match *self {
            ResponseErr::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            ResponseErr::Forbidden(_) => StatusCode::FORBIDDEN,
            ResponseErr::NotFound(_) => StatusCode::NOT_FOUND,
            ResponseErr::UnprocessableEntity(_) => StatusCode::UNPROCESSABLE_ENTITY,
            ResponseErr::InternetServerError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        match self {
            ResponseErr::Unauthorized(val) => HttpResponse::Unauthorized()
                .status(self.status_code())
                .json(val),
            ResponseErr::Forbidden(val) => HttpResponse::Forbidden()
                .status(self.status_code())
                .json(val),
            ResponseErr::NotFound(val) => HttpResponse::NotFound()
                .status(self.status_code())
                .json(val),
            ResponseErr::UnprocessableEntity(val) => HttpResponse::UnprocessableEntity()
                .status(self.status_code())
                .json(val),
            ResponseErr::InternetServerError => HttpResponse::InternalServerError()
                .status(self.status_code())
                .json("internal server error"),
        }
    }
}

impl From<SeaORMError> for ResponseErr {
    fn from(value: SeaORMError) -> Self {
        match value {
            //            SeaORMError::ConnectionAcquire => todo!(),
            //            SeaORMError::TryIntoErr { from, into, source } => todo!(),
            //            SeaORMError::Conn(runtime_err) => match runtime_err {
            //                RuntimeErr::SqlxError(err) => todo!(),
            //                RuntimeErr::Internal(_) => Self::InternetServerError,
            //            },
            //            SeaORMError::Exec(runtime_err) => match runtime_err {
            //                RuntimeErr::SqlxError(err) => match err {
            //
            //                },
            //                RuntimeErr::Internal(_) => Self::InternetServerError,
            //            },
            //            SeaORMError::Query(runtime_err) => match runtime_err {
            //                RuntimeErr::SqlxError(err) => match err {
            //                    _ => todo!(),
            //                },
            //                RuntimeErr::Internal(_) => Self::InternetServerError,
            //            },
            //            SeaORMError::UnpackInsertId => todo!(),
            SeaORMError::RecordNotFound(str) => Self::NotFound(json!({ "error": str })),
            //            SeaORMError::AttrNotSet(_) => todo!(),
            //            SeaORMError::Custom(_) => todo!(),
            //            SeaORMError::Type(_) => todo!(),
            //            SeaORMError::Json(_) => todo!(),
            //            SeaORMError::Migration(_) => todo!(),
            SeaORMError::RecordNotInserted => {
                Self::UnprocessableEntity(json!({"error": "already exist."}))
            }
            //            SeaORMError::RecordNotUpdated => todo!(),
            _ => Self::InternetServerError,
        }
    }
}
//
//impl From<sqlx_core::error::Error> for ResponseErr {
//    fn from(value: sqlx_core::error::Error) -> Self {
//        match value {
//            SqlxErr::Configuration(_) => todo!(),
//            SqlxErr::Database(_) => todo!(),
//            SqlxErr::Io(_) => todo!(),
//            SqlxErr::Tls(_) => todo!(),
//            SqlxErr::Protocol(_) => todo!(),
//            SqlxErr::RowNotFound => todo!(),
//            SqlxErr::TypeNotFound { type_name } => todo!(),
//            SqlxErr::ColumnIndexOutOfBounds { index, len } => todo!(),
//            SqlxErr::ColumnNotFound(_) => todo!(),
//            SqlxErr::ColumnDecode { index, source } => todo!(),
//            SqlxErr::Decode(_) => todo!(),
//            SqlxErr::AnyDriverError(_) => todo!(),
//            SqlxErr::PoolTimedOut => todo!(),
//            SqlxErr::PoolClosed => todo!(),
//            SqlxErr::WorkerCrashed => todo!(),
//            _ => todo!(),
//        }
//    }
//}
