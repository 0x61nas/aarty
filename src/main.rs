use std::io::Write;
use clap::Parser;
use image::GenericImageView;


extern crate pretty_env_logger;
#[macro_use]
extern crate log;

mod args;
mod ascii_processor;

use crate::args::{
    args::Arguments,
    enums::OutputMethod,
};

use crate::ascii_processor::generate_ascii;

fn main() {
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

    // Process the image
    let output = generate_ascii(image, &arguments);
    info!("Successfully processed image");

    // Output the image
    info!("Outputting image");
    match arguments.output_method {
        OutputMethod::File => {
            match std::fs::write(arguments.output.clone(),
                                 output.iter().map(|s| s.to_string()).collect::<String>()) {
                Ok(_) => info!("Successfully outputted image: {}", arguments.output),
                Err(e) => {
                    error!("Failed to output image: {:?}", e);
                    eprintln!("Failed to output image: {:?}", e);
                    std::process::exit(1);
                }
            }
        }
        OutputMethod::Stdout => {
            for char in output {
                print!("{}", char);
                std::io::stdout().flush().unwrap();
            }
            info!("Successfully outputted image");
        }
    }
}
