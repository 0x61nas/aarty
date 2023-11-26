use std::io::Write;

use aarty::ToTextImage;
use clap::Parser;
use image::GenericImageView;

extern crate pretty_env_logger;

#[macro_use]
extern crate log;

mod args;
mod output;

use crate::args::enums::Mode;
use crate::args::{args::Arguments, enums::OutputMethod};

fn main() -> std::io::Result<()> {
    // Initialize the logger
    pretty_env_logger::init();
    info!("Successfully initialized logger");
    info!("Parsing arguments");
    // Parse the arguments
    let arguments = Arguments::parse();
    info!("Successfully parsed arguments");
    trace!("Arguments: {:?}", arguments);

    // Validate the arguments
    info!("Validating arguments");
    match arguments.validate() {
        Ok(_) => (),
        Err(e) => {
            error!("Failed to validate arguments: {}", e);
            eprintln!("Failed to validate arguments: {}", e);
            std::process::exit(1);
        }
    }

    // Open the image
    info!("Opening image: {}", arguments.image);
    let image = match image::open(arguments.image.clone()) {
        Ok(image) => image,
        Err(e) => {
            error!("Failed to open image: {:?}", e);
            eprintln!("Failed to open image: {:?}", e);
            std::process::exit(1);
        }
    };
    info!("Successfully opened image");
    let (w, h) = image.dimensions();
    trace!("Image dimensions: {:?}", image.dimensions());
    let image = image.resize(w, h / 3, image::imageops::FilterType::Nearest);

    let mut image = image.to_text(arguments.characters.chars().collect());

    if let Some(_) = &arguments.background {
        // TODO: parse the color like `lanterna`
        image = image.with_background((255, 208, 187));
    }

    image.no_colors = arguments.mode == Mode::NormalAscii;
    image.reverse = true;

    let mut out = output::prepare_output(&arguments)?;

    out.write_all(image.to_string().as_bytes())?;
    // convert_image_to_ascii(
    //     image,
    //     arguments.mode == Mode::Colored,
    //     arguments.background.clone(),
    //     arguments.characters.chars().collect(),
    //     arguments.width.unwrap_or(arguments.scale),
    //     &mut output::prepare_output(&arguments)?,
    // )?;
    info!("Successfully processed image");
    Ok(())
}
