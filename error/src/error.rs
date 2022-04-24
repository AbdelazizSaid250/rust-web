use std::fmt::{Display, Formatter};

use actix_web::{HttpResponse, ResponseError};
use diesel::result::Error as DieselError;
use juniper::{ScalarValue, FieldError, IntoFieldError, graphql_value};
use serde::{Deserialize, Serialize};
use validator::{ValidationErrors, ValidationErrorsKind};

#[derive(Debug)]
pub enum ServerErrorResponse {
    InternalServerError(Vec<ErrorCode>),
    BadReq(Vec<ErrorCode>),
    NotFound(ErrorCode),
}

impl ResponseError for ServerErrorResponse {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServerErrorResponse::InternalServerError(errors) => HttpResponse::InternalServerError().json(errors),
            ServerErrorResponse::BadReq(errors) => HttpResponse::BadRequest().json(errors),
            ServerErrorResponse::NotFound(errors) => HttpResponse::NotFound().json(errors),
        }
    }
}

#[derive(Debug)]
pub enum Error {
    DBError(DieselError),
    BadRequest(String),
    InternalServerError(String),
    NotFound(String),
    HttpRequest(String),
    DuplicationError,
    DeletedDuplicationError,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ErrorCode {
    pub code: String,
}

impl ErrorCode {
    pub fn validate_errors(error: ValidationErrors, errors: &mut Vec<ErrorCode>) {
        for value in error.errors().values() {
            match value {
                ValidationErrorsKind::Struct(_) => {}
                ValidationErrorsKind::List(_) => {}
                ValidationErrorsKind::Field(validation_error_vec) => {
                    for validation_error in validation_error_vec {
                        let code = validation_error.clone().code.to_string();
                        let _message = validation_error
                            .clone()
                            .message
                            .expect("Validation Error")
                            .to_string();
                        errors.push(ErrorCode {
                            code,
                        });
                    }
                }
            }
        }
    }
}

pub struct ErrorCodesWrapper {
    error_codes: Vec<ErrorCode>,
}

impl ErrorCodesWrapper {
    pub fn get_error_codes(&self) -> Vec<ErrorCode> {
        self.error_codes.clone()
    }
}


impl From<DieselError> for Error {
    fn from(err: DieselError) -> Self {
        Self::DBError(err)
    }
}

impl From<Error> for ErrorCodesWrapper {
    fn from(err: Error) -> Self {
        match err {
            Error::DBError(diesel_error) => {
                match diesel_error {
                    DieselError::InvalidCString(nul_error) => Self::from(format!("invalid-CString {}.", nul_error).as_str()),
                    DieselError::DatabaseError(_, _) => Self::from("database-error"),
                    DieselError::NotFound => Self::from("object-not-found"),
                    DieselError::QueryBuilderError(err) => Self::from(format!("query-builder-error {}.", err).as_str()),
                    DieselError::DeserializationError(err) => Self::from(format!("deserialization-error {}.", err).as_str()),
                    DieselError::SerializationError(err) => Self::from(format!("serialization-error {}.", err).as_str()),
                    DieselError::RollbackTransaction => Self::from("rollback-transaction."),
                    DieselError::AlreadyInTransaction => Self::from("already-in-transaction."),
                    DieselError::__Nonexhaustive => Self::from("non-exhaustive."),
                }
            }
            Error::BadRequest(error) => Self::from(error.as_str()),
            Error::InternalServerError(error) => Self::from(error.as_str()),
            Error::NotFound(error) => Self::from(error.as_str()),
            Error::HttpRequest(error) => Self::from(error.as_str()),
            Error::DuplicationError => Self::from("duplication-error"),
            Error::DeletedDuplicationError => Self::from("deleted-duplication-error"),
        }
    }
}

impl From<&str> for ErrorCodesWrapper {
    fn from(str: &str) -> Self {
        Self { error_codes: vec![ErrorCode { code: str.to_string() }] }
    }
}

impl From<Vec<ErrorCode>> for ServerErrorResponse {
    fn from(error_codes: Vec<ErrorCode>) -> Self {
        Self::InternalServerError(error_codes)
    }
}


impl Display for ServerErrorResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ServerErrorResponse::InternalServerError(_) => write!(f, "Internal Server Error Display."),
            ServerErrorResponse::BadReq(_) => write!(f, "Bas Request Display."),
            ServerErrorResponse::NotFound(_) => write!(f, "Not Found Display."),
        }
    }
}

impl<S: ScalarValue> IntoFieldError<S> for Error {
    fn into_field_error(self) -> FieldError<S> {
        match self {
            Error::DBError(diesel_error) => {
                match diesel_error {
                    DieselError::InvalidCString(_) => FieldError::new("gql_diesel_error", graphql_value!({ "type": "InvalidCString"})),
                    DieselError::DatabaseError(_, _) => FieldError::new("gql_diesel_error", graphql_value!({ "type": "DatabaseError"})),
                    DieselError::NotFound => FieldError::new("gql_diesel_error", graphql_value!({ "type": "NotFound"})),
                    DieselError::QueryBuilderError(_) => FieldError::new("gql_diesel_error", graphql_value!({ "type": "QueryBuilderError"})),
                    DieselError::DeserializationError(_) => FieldError::new("gql_diesel_error", graphql_value!({ "type": "DeserializationError"})),
                    DieselError::SerializationError(_) => FieldError::new("gql_diesel_error", graphql_value!({ "type": "SerializationError"})),
                    DieselError::RollbackTransaction => FieldError::new("gql_diesel_error", graphql_value!({ "type": "RollbackTransaction"})),
                    DieselError::AlreadyInTransaction => FieldError::new("gql_diesel_error", graphql_value!({ "type": "AlreadyInTransaction"})),
                    DieselError::__Nonexhaustive => FieldError::new("gql_diesel_error", graphql_value!({ "type": "__Nonexhaustive"})),
                }
            }
            Error::BadRequest(_) => FieldError::new("gql_bad_request", graphql_value!({ "type": "BadRequest" })),
            Error::InternalServerError(_) => FieldError::new("gql_bad_request", graphql_value!({ "type": "InternalServerError" })),
            Error::NotFound(_) => FieldError::new("gql_bad_request", graphql_value!({ "type": "NotFound" })),
            Error::HttpRequest(_) => FieldError::new("gql_bad_request", graphql_value!({ "type": "HttpRequest" })),
            Error::DuplicationError => FieldError::new("gql_bad_request", graphql_value!({ "type": "DuplicationError" })),
            Error::DeletedDuplicationError => FieldError::new("gql_bad_request", graphql_value!({ "type": "DeletedDuplicationError" })),
        }
    }
}