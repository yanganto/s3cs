use std::fs::{remove_file, File};
use std::io::prelude::*;

use executable_path::executable_path;
use subprocess::{Exec, Popen};
#[cfg(feature = "ci-test")]
use tokio::time::{delay_for, Duration};

pub struct MockServer {
    inner: Popen,
    // TODO: use tempfile crate
    pub tmp_file: String,
    pub tmp_file_content: String,
    pub access_key: &'static str,
    pub secret_key: &'static str,
}

impl MockServer {
    pub async fn new() -> Self {
        let tmp_file_name = "/tmp/s3cs-test";
        let access_key = "AAAAAAAAAAAAAAAAAAAA";
        let secret_key = "SSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSS";
        let popen = Exec::cmd(executable_path("s3cs")).popen().unwrap();
        let tmp_file_content = "This is a test file\n";

        // Eventurally, the file base key is for admin
        let mut file = File::create(format!("/tmp/{}", access_key)).unwrap();
        file.write_all(secret_key.as_bytes()).unwrap();

        let mut tmp_file = File::create(tmp_file_name).unwrap();
        let content: Vec<u8> = tmp_file_content.bytes().collect();
        tmp_file.write_all(&content).unwrap();

        #[cfg(feature = "ci-test")]
        delay_for(Duration::from_secs(30)).await;

        Self {
            inner: popen,
            tmp_file: tmp_file_name.into(),
            tmp_file_content: tmp_file_content.into(),
            access_key,
            secret_key,
        }
    }
}

impl Drop for MockServer {
    fn drop(&mut self) {
        self.inner.kill().unwrap();
        self.inner.wait().unwrap();
        remove_file(&self.tmp_file).unwrap();
    }
}
