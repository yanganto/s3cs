use std::fs::{remove_file, File};
use std::io::prelude::*;

use executable_path::executable_path;
use subprocess::{Exec, Popen};

pub struct Server {
    inner: Popen,
    // TODO: use tempfile crate
    pub tmp_file: String,
    pub access_key: &'static str,
    pub secret_key: &'static str,
}

impl Server {
    pub fn new() -> Self {
        let tmp_file_name = "/tmp/s3cs-test";
        let access_key = "AAAAAAAAAAAAAAAAAAAA";
        let secret_key = "SSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSS";

        // Eventurally, the file base key is for admin
        let mut file = File::create(format!("/tmp/{}", access_key)).unwrap();
        file.write_all(secret_key.as_bytes()).unwrap();

        let mut tmp_file = File::create(tmp_file_name).unwrap();
        tmp_file.write_all(b"This is a test file\n").unwrap();

        Self {
            inner: Exec::cmd(executable_path("s3cs")).popen().unwrap(),
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
