use crate::font_wiz::PixelFontWiz;
use crate::utils::log::print_verbose;
use miniserde::json;
use std::fmt::Display;
use std::fs;

/// Reads and parses a JSON file at `file_path` into PixelFont
pub fn read_json(
    file_path: &str,
    verbose: bool,
) -> Result<PixelFontWiz, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(file_path)?;
    print_verbose("Parsing JSON", verbose);
    let config: PixelFontWiz = json::from_str(&content)?;
    Ok(config)
}
