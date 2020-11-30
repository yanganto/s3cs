use std::fs::File;
use std::io::prelude::*;

use executable_path::executable_path;
use subprocess::{Exec, Popen};

pub struct Server {
    inner: Popen,
}

impl Server {
    pub fn new() -> Self {
        // Eventurally, the file base key is for admin
        let mut file = File::create("/tmp/AAAAAAAAAAAAAAAAAAAA").unwrap();
        file.write_all(b"SSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSS")
            .unwrap();
        Self {
            inner: Exec::cmd(executable_path("s3cs")).popen().unwrap(),
        }
    }
}

impl Drop for Server {
    fn drop(&mut self) {
        self.inner.kill().unwrap();
        self.inner.wait().unwrap();
    }
}
