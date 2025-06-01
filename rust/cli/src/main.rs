use std::path::PathBuf;
use std::{fs, process};
use clap::Parser;
use crate::cli::CliArguments;
use compiler::{compile_font, vprintln};
use compiler::utils::io::write_png_to_file;
mod cli;
mod io_helper;

fn main() {
    let cli_args = CliArguments::parse();
    let CliArguments {
        font_image_path,
        json_info_path,
        verbose,
        output_dir,
    } = cli_args;

    vprintln!(verbose, "Reading assets... {}", json_info_path);

    // Load the font PNG data from file
    let font_png_data = fs::read(font_image_path.clone()).unwrap_or_else(|err| {
        eprintln!(
            "Failed to read the font image file {}.\n{}",
            font_image_path, err
        );
        process::exit(1);
    });

    // Load the font JSON data from file
    let font_json_data = fs::read_to_string(json_info_path.clone()).unwrap_or_else(|err| {
        eprintln!(
            "Failed to read or parse the font JSON config file {json_info_path}.\n{}",
            err
        );
        process::exit(1);
    });

    let compiled = compile_font(&font_png_data, font_json_data, true).unwrap_or_else(|err| {
        eprintln!(
            "Failed to compile a CBF font from the provided assets.\n{}",
            err
        );
        process::exit(1);
    });

    let sample_png_filename = format!("{}.sample.png", compiled.file_name);

    let mut cbf_path: PathBuf = PathBuf::new();

    // SAVE THE CBF DATA TO A FILE
    // ----------------------------
    io_helper::write_cbf_to_file(
        &output_dir,
        &compiled.file_name,
        &compiled.cbf_binary_file_data,
        &mut cbf_path,
    )
    .unwrap_or_else(|e| {
        eprintln!("Failed to write the resulting font to {output_dir}.\n{}", e);
        process::exit(1);
    });

    // SAVE THE SAMPLE PNG TO FILE
    // ----------------------------
    write_png_to_file(
        &output_dir,
        &sample_png_filename,
        &compiled.font_sample_png_data,
        &compiled.font_sample_png_dimensions,
    )
    .unwrap_or_else(|e| {
        eprintln!("Failed to write the sample PNG to:  {output_dir}.\n{}", e);
        process::exit(1);
    });
}
