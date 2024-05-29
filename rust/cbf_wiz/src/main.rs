use crate::cli::CliArguments;
use crate::font::{PixelFont, PixelFontMeta};
use crate::utils::io::file::read_image;
use crate::utils::log::print_verbose;
use clap::Parser;
use std::process;
use image::EncodableLayout;
use crate::utils::bit_operations;
use crate::utils::text::utf8_char_to_u16_vec;
use crate::write_result_helper::create_dir_and_write_file;
use std::io::Write;
mod cli;
mod json;

mod font;
mod types;
mod utils;

mod font_constants;
mod glyph_helper;
mod write_result_helper;

fn main() {
    let cli_args = CliArguments::parse();
    let CliArguments {
        font_image_path,
        json_info_path,
        verbose,
        output_dir,
    } = cli_args;
    print_verbose(&format!("Reading {}", json_info_path), verbose);

    // Read and parse the font info from the JSON
    // ==========================================
    let font_info = json::read_json(&json_info_path, verbose).unwrap_or_else(|e| {
        eprintln!(
            "Failed to read or parse the font JSON config file {json_info_path}.\n{}",
            e
        );
        process::exit(1);
    });
    print_verbose(&format!("Font summary:\n\t{font_info}"), verbose);

    // Read and parse the font source image
    // ====================================
    let mut font_image_buf: Vec<u32> = Vec::new();
    print_verbose("Reading the font image", verbose);
    let image_dimensions = read_image(&font_image_path, &mut font_image_buf, verbose)
        .unwrap_or_else(|e| {
            eprintln!("Failed to read the font image or its dimensions. {}\n\t", e);
            process::exit(1);
        });

    print_verbose(
        &format!(
            "Font source image width: {}, height: {} ",
            image_dimensions.w, image_dimensions.h
        ),
        verbose,
    );



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

    // default_char.


    // Get the first character
    let default_char = if let Some(first_char) = default_char.chars().next() {
        first_char
    } else {
        '?'
    };


    let default_char_parts = utf8_char_to_u16_vec(default_char);

    let char_widths_map = glyph_helper::get_width(
        &font_image_buf,
        &image_dimensions,
        &char_order,
        verbose,
    );


    let char_widths: Vec<u8> = char_order.chars().map(|ch| { *char_widths_map.get(&ch).unwrap() }).collect();

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
    font_header[6] = image_dimensions.w as u16;
    font_header[7] = image_dimensions.h as u16;
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
    font_body.extend(bit_operations::rgb_to_one_bit_image(&font_image_buf));


    // SAVE AND WRITE
    // --------------

    let file_name = format!("{}.cbf",font_name);

    create_dir_and_write_file(&output_dir, &file_name, &font_header, &font_body).unwrap_or_else(|e| {
        eprintln!(
            "Oops... Failed to write the resulting font to {output_dir}.\n{}",
            e
        );
        process::exit(1);
    });



}
