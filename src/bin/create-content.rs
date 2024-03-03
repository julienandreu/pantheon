use lambda_http::{
    http::Method, run, service_fn, tracing, Body, Error, Request, RequestExt, Response,
};
use pantheon::entity::{Content, HttpError, HttpErrorType};
use serde_json::json;

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

    let payload = event.body();

    let who = event
        .query_string_parameters_ref()
        .and_then(|params| params.first("name"))
        .unwrap_or("world");
    let content = Content::new(
        String::from(who),
        json!(payload).to_string(),
        String::from("text/plain"),
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
