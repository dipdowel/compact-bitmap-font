use crate::error::CompileFontError;
use crate::types::Dimensions2d;
use image::{DynamicImage, ImageBuffer, RgbaImage, ImageFormat};
use std::io::Cursor;

pub fn pixels_to_png(
    image_buf: &Vec<u32>,
    image_dims: &Dimensions2d,
) -> Result<Vec<u8>, CompileFontError> {
    //
    // Convert the RGBA data from pixels (`u32`) to just bytes (`u8`).
    let rgba_buffer: Vec<u8> = image_buf
        .iter()
        .flat_map(|&pixel| {
            vec![
                ((pixel >> 16) & 0xFF) as u8, // R
                ((pixel >> 8) & 0xFF) as u8,  // G
                (pixel & 0xFF) as u8,         // B
                ((pixel >> 24) & 0xFF) as u8, // A
            ]
        })
        .collect();

    let Dimensions2d { w, h } = *image_dims;

    // Create an ImageBuffer from the RGBA buffer
    let img_buffer: RgbaImage =
        ImageBuffer::from_raw(w, h, rgba_buffer).ok_or(CompileFontError::PNG(
            format!("Could not make a buffer for PNG, width:{}, height:{}", w, h).to_string(),
        ))?;


    let image: DynamicImage = DynamicImage::from(img_buffer);


    // Create a buffer in memory to hold the encoded PNG.
    let mut png_bytes: Vec<u8> = Vec::new();
    let mut cursor = Cursor::new(&mut png_bytes);

    // Encode the image as PNG into the buffer.
    image
        .write_to(&mut cursor, ImageFormat::Png)
        .map_err(|e| CompileFontError::PNG(e.to_string()))?;

    // Return the PNG data as Vec<u8>
    Ok(png_bytes)
}