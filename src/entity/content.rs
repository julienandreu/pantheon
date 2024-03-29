use lambda_http::{Body, Error, Response};
use serde::{Deserialize, Serialize};
use serde_json::{json, Error as JsonError, Value};

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

    pub fn from(value: Value) -> Result<Self, JsonError> {
        serde_json::from_value(value)
    }

    pub fn to_response(&self, status: u16) -> Result<Response<Body>, Error> {
        let response_body = json!({
            "statusCode": status,
            "body": self,
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
