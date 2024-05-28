use std::fmt::Display;
use std::fs;
use miniserde::json;
use crate::font::PixelFont;
use crate::utils::log::print_verbose;

/// Reads and parses a JSON file at `file_path` into PixelFont
pub fn read_json(file_path: &str, verbose: bool) -> Result<PixelFont, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(file_path)?;
    print_verbose("Parsing JSON", verbose);
    let config: PixelFont = json::from_str(&content)?;
    Ok(config)
}