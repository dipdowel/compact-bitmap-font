use std::collections::HashMap;
use crate::types::Dimensions2d;
use crate::utils::log::print_verbose;

pub fn get_width(image_buf: &Vec<u32>, image_dimensions: &Dimensions2d, char_order:&str, verbose:bool) -> HashMap<char,u8>{

    let ordered_font_chars:Vec<char> = char_order.chars().collect();

    let total_width: usize = image_dimensions.w as usize;
    let mut cur_char_index:usize = 0;
    let mut cur_char_width:usize = 0;
    let mut is_over_glyph = false;
    let mut result:HashMap<char, u8> = HashMap::new();

    print_verbose(&format!("Mapping chars to their widths. Total chars found: {}", ordered_font_chars.len()), verbose);

    for x in 1..total_width {
        is_over_glyph = x>0 && image_buf[x] == 0x_00_ff_ff_ff;
        if is_over_glyph {
            cur_char_width +=1
        } else {
            if cur_char_width > 0 {
                // save the character
                let ch = ordered_font_chars[cur_char_index];
                print_verbose(&format!("\t{} -> {}", ch, cur_char_width), verbose);
                result.insert(ch, cur_char_width as u8);
            }
            cur_char_index += 1;
            cur_char_width = 0;
        }
    }


    return result;
}

// TODO Unit tests!