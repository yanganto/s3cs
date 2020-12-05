use std::fs::File;
use std::io::{Read, Write};
use std::sync::Arc;

use dirs::config_dir;
use futures_util::future::join;
use human_panic::setup_panic;
use hyper::Server;
use rocksdb::DB;
use structopt::StructOpt;
use tracing::{error, info};
use tracing_subscriber;

use cli::{Config, Opt};
use handlers::{admin::MakeAdminSvc, s3::MakeS3Svc};

mod cli;
mod handlers;

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    setup_panic!();
    let Opt {
        completions,
        config,
        generate_default,
        ..
    } = Opt::from_args();
    if let Some(shell) = completions {
        Opt::clap().gen_completions(
            env!("CARGO_PKG_NAME"),
            shell,
            config.unwrap_or_else(|| ".".to_string()),
        );
        return Ok(());
    }

    if generate_default {
        let mut f = if let Some(config) = config {
            File::create(&config)
        } else if let Some(mut config) = config_dir() {
            config.push("s3cs.toml");
            File::create(config)
        } else {
            panic!("no config folder in your OS");
        }
        .expect("config file should be opened");
        f.write_all(toml::to_string(&Config::default()).unwrap().as_bytes())
            .expect("config file should be written");
        return Ok(());
    }

    let config: Config = {
        let mut contents = String::new();
        let f = if let Some(config) = config {
            File::open(&config)
        } else if let Some(mut config) = config_dir() {
            config.push("s3cs.toml");
            File::open(config)
        } else {
            panic!("no config folder in your OS");
        };
        match f {
            Ok(mut f) => {
                f.read_to_string(&mut contents)
                    .expect("config should be read");
                toml::from_str(&contents).expect("config file format should correct")
            }
            Err(_e) => {
                println!("configure file can not be opened correctly, please use `s3sc -g` to generate one");
                return Ok(());
            }
        }
    };

    tracing_subscriber::fmt::init();

    let s3_addr = ([127, 0, 0, 1], config.s3_port).into();
    info!("s3 service start at {}", config.s3_port);
    let admin_addr = ([127, 0, 0, 1], config.admin_port).into();
    info!("admin service start at {}", config.admin_port);

    // TODO: load keys from key folder

    let db = Arc::new(DB::open_default(config.db_path).unwrap());
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
