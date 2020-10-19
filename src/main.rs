#![feature(str_split_once)]
use actix_web::{middleware, web, App, HttpServer};
use handlers::{get_hander, post_handler, put_handler};
use tracing_subscriber;

mod handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();

    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    std::fs::create_dir_all("/tmp/s3cs").unwrap();

    let ip = "0.0.0.0:12537";

    HttpServer::new(|| {
        App::new().wrap(middleware::Logger::default()).service(
            web::resource("/{key:.*}")
                .route(web::get().to(get_hander))
                .route(web::post().to(post_handler))
                .route(web::put().to(put_handler)),
        )
    })
    .bind(ip)?
    .run()
    .await
}
