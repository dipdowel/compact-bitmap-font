use crate::types::Dimensions2dWiz;
use crate::utils::io::file::FontImageDim;
use crate::utils::log::print_verbose;


/// Throws away the spaces between the glyphs in the image buffer and the top row of pixels,
/// where the separation markers are located.
/// 
/// # Arguments
/// 
/// * `font_img_buf` - The image buffer containing the font data.
/// * `image_dimensions` - The dimensions of the image.
/// * `char_widths` - The widths of the characters.
/// * `verbose` - A boolean flag to control verbosity of the output.

pub fn condense(
    font_img_buf: &Vec<u32>,
    image_dimensions: &FontImageDim,
    char_widths: &Vec<u8>,
    verbose: bool,
) -> (Vec<u32>, Dimensions2dWiz) {
    // pre-compute x values where spaces between the glyphs are
    let mut spaces: Vec<u32> = Vec::new();
    let mut offset: u32 = 0;

    for i in 0..char_widths.len() {
        let char_width = char_widths[i];
        spaces.push(offset);
        offset += char_width as u32 + 1;
    }
    spaces.push(offset);

    print_verbose(&format!("{:?} spaces:", spaces), verbose);

    let mut dbg_img_buf: Vec<u32> = Vec::new();

    let Dimensions2dWiz { h, w } = image_dimensions.original;

    // Access the image's pixels.
    for y in 1..h {
        for x in 0..w {
            let pixel = font_img_buf[(y * w + x) as usize];
            let r = ((pixel >> 16) & 0xFF) as u8;
            let g = ((pixel >> 8) & 0xFF) as u8;
            let b = (pixel & 0xFF) as u8;

            let mut pixel_rgba: u32 = 0xff_ff_ff_ff;
            if r != 0xff && g != 0xff && b != 0xff {
                pixel_rgba = 0x00_00_00_00;
            }

            if !spaces.contains(&x) {
                dbg_img_buf.push(pixel_rgba);
            }
        }
    }

    let dense_w = w - spaces.len() as u32;
    let dense_h = h - 1; // since we skipped the first row of pixels

    /*
    // SAVE THE DEBUG PNG TO FILE
    // --------------------------------------
    io_helper::write_png_to_file(
        &"/home/leo/temp/cbf/".to_string(),
        &"debug-cbf.png".to_string(),
        &dbg_img_buf,
        &Dimensions2dWiz {
            w: dense_w,
            h: dense_h,
        },
    )
        .unwrap_or_else(|e| {
            eprintln!("Failed to write the debug PNG!.\n{}", e);
            process::exit(1);
        });
    // --------------------------------------
    */

    (
        dbg_img_buf,
        Dimensions2dWiz {
            w: dense_w,
            h: dense_h,
        },
    )
}
