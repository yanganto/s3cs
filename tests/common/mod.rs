use std::fs::{remove_file, File};
use std::io::prelude::*;

use executable_path::executable_path;
use subprocess::{Exec, Popen};
#[cfg(feature = "ci-test")]
use tokio::time::{delay_for, Duration};

pub struct Server {
    inner: Popen,
    // TODO: use tempfile crate
    pub tmp_file: String,
    pub access_key: &'static str,
    pub secret_key: &'static str,
}

impl Server {
    pub async fn new() -> Self {
        let tmp_file_name = "/tmp/s3cs-test";
        let access_key = "AAAAAAAAAAAAAAAAAAAA";
        let secret_key = "SSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSS";
        let popen = Exec::cmd(executable_path("s3cs")).popen().unwrap();

        // Eventurally, the file base key is for admin
        let mut file = File::create(format!("/tmp/{}", access_key)).unwrap();
        file.write_all(secret_key.as_bytes()).unwrap();

        let mut tmp_file = File::create(tmp_file_name).unwrap();
        tmp_file.write_all(b"This is a test file\n").unwrap();

        #[cfg(feature = "ci-test")]
        delay_for(Duration::from_secs(30)).await;

        Self {
            inner: popen,
            tmp_file: tmp_file_name.into(),
            access_key,
            secret_key,
        }
    }
}

impl Drop for Server {
    fn drop(&mut self) {
        self.inner.kill().unwrap();
        self.inner.wait().unwrap();
        remove_file(&self.tmp_file).unwrap();
    }
}
