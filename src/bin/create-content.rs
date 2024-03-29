use lambda_http::{http::Method, run, service_fn, tracing, Body, Error, Request, Response};
use pantheon::entity::{Content, HttpError, HttpErrorType};

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    run(service_fn(create_content)).await
}

async fn create_content(event: Request) -> Result<Response<Body>, Error> {
    let method = event.method().clone();

    match method {
        Method::POST => {
            let body = String::from_utf8(event.body().to_vec())?;
            let payload = serde_json::from_str(&body)?;

            match Content::from(payload) {
                Ok(content) => content.to_response(201),
                Err(e) => {
                    return HttpError::new(HttpErrorType::BadRequest, e.to_string()).to_response();
                }
            }
        }
        _ => {
            return HttpError::new(
                HttpErrorType::NotAllowed,
                String::from(format!("HTTP {} Method not allowed", method,)),
            )
            .to_response();
        }
    }
}
