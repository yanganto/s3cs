use std::fs::File;
use std::io::prelude::*;

use s3handler::none_blocking::primitives::S3Pool;

use common::Server;

mod common;

#[tokio::test]
async fn test_basic_operations() {
    let temp_test_file = "/tmp/async-test";
    let _s = Server::new();
    let s3_pool = S3Pool::new("127.0.0.1:3000".to_string()).aws_v2(
        "AAAAAAAAAAAAAAAAAAAA".to_string(),
        "SSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSS".to_string(),
    );
    let obj = s3_pool.bucket("bucket").object("object");
    obj.download_file(temp_test_file).await.unwrap();
    let mut file = File::open(temp_test_file).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{:?}", contents);
}
