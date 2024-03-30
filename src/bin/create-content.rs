use lambda_http::{http::Method, run, service_fn, tracing, Body, Error, Request, Response};
use pantheon::entity::{Content, HttpError, HttpErrorType};

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    run(service_fn(create_content)).await
}

async fn create_content(event: Request) -> Result<Response<Body>, Error> {
    let method = event.method().to_owned();
    let body = String::from_utf8(event.body().to_vec())?;

    match method {
        Method::POST => match Content::try_from(body) {
            Ok(content) => content.try_into(),
            Err(e) => HttpError::new(HttpErrorType::BadRequest, e.to_string()).to_response(),
        },
        _ => HttpError::new(
            HttpErrorType::NotAllowed,
            format!("HTTP {} Method not allowed", method),
        )
        .to_response(),
    }
}
