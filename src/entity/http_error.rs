use lambda_http::{Body, Error, Response};
use serde_json::json;
use std::fmt;

pub enum HttpErrorType {
    NotFound,
    BadRequest,
    NotAllowed,
    InternalServerError,
}

impl fmt::Display for HttpErrorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HttpErrorType::NotFound => write!(f, "NotFound"),
            HttpErrorType::BadRequest => write!(f, "BadRequest"),
            HttpErrorType::NotAllowed => write!(f, "NotAllowed"),
            HttpErrorType::InternalServerError => write!(f, "InternalServerError"),
        }
    }
}

pub struct HttpError {
    code: HttpErrorType,
    message: String,
}

impl HttpError {
    pub fn new(code: HttpErrorType, message: String) -> Self {
        Self { code, message }
    }

    pub fn to_response(&self) -> Result<Response<Body>, Error> {
        let status = match self.code {
            HttpErrorType::BadRequest => 400,
            HttpErrorType::NotFound => 404,
            HttpErrorType::NotAllowed => 405,
            HttpErrorType::InternalServerError => 500,
        };

        let response_body = json!({
          "statusCode": status,
          "body": {
            "error": {
              "code": self.code.to_string(),
              "message": self.message,
            },
          },
        })
        .to_string()
        .into();

        Ok(Response::builder()
            .status(status)
            .header("content-type", "application/json")
            .body(response_body)
            .map_err(Box::new)?)
    }
}
