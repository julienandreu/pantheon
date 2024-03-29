use lambda_http::{run, service_fn, tracing, Body, Error, Request, RequestExt, Response};
use pantheon::entity::Content;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    run(service_fn(get_all_contents)).await
}

async fn get_all_contents(event: Request) -> Result<Response<Body>, Error> {
    let verb = event.method().as_str();
    let payload = event.body();

    let who = event
        .query_string_parameters()
        .first("name")
        .map_or("world".to_string(), |name| name.to_string());

    let description = std::str::from_utf8(payload).expect("");

    let content = Content::new(who.to_string(), description.to_string(), verb.to_string());

    let body = json!(content);

    Ok(Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(body.to_string().into())
        .map_err(Box::new)?)
}
