use lambda_http::{Body, Error as LambdaHttpError, Response};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::convert::{TryFrom, TryInto};
use std::error::Error;
use std::fmt;

#[derive(Clone, Debug)]
pub enum ContentErrorType {
    BadRequest(String),
}

impl fmt::Display for ContentErrorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ContentErrorType::BadRequest(source) => {
                write!(f, "Invalid content: {}", source)
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct ContentError {
    error: ContentErrorType,
    description: String,
}

impl ContentError {
    fn new(error: ContentErrorType) -> Self {
        ContentError {
            description: error.to_string(),
            error,
        }
    }
}

impl fmt::Display for ContentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.error)
    }
}

impl Error for ContentError {
    fn description(&self) -> &str {
        &self.description
    }
}

#[derive(Serialize, Deserialize)]
pub struct Content {
    pub name: String,
    pub description: String,
    pub image: String,
}

impl Content {
    pub fn new(name: String, description: String, image: String) -> Self {
        Self {
            name,
            description,
            image,
        }
    }
}

impl TryFrom<String> for Content {
    type Error = ContentError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match serde_json::from_str(&value) {
            Ok(content) => Ok(content),
            Err(error) => Err(ContentError::new(ContentErrorType::BadRequest(
                error.to_string(),
            ))),
        }
    }
}

impl TryInto<Response<Body>> for Content {
    type Error = LambdaHttpError;

    fn try_into(self) -> Result<Response<Body>, Self::Error> {
        let response_body = json!({
            "statusCode": 200,
            "body": self,
        })
        .to_string()
        .into();

        Ok(Response::builder()
            .status(200)
            .header("content-type", "application/json")
            .body(response_body)
            .map_err(Box::new)?)
    }
}
