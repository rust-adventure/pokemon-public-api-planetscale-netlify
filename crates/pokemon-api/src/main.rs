use lambda_http::{
    http::header::CONTENT_TYPE, run, service_fn, Body,
    Error, Request, Response,
};

async fn function_handler(
    event: Request,
) -> Result<Response<Body>, Error> {
    dbg!(event);
    let html = "<html><body><h1>hello!</h1></body></html>";
    let resp = Response::builder()
        .status(200)
        .header(CONTENT_TYPE, "text/html")
        .body(Body::Text(html.to_string()))?;
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(service_fn(function_handler)).await
}
