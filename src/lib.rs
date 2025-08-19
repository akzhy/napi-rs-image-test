#![deny(clippy::all)]
use image::GenericImageView;

use napi_derive::napi;


#[napi]
pub fn image_dimensions(path: String) -> napi::Result<(u32, u32)> {
  let img = image::open(path).map_err(|e| napi::Error::new(napi::Status::GenericFailure, e.to_string()))?;
  Ok(img.dimensions())
}