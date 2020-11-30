use std::convert::Infallible;

use http::header::AUTHORIZATION;
use hyper::{Body, Method, Request, Response};
use tokio::fs::File;
use tokio::prelude::*;

use crate::constants::KEY_FOLDER;

pub async fn s3(r: Request<Body>) -> Result<Response<Body>, Infallible> {
    let is_valid = {
        let mut is_valid = false;
        let headers = r.headers();
        if headers.contains_key(AUTHORIZATION) {
            match headers[AUTHORIZATION].to_str() {
                // TODO support V4
                Ok(auth_header) if auth_header.starts_with("AWS ") => {
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
                    println!("{:?} : {:?}", access_key, signature);
                    if let Ok(mut key_file) =
                        File::open(&format!("{}/{}", KEY_FOLDER, access_key)).await
                    {
                        let mut contents = vec![];
                        key_file
                            .read_to_end(&mut contents)
                            .await
                            .expect("key file should have integraty");
                        println!("{:?}", contents);
                        is_valid = true;
                    }
                }
                _ => (),
            }
        }
        is_valid
    };
    if !is_valid {
        return Ok(Response::new(Body::from("Unauthorize")));
    }
    match *r.method() {
        Method::GET => Ok(Response::new(Body::from("object"))),
        _ => Ok(Response::new(Body::from("Unimplement"))),
    }
}
