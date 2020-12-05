use std::sync::Arc;

use futures_util::future::join;
use hyper::Server;
use rocksdb::DB;
use tracing::{error, info};
use tracing_subscriber;

use admin::MakeAdminSvc;
use constants::{ADMIN_PORT, S3_PORT};
use handlers::MakeS3Svc;

mod admin;
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

    let s3_addr = ([127, 0, 0, 1], S3_PORT).into();
    info!("s3 service start at {}", S3_PORT);
    let admin_addr = ([127, 0, 0, 1], ADMIN_PORT).into();
    info!("admin service start at {}", ADMIN_PORT);

    let db = Arc::new(DB::open_default(constants::DB_PATH).unwrap());
    let db_cloned = db.clone();
    let s3_server = Server::bind(&s3_addr)
        .serve(MakeS3Svc { db })
        .with_graceful_shutdown(shutdown_signal());

    let admin_server = Server::bind(&admin_addr)
        .serve(MakeAdminSvc { db: db_cloned })
        .with_graceful_shutdown(shutdown_signal());

    let (s3_res, admin_res) = join(s3_server, admin_server).await;

    if let Err(e) = s3_res {
        error!("s3 server error: {}", e);
    }
    if let Err(e) = admin_res {
        error!("admin server error: {}", e);
    }

    Ok(())
}
