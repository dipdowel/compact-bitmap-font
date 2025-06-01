use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::process;

use clap::Parser;
use graph1::primitives::plane::Dimensions2d;

use crate::cli::CliArguments;
use crate::font_wiz::{PixelFontMetaWiz, PixelFontWiz};
use crate::sampler::make_sample;
use crate::types::Dimensions2dWiz;
use crate::utils::bit_operations;
use crate::utils::io::density;
use crate::utils::io::file::read_image;
use crate::utils::log::print_verbose;
use crate::utils::text::utf8_char_to_u16_vec;

mod cli;
mod json;

mod font_wiz;
mod types;
mod utils;

mod font_constants;
mod glyph_helper;
mod io_helper;
mod sampler;

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

    // Buffer for font with the glyph width marks.
    // It should be used for figuring out the glyph widths.
    let mut font_image_buf: Vec<u32> = Vec::new();

    print_verbose("Reading the font image", verbose);
    
    let image_dimensions = read_image(&font_image_path, &mut font_image_buf, verbose)
        .unwrap_or_else(|e| {
            eprintln!("Failed to read the font image or its dimensions. {}\n\t", e);
            process::exit(1);
        });

    let PixelFontWiz {
        char_order,
        default_char,
        spacing,
        sample_text,
        meta,
    } = font_info;
    let PixelFontMetaWiz {
        font_name,
        author_signature,
        date_day,
        date_year,
        date_month,
        font_ver,
    } = meta;

    let spacing_props: u16 = ((spacing.leading_px as u16) << 8) | (spacing.kerning_px as u16);
    let month_day: u16 = ((date_day as u16) << 8) | (date_month as u16);

    // Get the first character
    let default_char = if let Some(first_char) = default_char.chars().next() {
        first_char
    } else {
        '?'
    };

    let default_char_parts = utf8_char_to_u16_vec(default_char);

    let char_widths_map = glyph_helper::get_width(
        &font_image_buf,
        &image_dimensions.original,
        &char_order,
        verbose,
    );

    let char_widths: Vec<u8> = char_order
        .chars()
        .map(|ch| *char_widths_map.get(&ch).unwrap())
        .collect();

    let (font_image_buf_dense, dense_dimensions) =
        density::condense(&font_image_buf, &image_dimensions, &char_widths, verbose);

    print_verbose(
        &format!(
            "Font source image width: {}, height: {} ",
            image_dimensions.original.w, image_dimensions.original.h
        ),
        verbose,
    );

    print_verbose(
        &format!(
            "Font image to pack (dense). Width: {}, height: {} ",
            dense_dimensions.w, dense_dimensions.h
        ),
        verbose,
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

    // SAVE THE CBF DATA TO A FILE
    // ----------------------------
    let file_name = font_name.replace(" ", "_");
    let file_name_cbf = format!("{}.cbf", file_name);
    let mut cbf_path: PathBuf = PathBuf::new();

    io_helper::write_cbf_to_file(
        &output_dir,
        &file_name_cbf,
        &font_header,
        &font_body,
        &mut cbf_path,
    )
    .unwrap_or_else(|e| {
        eprintln!("Failed to write the resulting font to {output_dir}.\n{}", e);
        process::exit(1);
    });

    // READ THE CBF DATA BACK FROM THE FILE
    // -------------------------------------
    let file = File::open(cbf_path.clone());

    let mut cbf_data: Vec<u8> = Vec::new();
    file.unwrap()
        .read_to_end(&mut cbf_data)
        .unwrap_or_else(|e| {
            let path = cbf_path.to_string_lossy();
            eprintln!("Failed to read the created CBF data from {}.\n{}", path, e);
            process::exit(1);
        });

    // CONSTRUCT FONTS BASED ON THE CBF DATA
    // -------------------------------------

    // TODO:
    // TODO:
    // TODO: The code below is of a prototype quality.
    // TODO: 1. Place it into (a) dedicated file(s).
    // TODO: 2. Come up with a creative visualisation of the text sample
    // TODO: 3. Use the `sample_text` field from JSON
    // TODO: 4. Add more verbose output for the `-v` CLI flag
    // TODO: 5. Test that Leading and Kerning are handled correctly
    // TODO:

    let sample_image_size: Dimensions2d = Dimensions2d { w: 1800, h: 768 };

    let image_buf = make_sample(cbf_data, sample_text, &sample_image_size);

    // SAVE THE SAMPLE PNG TO FILE
    // ----------------------------
    io_helper::write_png_to_file(
        &output_dir,
        &file_name,
        &image_buf,
        &Dimensions2dWiz {
            w: sample_image_size.w,
            h: sample_image_size.h,
        },
    )
    .unwrap_or_else(|e| {
        eprintln!("Failed to write the sample PNG to:  {output_dir}.\n{}", e);
        process::exit(1);
    });
}
