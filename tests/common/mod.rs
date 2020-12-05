use std::fs::{remove_file, File};
use std::io::prelude::*;
use std::process::Command;

use executable_path::executable_path;
use subprocess::{Exec, Popen, Redirection};
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
        let log_file = File::create("/tmp/log").expect("log open error");
        let exe = executable_path("s3cs");
        let status = Command::new(&exe)
            .arg("-g")
            .status()
            .expect("failed to execute process");
        assert!(status.success());
        let popen = Exec::cmd(exe)
            .env("RUST_LOG", "trace")
            .stdout(Redirection::File(log_file))
            .popen()
            .unwrap();
        let tmp_file_content = "This is a test file\n";

        // Eventurally, the file base key is for admin
        let mut file = File::create(format!("/tmp/{}", access_key)).unwrap();
        file.write_all(secret_key.as_bytes()).unwrap();

        let mut tmp_file = File::create(tmp_file_name).unwrap();
        let content: Vec<u8> = tmp_file_content.bytes().collect();
        tmp_file.write_all(&content).unwrap();

        // This is a simple workaround to wait the server ready
        // if you got some service unavailbe problem on running test,
        // extending the delay time may work
        #[cfg(feature = "ci-test")]
        delay_for(Duration::from_secs(30)).await;
        #[cfg(not(feature = "ci-test"))]
        delay_for(Duration::from_secs(1)).await;

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
