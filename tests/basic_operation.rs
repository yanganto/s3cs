use s3handler::none_blocking::primitives::S3Pool;

use common::Server;

mod common;

#[tokio::test]
async fn test_basic_operations() {
    let s = Server::new().await;
    let s3_pool =
        S3Pool::new("127.0.0.1:3000".to_string()).aws_v2(s.access_key.into(), s.secret_key.into());

    let obj = s3_pool.bucket("bucket").object("object");
    obj.upload_file(&s.tmp_file).await.unwrap();
    // obj.download_file(&s.tmp_file).await.unwrap();
}
