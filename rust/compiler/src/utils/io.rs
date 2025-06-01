use std::path::Path;
use image::{ImageBuffer, RgbaImage};
use crate::types::Dimensions2d;

pub fn write_png_to_file(
    dir: &str,
    file_name: &str,
    image_buf: &Vec<u32>,
    image_dims: &Dimensions2d,
) -> std::io::Result<()> {
    // Convert u32 0RGB buffer to a Vec<u8> containing RGBA data
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

    // Create an ImageBuffer from the RGBA buffer
    let img_buffer: RgbaImage = ImageBuffer::from_raw(image_dims.w, image_dims.h, rgba_buffer)
        .ok_or("Failed to create ImageBuffer")
        .unwrap();

    let file_name_png = format!("{}.sample.png", file_name);
    let png_path = Path::new(&dir).join(file_name_png);
    // Save the ImageBuffer as a PNG file
    img_buffer.save(Path::new(&png_path)).unwrap();
    Ok(())
}
