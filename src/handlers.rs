use actix_web::{
    web::{Bytes, Path},
    Error, HttpResponse,
};
use tracing::debug;

pub async fn post_handler(mut payload: Bytes) -> Result<HttpResponse, Error> {
    debug!("post:{:#?}", payload);
    Ok(HttpResponse::Ok().into())
}

pub async fn put_handler(
    Path((key,)): Path<(String,)>,
    payload: Bytes,
) -> Result<HttpResponse, Error> {
    // debug!("put: {} bytes", payload.len());
    debug!("PUT key: {} {} bytes", key, payload.len());
    Ok(HttpResponse::Ok().into())
}

pub fn get_hander() -> HttpResponse {
    let html = r#"<html>
        <head><title>A S3 Cache Share Agent</title></head>
        <body>Hi this is a agnet help you share you s3 cache</body>
    </html>"#;
    HttpResponse::Ok().body(html)
}
