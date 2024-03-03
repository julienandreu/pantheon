use lambda_http::{http::Method, run, service_fn, tracing, Body, Error, Request, Response};
use pantheon::entity::{Content, HttpError, HttpErrorType};
use serde_json::{json, Value};

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    run(service_fn(create_content)).await
}

async fn create_content(event: Request) -> Result<Response<Body>, Error> {
    match event.method() {
        &Method::POST => (),
        _ => {
            return HttpError::new(
                HttpErrorType::NotAllowed,
                String::from(format!("HTTP {} Method not allowed", event.method(),)),
            )
            .to_response();
        }
    }

    let data = std::str::from_utf8(event.body()).expect("non utf-8");
    let payload: Value = serde_json::from_str(data)?;

    let content = Content::new(
        payload.get("name").unwrap().as_str().unwrap().to_string(),
        payload
            .get("description")
            .unwrap()
            .as_str()
            .unwrap()
            .to_string(),
        payload.get("image").unwrap().as_str().unwrap().to_string(),
    );
    let body = json!(content);

    // Return something that implements IntoResponse.
    // It will be serialized to the right response event automatically by the runtime
    let resp = Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(body.to_string().into())
        .map_err(Box::new)?;
    Ok(resp)
}
