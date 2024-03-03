use lambda_http::{run, service_fn, tracing, Body, Error, Request, RequestExt, Response};
use pantheon::entities::Content;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    run(service_fn(get_all_contents)).await
}

async fn get_all_contents(event: Request) -> Result<Response<Body>, Error> {
    // Extract some useful information from the request
    let who = event
        .query_string_parameters_ref()
        .and_then(|params| params.first("name"))
        .unwrap_or("world");
    let content = Content::new(
        String::from(who),
        String::from("description"),
        String::from("image"),
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
