use std::sync::Arc;

// use hyper::service::{make_service_fn, service_fn};
use hyper::Server;
use rocksdb::DB;

use handlers::MakeS3Svc;

mod constants;
mod handlers;

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = ([127, 0, 0, 1], 3000).into();

    let db = Arc::new(DB::open_default(constants::DB_PATH).unwrap());
    let server = Server::bind(&addr).serve(MakeS3Svc { db });

    let graceful = server.with_graceful_shutdown(shutdown_signal());

    if let Err(e) = graceful.await {
        eprintln!("server error: {}", e);
    }

    Ok(())
}
