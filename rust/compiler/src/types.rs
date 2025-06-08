use miniserde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub struct Dimensions2d {
    /// width in pixels
    pub w: u32,
    /// height in pixels
    pub h: u32,
}

#[derive(Debug)]
/// Struct to hold the result of a font compilation.
pub struct CompilationResult {
    /// The name of the font file: `font-name.cbf`
    pub file_name:String,
    /// The dimensions of the source image used for the font.
    pub src_img_dimensions:Dimensions2d,
    /// The dimensions of the destination image that is used to generate the CBF file.
    pub dst_img_dimensions:Dimensions2d,
    /// The binary data of the compiled font file. The user can write this to a file (e.g using the filename from `file_name`).
    pub cbf_binary_file_data: Vec<u8>,
    /// The font sample image: the RGBA pixels of the image containing the sample text rendered with the font. The sample text is taken from the provided JSON file.
    pub font_sample_raw_data: Vec<u32>,
    /// The font sample image as PNG data, can be written to a `*.png` file.
    pub font_sample_png_data: Vec<u8>,
    /// Dimensions of the font sample image
    pub sample_image_size: Dimensions2d,
}



#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Spacing {
    pub kerning_px: u8,
    pub leading_px: u8,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct PixelFontMeta {
    pub font_ver: u16,
    pub date_year: u16,
    pub date_month: u8,
    pub date_day: u8,
    pub font_name: String,
    pub author_signature: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct PixelFont {
    pub char_order: String,
    pub default_char: String,
    pub spacing: Spacing,
    pub meta: PixelFontMeta,
    pub sample_text: Vec<String>,
}
impl Display for PixelFont {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let PixelFontMeta {
            font_ver,
            date_year,
            date_month,
            date_day,
            font_name,
            author_signature,
        } = &self.meta;

        let output = format!(
            "{} ver. {} | Author: {} | Created: {}-{}-{} ",
            font_name, font_ver, author_signature, date_day, date_month, date_year
        );

        f.write_str(&output)
    }
}
