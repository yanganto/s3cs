use serde_derive::{Deserialize, Serialize};
use structopt::clap::Shell;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub(crate) struct Opt {
    #[structopt(short, long)]
    /// Activate debug mode
    pub debug: bool,

    /// Config file path (optional)
    /// default is  "~/.config/s3cs.toml"
    pub config: Option<String>,

    /// Generate completions
    /// ex: s3cs -c bash /usr/share/bash-completion/completions
    #[structopt(short, long)]
    pub completions: Option<Shell>,

    /// Generate default config
    /// ex: s3cs -g ~/.config/s3cs.toml
    #[structopt(short, long)]
    pub generate_default: bool,
}

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub key_folder: String,
    pub db_path: String,
    pub s3_port: u16,
    pub admin_port: u16,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            key_folder: "/tmp".into(),
            db_path: "/tmp/s3cs_db".into(),
            s3_port: 3000,
            admin_port: 3333,
        }
    }
}
