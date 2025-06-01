use crate::types::Dimensions2dWiz;
use image::{ImageBuffer, RgbaImage};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

/// Saves the CBF data to a file and returns the path to where the file was written
pub fn write_cbf_to_file(
    dir: &str,
    file_name: &str,
    font_header: &[u16],
    font_body: &[u8],
    path_buf: &mut PathBuf,
) -> std::io::Result<()> {
    let path = Path::new(dir);

    if !path.exists() {
        fs::create_dir_all(path)?;
    }

    let file_path = path.join(file_name);

    *path_buf = file_path.clone();

    let mut file = File::create(file_path)?;

    for num in font_header {
        file.write_all(&num.to_le_bytes()).unwrap(); // Using little endian encoding
    }
    for num in font_body {
        file.write_all(&num.to_le_bytes()).unwrap(); // Using little endian encoding
    }
    Ok(())
}

pub fn write_png_to_file(
    dir: &str,
    file_name: &str,
    image_buf: &Vec<u32>,
    image_dims: &Dimensions2dWiz,
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
