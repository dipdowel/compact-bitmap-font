use graph1::draw;
use graph1::graph1_core::context::{ContextWindow, GraphContext};
use graph1::primitives::primitives::{Dimensions2d, Point, RectArea};
use graph1::text::{font_embedder, printer};
use graph1::text::printer::ColorProperties;

/// Generates and returns an image using the font provided as CBF data.
pub fn make_sample(mut cbf_data: Vec<u8>,  sample_text: Vec<String>, sample_image_size:&Dimensions2d) -> Vec<u32> {

    let margin: u32 = 12;

    // Make the font and scale it up to 2, 4 and 8
    let font_1 = font_embedder::instantiate_external_font(&cbf_data.clone(), 1, None, None);
    let font_1_h = font_1.img_dimensions.h;

    let font_2 = font_embedder::instantiate_external_font(&cbf_data.clone(), 2, None, None);
    let font_2_h = font_2.img_dimensions.h;

    let font_4 = font_embedder::instantiate_external_font(&cbf_data.clone(), 4, None, None);
    let font_4_h = font_4.img_dimensions.h;

    let font_8 = font_embedder::instantiate_external_font(&cbf_data.clone(), 8, None, None);
    let font_8_h = font_8.img_dimensions.h;

    let ctx_win: ContextWindow = ContextWindow {
        w: sample_image_size.w,
        h: sample_image_size.h,
        w_usize: sample_image_size.w as usize,
        h_usize: sample_image_size.h as usize,
    };

    // Prepare our image buffer
    let buf_size = ctx_win.w_usize * ctx_win.h_usize;
    let mut image_buf: Vec<u32> = vec![0xff_e0_e0_aa; buf_size];

    let mut ctx: GraphContext = GraphContext {
        buf_view: &mut image_buf,
        win: &ctx_win,
        default_color: 0x00_ff_00_00,
    };

    // Print the font meta-data to the image
    printer::print_line(
        &mut ctx,
        &Point {
            x: margin,
            y: margin,
        },
        &font_1,
        &ColorProperties {
            color_transformer: None,
            color: Some(0xff_00_00_00),
        },
        &font_1.to_string(),
    );

    // PRINT THE WHOLE CHARSET, SCALE 1X
    //----------------------------------
    let block_1_y = margin + font_1_h + margin;
    draw::rectangle_filled(
        &mut ctx,
        &RectArea {
            top_left: Point {
                x: 0,
                y: block_1_y,
            },
            dimensions: Dimensions2d {
                w: ctx_win.w,
                h: font_1_h + margin,
            },
        },
        0xff_00_00_00,
    );

    printer::print_line(
        &mut ctx,
        &Point {
            x: margin,
            y: block_1_y + ( font_1_h / 2),
        },
        &font_1,
        &ColorProperties {
            color_transformer: None,
            color: Some(0xff_ff_ff_ff),
        },
        &format!("1x:{}", font_1.char_order.clone()),
    );


    // PRINT THE WHOLE CHARSET, SCALE 2X
    //----------------------------------

    let block_2_y = block_1_y + margin + font_1_h;
    draw::rectangle_filled(
        &mut ctx,
        &RectArea {
            top_left: Point { x: 0, y: block_2_y },
            dimensions: Dimensions2d {
                w: ctx_win.w,
                h: font_2_h + margin,
            },
        },
        0xff_00_33_00,
    );

    printer::print_line(
        &mut ctx,
        &Point {
            x: margin,
            y: block_2_y + margin /2,
        },
        &font_2,
        &ColorProperties {
            color_transformer: None,
            color: Some(0xff_11_ee_11),
        },
        &format!("2x:{}", font_2.char_order.clone()),
    );


    // PRINT THE WHOLE CHARSET, SCALE 4X
    //----------------------------------

    let block_4_y = block_2_y + margin + font_2_h;

    draw::rectangle_filled(
        &mut ctx,
        &RectArea {
            top_left: Point { x: 0, y: block_4_y },
            dimensions: Dimensions2d {
                w: ctx_win.w,
                h: font_4_h + margin,
            },
        },
        0xff_33_00_33,
    );

    printer::print_line(
        &mut ctx,
        &Point {
            x: margin,
            y: block_4_y + (margin),
        },
        &font_4,
        &ColorProperties {
            color_transformer: Some( |color:u32, x:u32, y:u32, w:u32, h:u32| -> u32  {
                if y % 2 == 0 {
                    return 0xff_bb_55_bb;
                }
                return 0xff_88_22_88
            }),
            // color: Some(0xff_ee_00_ee),
            color: None,
        },
        &format!("4x:{}", font_4.char_order.clone()),
    );

    // PRINT THE WHOLE CHARSET, SCALE 8X
    //----------------------------------

    let block_8_y = block_4_y + margin + font_4_h;

    draw::rectangle_filled(
        &mut ctx,
        &RectArea {
            top_left: Point { x: 0, y: block_8_y },
            dimensions: Dimensions2d {
                w: ctx_win.w,
                h: font_8_h + margin,
            },
        },
        0xff_00_00_44,
    );

    printer::print_line(
        &mut ctx,
        &Point {
            x: margin,
            y: block_8_y + margin,
        },
        &font_8,
        &ColorProperties {
            color_transformer: Some( |color:u32, x:u32, y:u32, w:u32, h:u32| -> u32  {
                if y % 2 == 0 {
                    return 0xff_00_00_ee;
                }
                return 0xff_00_00_55
            }),
            // color: Some(0xff_ee_00_ee),
            color: None,
        },
        &format!("8x:{}", font_8.char_order.clone()),
    );

    // PRINT PROVIDED SAMPLE TEXT (SCALE 1X)
    //--------------------------------------

    let block_text_y = block_8_y + margin + font_8_h;

    draw::rectangle_filled(
        &mut ctx,
        &RectArea {
            top_left: Point { x: 0, y: block_text_y },
            dimensions: Dimensions2d {
                w: ctx_win.w,
                h: ctx_win.h-block_text_y,
            },
        },
        0xff_11_11_11,
    );

    let text: Vec<&str> = sample_text.iter().map(|s| s.as_str()).collect();

    printer::print(
        &mut ctx,
        &Point {
            x: margin,
            y:  block_text_y+margin ,
        },
        &font_1,
        &ColorProperties {
            color_transformer: None,
            color: Some(0xff_ff_ff_ff),
        },
        &text
    );

    return image_buf;
}