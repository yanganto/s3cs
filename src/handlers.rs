use std::convert::Infallible;

use hyper::{Body, Request, Response};

pub async fn s3(_: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new(Body::from("Hello World!")))
}
