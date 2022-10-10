use clap::Parser;
use image::GenericImageView;

use std::io::Result;

extern crate pretty_env_logger;

#[macro_use]
extern crate log;

mod args;
mod ascii_processor;
mod output;

use crate::args::{args::Arguments, enums::OutputMethod};

use crate::ascii_processor::generate_ascii;

fn main() -> Result<()> {
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
    trace!("Image dimensions: {:?}", image.dimensions());

    generate_ascii(image, &arguments, output::prepare_output(&arguments)?)?;
    info!("Successfully processed image");
    Ok(())
}
