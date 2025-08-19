#![deny(clippy::all)]
use image::{GenericImageView, ImageReader};
use reqwest::get;
use std::io::Cursor;

use napi_derive::napi;


#[napi]
pub fn image_dimensions(path: String) -> napi::Result<(u32, u32)> {
  let img = image::open(path).map_err(|e| napi::Error::new(napi::Status::GenericFailure, e.to_string()))?;
  Ok(img.dimensions())
}

#[napi]
pub async fn get_remote_image_dimensions_result() -> napi::Result<(u32, u32)> {
    // Replace this with the image URL you want to fetch
    let url = "https://www.rust-lang.org/logos/rust-logo-512x512.png";

    // Fetch the image
    let response = get(url).await.map_err(|e| napi::Error::new(napi::Status::GenericFailure, e.to_string()))?;
    let bytes = response.bytes().await.map_err(|e| napi::Error::new(napi::Status::GenericFailure, e.to_string()))?;

    // Load the image from the bytes
    let cursor = Cursor::new(bytes);
    let img = ImageReader::new(cursor)
        .with_guessed_format()
        .map_err(|e| napi::Error::new(napi::Status::GenericFailure, e.to_string()))?
        .decode()
        .map_err(|e| napi::Error::new(napi::Status::GenericFailure, e.to_string()))?;

    // Get dimensions
    let (width, height) = img.dimensions();

    Ok((width, height))
}