use file_diff::diff_files;
use s3handler::none_blocking::primitives::S3Pool;
use std::fs::File;

use common::MockServer;

mod common;

#[tokio::test]
async fn test_basic_operations() {
    let s = MockServer::new().await;

    let obj = S3Pool::new("127.0.0.1:3000".to_string())
        .aws_v2(s.access_key.into(), s.secret_key.into())
        .bucket("bucket")
        .object("object");
    obj.upload_file(&s.tmp_file).await.unwrap();

    // TODO: use tempfile crate
    let new_temp_file = "/tmp/s3cs-test-downloaded";
    let new_obj = S3Pool::new("127.0.0.1:3000".to_string())
        .aws_v2(s.access_key.into(), s.secret_key.into())
        .bucket("bucket")
        .object("object");

    new_obj.download_file(new_temp_file).await.unwrap();
    let mut file1 = File::open(&s.tmp_file).expect("tmp file should setup by mock server");
    let mut file2 = File::open(new_temp_file).expect("new file should be downloaded");

    assert!(diff_files(&mut file1, &mut file2));
}
