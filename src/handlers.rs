use actix_web::{
    web::{Bytes, Path},
    Error, HttpResponse,
};
use tracing::debug;

pub async fn post_handler(
    Path((obj,)): Path<(String,)>,
    payload: Bytes,
) -> Result<HttpResponse, Error> {
    debug!("post:{:#?}", payload);

    // TODO: support json output
    if let Some((bucket, key)) = obj.as_str().split_once('/') {
        Ok(HttpResponse::Ok().body(format!("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<InitiateMultipartUploadResult xmlns=\"http://s3.amazonaws.com/doc/2006-03-01/\"><Bucket>{}</Bucket><Key>{}</Key><UploadId>7xx8ZHUavJ6n3nUd1ugYUPZjrsIiamytRkvGST6DsyqNbXbAig30yTXcGUAZRxGnNEMWKqe7QiK6VUpHkSf9xjGouk2EcvkbWEzg1iSm7sT427J8yDVW2NEWsdV5D2TsA5_.RU.MauLAk0Tlyo9VVg--</UploadId></InitiateMultipartUploadResult>", bucket, key)))
    } else {
        Ok(HttpResponse::MethodNotAllowed()
            .body("Post on root is not allow")
            .into())
    }
}

pub async fn put_handler(
    Path((key,)): Path<(String,)>,
    payload: Bytes,
) -> Result<HttpResponse, Error> {
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
