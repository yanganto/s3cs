use executable_path::executable_path;
use subprocess::{Exec, Popen};

pub struct Server {
    inner: Popen,
}

impl Server {
    pub fn new() -> Self {
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
