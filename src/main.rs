use std::sync::Arc;

use hyper::Server;
use rocksdb::DB;
use tracing::{error, info};
use tracing_subscriber;

use constants::PORT;
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
    tracing_subscriber::fmt::init();

    let addr = ([127, 0, 0, 1], PORT).into();

    info!("service start at {}", PORT);

    let db = Arc::new(DB::open_default(constants::DB_PATH).unwrap());
    let server = Server::bind(&addr).serve(MakeS3Svc { db });

    let graceful = server.with_graceful_shutdown(shutdown_signal());

    if let Err(e) = graceful.await {
        error!("server error: {}", e);
    }

    Ok(())
}
