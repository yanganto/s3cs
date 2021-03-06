use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

use http::header::AUTHORIZATION;
use hyper::service::Service;
use hyper::{body::HttpBody, Body, Method, Request, Response, StatusCode};
use rocksdb::DB;
use s3handler::blocking::aws::{aws_s3_v2_get_string_to_signed, aws_s3_v2_sign};
use tokio::fs;
use tracing::{debug, error, info, warn};

async fn validate_aws_v2(mut r: Request<Body>, auth_header: &str) -> Option<Vec<u8>> {
    let mut valid_payload: Option<Vec<u8>> = None;
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
    // TODO: load key from key folder before service running
    if let Ok(secret_key) = fs::read_to_string(&format!("/tmp/{}", access_key)).await {
        valid_payload = match r.body_mut().data().await {
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
            valid_payload = None;
        }
    }
    valid_payload
}

pub struct S3Svc {
    db: Arc<DB>,
}

impl Service<Request<Body>> for S3Svc {
    type Response = Response<Body>;
    type Error = hyper::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _: &mut Context) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, r: Request<Body>) -> Self::Future {
        let method = r.method().clone();
        let uri = r.uri().clone();
        let db = self.db.clone();

        Box::pin(async move {
            let valid_payload = {
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
            if let Some(valid_payload) = valid_payload {
                info!("{} request", method);
                debug!("payload {:?}", valid_payload);
                match method {
                    Method::GET => match db.get(uri.path()) {
                        Ok(Some(v)) => Ok(Response::new(Body::from(v))),
                        Ok(None) => {
                            // TODO: proxy here
                            Ok(Response::new(Body::from("Not Found")))
                        }
                        Err(e) => {
                            warn!("get object error: {:?}", e);
                            Ok(Response::new(Body::from("Not Found")))
                        }
                    },
                    Method::PUT => {
                        let mut r = Response::new(Body::from(Vec::new()));
                        if let Err(e) = db.put(uri.path(), valid_payload) {
                            error!("put object error: {:?}", e);
                            *r.status_mut() = StatusCode::CREATED;
                        } else {
                            *r.status_mut() = StatusCode::CREATED;
                        }
                        Ok(r)
                    }
                    _ => Ok(Response::new(Body::from("Unimplement"))),
                }
            } else {
                Ok(Response::new(Body::from("Unauthorize")))
            }
        })
    }
}

pub struct MakeS3Svc {
    pub db: Arc<DB>,
}

impl<T> Service<T> for MakeS3Svc {
    type Response = S3Svc;
    type Error = hyper::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _: &mut Context) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, _: T) -> Self::Future {
        let db = self.db.clone();
        let fut = async move { Ok(S3Svc { db }) };
        Box::pin(fut)
    }
}
