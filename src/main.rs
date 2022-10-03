use clap::Parser;
use image::GenericImageView;

extern crate pretty_env_logger;
#[macro_use] extern crate log;

mod args;

use crate::args::args::Arguments;

fn main() {
    // Initialize the logger
    pretty_env_logger::init();
    info!("Successfully initialized logger");
    info!("Parsing arguments");
    // Parse the arguments
    let arguments = Arguments::parse();
    info!("Successfully parsed arguments");
    trace!("Arguments: {:?}", arguments);

    // Open the image
    info!("Opening image: {}", arguments.image);
    let image = match image::open(arguments.image) {
        Ok(image) => image,
        Err(e) => {
            error!("Failed to open image: {:?}", e);
            eprintln!("Failed to open image: {:?}", e);
            std::process::exit(1);
        }
    };
    info!("Successfully opened image");
    trace!("Image dimensions: {:?}", image.dimensions());

}
