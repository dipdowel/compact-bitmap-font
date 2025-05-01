use graph1::core::context::{GraphContext, WindowContext};
use graph1::draw;
use graph1::primitives::plane::{Dimensions2d, RectArea};
use graph1::primitives::point::Point;
use graph1::text::printer::{Align, ColorProperties};
use graph1::text::{font_embedder, printer};
use graph1::text::font::Spacing;
use graph1::utils::color;
use graph1::utils::color::palettes::{OceanBreeze, RetroNeon, TropicalParadise};
use crate::utils::text;

/// Generates and returns an image using the font provided as CBF data.
pub fn make_sample(
    mut cbf_data: Vec<u8>,
    sample_text: Vec<String>,
    sample_image_size: &Dimensions2d,
) -> Vec<u32> {
    let margin: u32 = 12;

    // Make the font and scale it up to 2, 4 and 8
    let font_1 = font_embedder::instantiate_external_font(&cbf_data.clone(), 1, None, None);
    let font_1_h = font_1.img_dimensions.h;

    let font_2 = font_embedder::instantiate_external_font(&cbf_data.clone(), 2, None, None);
    let font_2_h = font_2.img_dimensions.h;

    let font_4 = font_embedder::instantiate_external_font(&cbf_data.clone(), 4, None, None);
    let font_4_h = font_4.img_dimensions.h;

    let mut font_8 = font_embedder::instantiate_external_font(&cbf_data.clone(), 8, None, None);
    let font_8_h = font_8.img_dimensions.h;
    font_8.spacing = Spacing {
        kerning_px: 7,
        leading_px: 4,
    };

    let ctx_win: WindowContext = WindowContext::new(
        sample_image_size.w,
        sample_image_size.h,


        Some(RetroNeon::DEEP_SPACE_BLUE ),
        Some(TropicalParadise::SAND_YELLOW),
    );

    // Prepare our image buffer
    let buf_size = ctx_win.w_usize * ctx_win.h_usize;
    let mut image_buf: Vec<u32> = vec![RetroNeon::STROBE_WHITE; buf_size];

    let mut ctx: GraphContext = GraphContext::new(ctx_win, false, false, None, 1, None);


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
            color: Some(RetroNeon::STROBE_WHITE),
            data: None,
        },
        &font_1.to_string(),
    );

    // PRINT THE WHOLE CHARSET, SCALE 1X
    //----------------------------------
    let block_1_y = margin + font_1_h + margin;
    let w = ctx.win.w;
    draw::rectangle::filled(
        &mut ctx,
        &RectArea {
            top_left: Point { x: 0, y: block_1_y },
            dimensions: Dimensions2d {
                w,
                h: font_1_h + margin,

            },
            color: Some(OceanBreeze::HARBOR_NAVY ),
        },

    );

    printer::print_line(
        &mut ctx,
        &Point {
            x: margin,
            y: block_1_y + (font_1_h / 2),
        },
        &font_1,
        &ColorProperties {
            color_transformer: None,
            color: Some(OceanBreeze::LIGHT_SAND ),
            data: None,
        },
        &format!("1x:{}", font_1.char_order.clone()),
    );

    // PRINT THE WHOLE CHARSET, SCALE 2X
    //----------------------------------

    let block_2_y = block_1_y + margin + font_1_h;

    let w = ctx.win.w;
    let h = font_2_h + margin;

    draw::rectangle::filled(
        &mut ctx,
        &RectArea {
            top_left: Point { x: 0, y: block_2_y },
            dimensions: Dimensions2d { w, h },
            color: Some(TropicalParadise::MANGO_ORANGE),
        },
    );

    printer::print_line(
        &mut ctx,
        &Point {
            x: margin,
            y: block_2_y + margin / 2,
        },
        &font_2,
        &ColorProperties {
            color_transformer: None,
            color: Some(TropicalParadise::OCEAN_DEEP),
            data: None,
        },
        &format!("2x:{}", font_2.char_order.clone()),
    );

    // PRINT THE WHOLE CHARSET, SCALE 4X
    //----------------------------------

    let block_4_y = block_2_y + margin + font_2_h;
    let w = ctx.win.w;
    let h = font_4_h + margin;
    draw::rectangle::filled(
        &mut ctx,
        &RectArea {
            top_left: Point { x: 0, y: block_4_y },
            dimensions: Dimensions2d { w, h },
            color: Some(TropicalParadise::HIBISCUS_RED),
        },
    );

    printer::print_line(
        &mut ctx,
        &Point {
            x: margin,
            y: block_4_y + (margin),
        },
        &font_4,
        &ColorProperties {
            color_transformer: Some(|color: u32, x: u32, y: u32, w: u32, h: u32, data| -> u32 {
                if y % 2 == 0 {
                    return RetroNeon::GLITCH_RED ;
                }
                return RetroNeon::STROBE_WHITE ;
            }),
            // color: Some(0xff_ee_00_ee),
            color: None,
            data: None,
        },
        &format!("4x:{}", font_4.char_order.clone()),
    );

    // PRINT THE WHOLE CHARSET, SCALE 8X
    //----------------------------------

    let block_8_y = block_4_y + margin + font_4_h;

    let w = ctx.win.w;
    let h = font_8_h + margin;
    draw::rectangle::filled(
        &mut ctx,
        &RectArea {
            top_left: Point { x: 0, y: block_8_y },
            dimensions: Dimensions2d { w, h },
            color: Some(RetroNeon::DEEP_SPACE_BLUE ),
        },
    );


    

    printer::print_line(
        &mut ctx,
        &Point {
            x: margin,
            y: block_8_y + margin,
        },
        &font_8,
        &ColorProperties {
            color_transformer: Some(|color: u32, x: u32, y: u32, w: u32, h: u32, data| -> u32 {
                if y % 2 == 0 {
                    return 0x000000ff ;
                }
                // return RetroNeon::STROBE_WHITE ;
                return RetroNeon::ELECTRIC_BLUE ;
            }),
            // color: Some(0xff_ee_00_ee),
            color: None,
            data: None,
        },
    
        &format!("8x:{}", font_8.char_order.clone()),
    
    );

    // PRINT PROVIDED SAMPLE TEXT (SCALE 1X)
    //--------------------------------------

    let block_text_y = block_8_y + margin + font_8_h;
    let w = ctx.win.w;
    let h = ctx.win.h - block_text_y;

    draw::rectangle::filled(
        &mut ctx,
        &RectArea {
            top_left: Point {
                x: 0,
                y: block_text_y,
            },
            dimensions: Dimensions2d { w, h },
            color: Some(OceanBreeze::LIGHT_SAND),
        },
    );

    let text: Vec<&str> = sample_text.iter().map(|s| s.as_str()).collect();

    printer::print(
        &mut ctx,
        &Point {
            x: margin*2,
            y: block_text_y + margin*2,
        },
        &font_1,
        &ColorProperties {
            color_transformer: None,
            color: Some(0x00_00_00_ff),
            data: None,
        },
        &text,
        Align::Left,
    );

    color::adapters::rgba_to_argb(
        &mut image_buf,
        &ctx.frame_buf,
        false
    );

    image_buf
}
