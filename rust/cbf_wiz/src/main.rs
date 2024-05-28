use std::process;
use clap::Parser;
use crate::cli::CliArguments;
use crate::utils::io::file::read_image;
use crate::utils::log::print_verbose;

mod cli;
mod json;

mod utils;
mod font;
mod types;


mod glyph_helper;

fn main() {
    let cli_args = CliArguments::parse();
    let CliArguments { font_image_path, json_info_path,verbose, output_dir} = cli_args;
    print_verbose(&format!("Reading {}", json_info_path), verbose);

    // Read and parse the font info from the JSON
    // ==========================================
    let font_info = json::read_json(&json_info_path, verbose).unwrap_or_else(|e| {
        eprintln!("Failed to read or parse the font JSON config file {json_info_path}.\n{}", e);
        process::exit(1);
    });
    print_verbose(&format!("Font summary:\n\t{font_info}"), verbose);



    // Read and parse the font source image
    // ====================================
    let mut font_image_buf: Vec<u32> = Vec::new();
    print_verbose("Reading the font image", verbose);
    let image_dimensions = read_image(&font_image_path, &mut font_image_buf, verbose).unwrap_or_else(| e|  {
        eprintln!("Failed to read the font image or its dimensions. {}\n\t",e);
        process::exit(1);
    });

    print_verbose(&format!("Font source image width: {}, height: {} ",image_dimensions.w,image_dimensions.h), verbose);

    let char_widths = glyph_helper::get_width(&font_image_buf, &image_dimensions, &font_info.char_order, verbose);
}
