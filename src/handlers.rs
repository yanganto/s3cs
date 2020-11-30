use std::convert::Infallible;

use http::header::AUTHORIZATION;
use hyper::{body::HttpBody, Body, Method, Request, Response};
use s3handler::blocking::aws::{aws_s3_v2_get_string_to_signed, aws_s3_v2_sign};
use tokio::fs;

use crate::constants::KEY_FOLDER;

async fn validate_aws_v2(mut r: Request<Body>, auth_header: &str) -> Option<Vec<u8>> {
    let mut valid_paload: Option<Vec<u8>> = None;
    let (access_key, signature) = {
        let auth_body = auth_header
            .split(" ")
            .nth(1)
            .unwrap_or_default()
            .to_string();
        let mut auth_body = auth_body.split(":");
        (
            auth_body.next().unwrap_or_default().to_string(),
            auth_body.next().unwrap_or_default().to_string(),
        )
    };
    if let Ok(secret_key) = fs::read_to_string(&format!("{}/{}", KEY_FOLDER, access_key)).await {
        valid_paload = match r.body_mut().data().await {
            Some(Ok(payload)) => Some(payload.to_vec()),
            _ => Some(Vec::new()),
        };

        let uri = r.uri().clone();
        let mut signed_headers: Vec<(&str, &str)> =
            r.headers().iter().fold(Vec::new(), |mut v, h| {
                if h.0.as_str() == "date" || h.0.as_str().starts_with("x-amz") {
                    v.push((h.0.as_str(), h.1.to_str().unwrap_or_default()));
                }
                v
            });

        // TODO: prevent replay attack heere

        let sig = aws_s3_v2_sign(
            &secret_key,
            &aws_s3_v2_get_string_to_signed(
                &r.method().to_string(),
                uri.into_parts()
                    .path_and_query
                    .expect("request should have path")
                    .path(),
                &mut signed_headers,
                &Vec::new(),
            ),
        );
        if sig != signature {
            valid_paload = None;
        }
    }
    valid_paload
}

pub async fn s3(r: Request<Body>) -> Result<Response<Body>, Infallible> {
    let method = r.method().clone();
    let valid_paload = {
        let headers = r.headers().clone();
        if headers.contains_key(AUTHORIZATION) {
            match headers[AUTHORIZATION].to_str() {
                // TODO support V4 and more than AWS client
                Ok(auth_header) if auth_header.starts_with("AWS ") => {
                    validate_aws_v2(r, auth_header).await
                }
                _ => None,
            }
        } else {
            None
        }
    };
    if valid_paload.is_none() {
        return Ok(Response::new(Body::from("Unauthorize")));
    }
    match method {
        Method::GET => Ok(Response::new(Body::from("object"))),
        _ => Ok(Response::new(Body::from("Unimplement"))),
    }
}
