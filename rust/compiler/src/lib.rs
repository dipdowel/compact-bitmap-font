// == [ MODULES ] ==================================================================================
mod error;
mod font_constants;

mod types;

pub mod utils {
    pub(crate) mod bit_operations;

    pub(crate) mod density;
    
    pub(crate) mod make_cbf_file_data;
    pub(crate) mod make_png;
    pub(crate) mod text;
}

pub(crate) mod glyph_helper;
pub(crate) mod sampler;

// == [ MACROS ] ===================================================================================
#[macro_use]
pub mod macros;

// == [ IMPORTS ] ==================================================================================

use crate::types::{CompilationResult, Dimensions2d, PixelFont, PixelFontMeta};
use miniserde::json;

use crate::error::CompileFontError;
use crate::sampler::make_sample;
use crate::utils::make_cbf_file_data::make_cbf_file_data;
use crate::utils::make_png::pixels_to_png;
use crate::utils::text::utf8_char_to_u16_vec;
use crate::utils::{bit_operations, density};
use std::io::Cursor;
use std::path::Path;

/// Compiles a CBF-font from a PNG image and a JSON configuration string.
/// # Arguments:
/// * `font_image_buf`: A byte slice containing the PNG image data of the font.
/// * `font_info_json`: A JSON string containing the font metadata and configuration.
/// * `verbose`: A boolean flag to control the verbosity of the output.
/// # Returns:
/// * `Result<CompilationResult, CompileFontError>`: On success, returns a `CompilationResult` containing the compiled font data and metadata. On failure, returns a `CompileFontError`.
///
pub fn compile_font(
    font_image_buf: &[u8],
    font_info_json: String,
    verbose: bool,
) -> Result<CompilationResult, CompileFontError> {
    // Deserialize the JSON string into a PixelFont struct
    let font_info: PixelFont = json::from_str(&font_info_json)?;

    // Decode the PNG image from the buffer
    let font_image = image::load(Cursor::new(font_image_buf), image::ImageFormat::Png)?;

    let src_img_dimensions: Dimensions2d = Dimensions2d {
        w: font_image.width(),
        h: font_image.height(),
    };

    vprintln!(verbose, "font_info decoded from JSON:\n{:#?}", font_info);

    // Convert the image to `Vec<u32>`, where `u32` is a pixel in `RGBA`.
    let buf_marked = font_image
        .to_rgba8()
        .into_raw()
        .chunks_exact(4)
        .map(|px| u32::from_le_bytes([px[0], px[1], px[2], px[3]]))
        .collect();

    let PixelFont {
        char_order,
        default_char,
        spacing,
        sample_text,
        meta,
    } = font_info;

    let PixelFontMeta {
        font_name,
        author_signature,
        date_day,
        date_year,
        date_month,
        font_ver,
    } = meta;

    let spacing_props: u16 = ((spacing.leading_px as u16) << 8) | (spacing.kerning_px as u16);
    let month_day: u16 = ((date_day as u16) << 8) | (date_month as u16);

    // Get the first (and the only) character that will be used as the default one.
    let default_char = if let Some(first_char) = default_char.chars().next() {
        first_char
    } else {
        '?'
    };

    let default_char_parts = utf8_char_to_u16_vec(default_char);

    let char_widths_map =
        glyph_helper::get_width(&buf_marked, &src_img_dimensions, &char_order, verbose);

    // vprintln!(verbose, "char_widths_map: {:#?}", char_widths_map);

    let char_widths: Vec<u8> = char_order
        .chars()
        .map(|ch| *char_widths_map.get(&ch).unwrap())
        .collect();

    let (font_image_buf_dense, dense_dimensions) =
        density::condense(&buf_marked, &src_img_dimensions, &char_widths, verbose);

    vprintln!(
        verbose,
        "\n--> Font source image width: {}, height: {}",
        src_img_dimensions.w,
        src_img_dimensions.h
    );
    vprintln!(
        verbose,
        "--> Font image to pack (dense). Width: {}, height: {}\n",
        dense_dimensions.w,
        dense_dimensions.h
    );

    // FILL IN THE HEADER
    // ------------------
    let mut font_header: Vec<u16> = vec![0; 14];

    // File identification
    font_header[0] = font_constants::CBF_MAGIC_NUMBER; // The `CBF0` magic number
    font_header[1] = font_constants::CBF_VERSION; // CBF format version

    // Sizes of the variable-length data fields
    font_header[2] = font_name.len() as u16;
    font_header[3] = author_signature.len() as u16;
    font_header[4] = char_order.len() as u16;
    font_header[5] = char_widths.len() as u16;

    // Font image and font properties
    font_header[6] = dense_dimensions.w as u16;
    font_header[7] = dense_dimensions.h as u16;
    font_header[8] = spacing_props;

    // The font's default char (utf8, hence can be up to 4 bytes, hence 2 u16 values needed.
    font_header[9] = default_char_parts[0];
    font_header[10] = default_char_parts[1];

    // Date of creation of the font
    font_header[11] = font_ver;
    font_header[12] = date_year;
    font_header[13] = month_day;

    // FILL IN THE BODY
    // ----------------
    let mut font_body: Vec<u8> = Vec::from(font_name.clone());
    font_body.extend(author_signature.as_bytes());
    font_body.extend(char_order.as_bytes());
    font_body.extend(char_widths);
    font_body.extend(bit_operations::rgb_to_one_bit_image(&font_image_buf_dense));

    // vprintln!(verbose, "font_header: {:#?}",  font_header);

    vprintln!(
        verbose,
        "font_header: [{}]",
        font_header
            .iter()
            .flat_map(|w| w.to_le_bytes())
            .collect::<Vec<u8>>()
            .into_iter()
            .map(|n| format!("{:02X}", n))
            .collect::<Vec<_>>()
            .join(", ")
    );

    vprintln!(
        verbose,
        "font_body: [{}]",
        font_body
            .iter()
            .map(|n| format!("{:02X}", n))
            .collect::<Vec<_>>()
            .join(", ")
    );

    let cbf_blob = make_cbf_file_data(&font_header, &font_body);

    // Prepare the file name for the CBF file
    let file_name = font_name.replace(" ", "_");
    let file_name_cbf = format!("{}.cbf", file_name);

    let sample_image_size: Dimensions2d = Dimensions2d { w: 1800, h: 768 };

    let font_sample_raw_data = make_sample(
        &cbf_blob,
        &sample_text,
        sample_image_size.w,
        sample_image_size.h,
    );

    let font_sample_png_data = pixels_to_png(&font_sample_raw_data, &sample_image_size)?;

    Ok(CompilationResult {
        file_name: file_name_cbf,
        src_img_dimensions,
        dst_img_dimensions: dense_dimensions,
        cbf_binary_file_data: cbf_blob,
        font_sample_raw_data,
        font_sample_png_data,
        sample_image_size,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    // use crate::utils::io::write_png_to_file;
    use std::fs;
    use std::fs::File;
    use std::io::Write;
    use std::path::Path;

    #[test]
    fn test_compile_font() {
        // Load the test font PNG data from file
        let font_png_data =
            fs::read("test_data/assets/cc_red_alert_inet.png").expect("Failed to read PNG file");

        let font_json_data: String = fs::read_to_string("test_data/assets/cc_red_alert_inet.json")
            .expect("Failed to read JSON file");

        let compiled =
            compile_font(&font_png_data, font_json_data, true).expect("Failed to compile font");

        assert!(compiled.src_img_dimensions.w == 520 && compiled.src_img_dimensions.h == 10);

        assert!(compiled.dst_img_dimensions.w == 424 && compiled.dst_img_dimensions.h == 9);

        // Save the PNG as a PNG file
        let file_name_png = format!("{}.sample.png", compiled.file_name);
        let png_path = Path::new(".").join(file_name_png);

        let mut file = File::create(png_path).unwrap_or_else(|e| {
            panic!("Failed to create PNG file: {}", e);
        });

        file.write_all(&compiled.font_sample_png_data)
            .unwrap_or_else(|e| {
                panic!("Failed to create PNG file: {}", e);
            })
    }
}
