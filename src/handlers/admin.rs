use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

use hyper::service::Service;
use hyper::{Body, Request, Response};
use rocksdb::DB;
// use tracing::{debug, error, info, warn};

pub struct AdminSvc {
    _db: Arc<DB>,
}

impl Service<Request<Body>> for AdminSvc {
    type Response = Response<Body>;
    type Error = hyper::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _: &mut Context) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, r: Request<Body>) -> Self::Future {
        let path = format!("{}", r.uri().path());
        // TODO CRUD /key
        // TODO clear cache
        // DELETE /cache/2020-12-05              // delete cache older than
        // DELETE /cache/2020-12-03~2020-12-05/  // delete cache between 2020-12-03~2020-12-05
        Box::pin(async move { Ok(Response::new(Body::from(path))) })
    }
}

pub struct MakeAdminSvc {
    pub db: Arc<DB>,
}

impl<T> Service<T> for MakeAdminSvc {
    type Response = AdminSvc;
    type Error = hyper::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _: &mut Context) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, _: T) -> Self::Future {
        let db = self.db.clone();
        let fut = async move { Ok(AdminSvc { _db: db }) };
        Box::pin(fut)
    }
}
