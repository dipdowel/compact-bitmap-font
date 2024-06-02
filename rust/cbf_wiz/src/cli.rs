use clap::{ArgGroup, Parser};

#[derive(Parser, Debug)]
#[command(
    name = "CBF Wizard",
    version = "0.1.0",
    author = "Leo Boguslavskiy <info@codument.com>",
    about = "\n\n\
    =========================\n\
    CBF = Compact Bitmap Font\n\
    =========================\n\
    This wizard allows generating and viewing bitmap/pixel fonts in CBF format.\n\
    The font is based on a designs provided as an image (primarily PNG) \n\
    and font configuration JSON.\n\
    Please see README.md for details and examples."
)]

// FIXME: Use the ArgGroup for generate / display

// #[command(group(
//     ArgGroup::new("verbosity")
//     .required(false)
//     .args(&["debug", "quiet"])
// ))]
pub struct CliArguments {
    /// Image with glyphs of the font
    #[arg(short = 'i', long = "image", value_name = "source-font-image.[png]")]
    pub font_image_path: String,

    /// JSON file with the font configuration
    #[arg(short = 'j', long = "json", value_name = "font-configuration.json")]
    pub json_info_path: String,

    /// Generated font and sample(s) will be placed into this directory
    #[arg(short = 'o', long = "outdir", value_name = "output-directory")]
    pub output_dir: String,

    /// Verbose mode
    #[arg(short, long)]
    pub verbose: bool,
    // /// Input file to use
    // #[arg(value_name = "font-source-image.[png]", index = 1)]
    // pub src_image_path: String,
    //
    // /// Input file to use
    // #[arg(value_name = "font_image", index = 2)]
    // pub json_info_path: String,
    //
    // /// Turn on debugging information
    // #[arg(short, long)]
    // pub debug: bool,
    //
    // /// Suppress output
    // #[arg(short, long)]
    // pub quiet: bool,
}
