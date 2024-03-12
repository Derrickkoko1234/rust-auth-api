// cloudflare file management
use std::io::Error;
use rusoto_s3::DeleteObjectRequest;
use rusoto_s3::S3; // v0.44.0
use rusoto_s3::PutObjectRequest;

use crate::app::mongo::AppInstance;
use crate::library::constant;

pub async fn upload_files(app: &AppInstance,file_bytes: Vec<u8>,storage_path: String,mimetype: String)->Result<bool,Error>{
    // Upload the file to the existing S3 bucket
    let put_object_request = PutObjectRequest {
        bucket: constant::CLOUDFLARE_BUCKET.to_string(),
        key: storage_path,
        body: Some(file_bytes.into()),
        content_type: Some(mimetype),
        ..Default::default()
    };

    let cloned_client = app.cloudflare_s3.s3_client.clone();
    let result = cloned_client.put_object(put_object_request).await;

    if result.is_err(){
        return Ok(!result.is_err());
    }

    Ok(true)
}

pub async fn remove_file(app: &AppInstance,storage_path: String)->Result<bool,Error>{
    // Delete the file from S3 bucket
    let delete_object_request = DeleteObjectRequest {
        bucket: constant::CLOUDFLARE_BUCKET.to_string(),
        key: storage_path.to_string(),
        ..Default::default()
    };

    let cloned_client = app.cloudflare_s3.s3_client.clone();
    let result = cloned_client.delete_object(delete_object_request).await;

    if result.is_err(){
        return Ok(!result.is_err());
    }

    Ok(true)
}