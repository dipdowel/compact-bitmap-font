use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process;

use clap::Parser;
use graph1::graph1_core::context::{ContextWindow, GraphContext};
use graph1::primitives::primitives::Point;
use graph1::text::{font, font_embedder, printer};
use graph1::text::printer::ColorProperties;
use image::{EncodableLayout, ImageBuffer, Pixel, RgbaImage};

use crate::cli::CliArguments;
use crate::font_wiz::{PixelFontMetaWiz, PixelFontWiz};
use crate::utils::bit_operations;
use crate::utils::io::file::read_image;
use crate::utils::log::print_verbose;
use crate::utils::text::utf8_char_to_u16_vec;
use crate::io_helper::write_cbf_to_file;

mod cli;
mod json;

mod font_wiz;
mod types;
mod utils;

mod font_constants;
mod glyph_helper;
mod io_helper;

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

    // This buffer will contain font with the glyph width marks.
    // It should be used for figuring out the glyph widths.
    let mut font_image_buf: Vec<u32> = Vec::new();

    // This buffer will contain a cropped image with just the font (the size marks removed).
    // It should be packed into the resulting CBF file.
    let mut font_image_buf_cropped: Vec<u32> = Vec::new();

    print_verbose("Reading the font image", verbose);
    let image_dimensions = read_image(
        &font_image_path,
        &mut font_image_buf,
        &mut font_image_buf_cropped,
        verbose,
    )
    .unwrap_or_else(|e| {
        eprintln!("Failed to read the font image or its dimensions. {}\n\t", e);
        process::exit(1);
    });

    print_verbose(
        &format!(
            "Font source image width: {}, height: {} ",
            image_dimensions.original.w, image_dimensions.original.h
        ),
        verbose,
    );
    print_verbose(
        &format!(
            "Font image to pack. Width: {}, height: {} ",
            image_dimensions.cropped.w, image_dimensions.cropped.h
        ),
        verbose,
    );

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
        &image_dimensions.original,
        &char_order,
        verbose,
    );

    let char_widths: Vec<u8> = char_order
        .chars()
        .map(|ch| *char_widths_map.get(&ch).unwrap())
        .collect();

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
    font_header[6] = image_dimensions.cropped.w as u16;
    font_header[7] = image_dimensions.cropped.h as u16;
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
    font_body.extend(bit_operations::rgb_to_one_bit_image(
        &font_image_buf_cropped,
    ));

    // SAVE THE CBF DATA TO A FILE
    // ----------------------------
    let file_name = font_name.replace(" ", "_");
    let file_name_cbf = format!("{}.cbf", file_name);
    let mut cbf_path: PathBuf = PathBuf::new();
    write_cbf_to_file(
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
    // TODO: 5. Fix the font version and CBF version in the `to_string()` of a font
    // TODO:

    let font_1 = font_embedder::instantiate_external_font(&cbf_data.clone(), 1, None, None);
    let font_2 = font_embedder::instantiate_external_font(&cbf_data.clone(), 2, None, None);
    let font_4 = font_embedder::instantiate_external_font(&cbf_data.clone(), 4, None, None);
    let font_8 = font_embedder::instantiate_external_font(&cbf_data.clone(), 8, None, None);
    let font_16 = font_embedder::instantiate_external_font(&cbf_data.clone(), 16, None, None);

    let context_window: ContextWindow = ContextWindow {
        w: 1024,
        h: 768,
        w_usize: 1024_usize,
        h_usize: 768_usize,
    };

    let buf_size = context_window.w_usize * context_window.h_usize;
    let mut image_buf: Vec<u32> = vec![0xff_77_00_77; buf_size];

    let mut ctx: GraphContext = GraphContext {
        buf_view: &mut image_buf,
        win: &context_window,
        default_color: 0x00_ff_00_00,
    };

    printer::print_line(
        &mut ctx,
        &Point { x: 0, y: 0 },
        &font_1,
        &ColorProperties {
            color_transformer: None,
            color: Some(0xff_ff_00_ff),
        },
        &format!("{}{}", font::DEFAULT_CHAR_ORDER, font::DEFAULT_CHAR_ORDER),
    );

    printer::print_line(
        &mut ctx,
        &Point { x: 0, y: 20 },
        &font_2,
        &ColorProperties {
            color_transformer: None,
            color: Some(0xff_ff_00_33),
        },
        &format!("{}{}", font::DEFAULT_CHAR_ORDER, font::DEFAULT_CHAR_ORDER),
    );

    printer::print_line(
        &mut ctx,
        &Point { x: 0, y: 40 },
        &font_4,
        &ColorProperties {
            color_transformer: None,
            color: Some(0xff_ff_00_77),
        },
        &font_4.to_string(),
    );

    printer::print_line(
        &mut ctx,
        &Point { x: 0, y: 80 },
        &font_8,
        &ColorProperties {
            color_transformer: None,
            color: Some(0xff_ff_00_ff),
        },
        &format!("{}{}", font::DEFAULT_CHAR_ORDER, font::DEFAULT_CHAR_ORDER),
    );

    printer::print_line(
        &mut ctx,
        &Point { x: 0, y: 160 },
        &font_16,
        &ColorProperties {
            color_transformer: None,
            color: Some(0xff_ff_00_ff),
        },
        &format!("{}{}", font::DEFAULT_CHAR_ORDER, font::DEFAULT_CHAR_ORDER),
    );

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
    let img_buffer: RgbaImage =
        ImageBuffer::from_raw(context_window.w, context_window.h, rgba_buffer)
            .ok_or("Failed to create ImageBuffer")
            .unwrap();

    let file_name_png = format!("{}.sample.png", file_name);
    let png_path = Path::new(&output_dir).join(file_name_png);
    // Save the ImageBuffer as a PNG file
    img_buffer.save(Path::new(&png_path)).unwrap();
}
