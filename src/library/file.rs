// handles offline and online files

use std::fs;
use std::io;

use std::error::Error;
use std::fs::File;

use super::constant;

pub fn delete_file(file_names: Vec<String>)->Result<bool,io::Error>{
    if file_names.len() > 0{
        for file_name in file_names{
            let _ = fs::remove_file(file_name);
        }
    }
    Ok(true)
}

pub fn read_file_content_as_string(file_path: String)->Result<String,io::Error>{
    let contents = fs::read_to_string(file_path)?;
    Ok(contents)
}

pub fn download_file(url: &str,file_name: &str)->Result<bool, Box<dyn Error>>{
    let response = isahc::get(url)?;

    // Abort download if file size exceeds 5MB (5 * 1024 * 1024 bytes)
    let content_length = response
        .headers()
        .get("content-length")
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.parse::<u64>().ok())
        .unwrap_or(0);

    if content_length > constant::FIVE_MB {
        return Err("File size exceeds 5MB. Aborting download".into());
    }

    let mut dest_file = File::create(file_name)?;

    io::copy(&mut response.into_body(), &mut dest_file)?;

    Ok(true)
}

pub fn file_exists(file_path: &str) -> bool {
    if let Ok(metadata) = fs::metadata(file_path) {
        // If the file exists, metadata.is_file() will be true
        metadata.is_file()
    } else {
        // If there was an error (e.g., the file doesn't exist), return false
        false
    }
}