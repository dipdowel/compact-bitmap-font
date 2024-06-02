use miniserde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Display;

#[derive(Serialize, Deserialize, Debug)]
pub struct SpacingWiz {
    pub kerning_px: u8,
    pub leading_px: u8,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PixelFontMetaWiz {
    pub font_ver: u16,
    pub date_year: u16,
    pub date_month: u8,
    pub date_day: u8,
    pub font_name: String,
    pub author_signature: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PixelFontWiz {
    pub char_order: String,
    pub default_char: String,
    pub spacing: SpacingWiz,
    pub meta: PixelFontMetaWiz,
    pub sample_text: Vec<String>,
}
impl Display for PixelFontWiz {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let PixelFontMetaWiz {
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
