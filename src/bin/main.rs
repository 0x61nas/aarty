use std::{
    io::{self, BufWriter, Write},
    process,
};

use aarty::ToTextImage;
use image::{imageops::FilterType, GenericImageView};

use crate::args::Opts;

mod args;

fn main() {
    // Parse the arguments
    let opts = match Opts::from_args() {
        Ok(opts) => opts,
        Err(e) => {
            eprintln!("Encounter an error while prasing the arguments\n{e}");
            process::exit(1);
        }
    };

    let image = match image::open(opts.path.clone()) {
        Ok(image) => image,
        Err(e) => {
            eprintln!("Failed to open image: {e}");
            process::exit(2);
        }
    };
    let (mut w, mut h) = image.dimensions();
    let mut sf = 0b11u8;
    if let Some(width) = opts.width {
        w = width;
        sf &= 0b01;
    }
    if let Some(height) = opts.height {
        h = height;
        sf &= 0b10;
    }
    if opts.scale > 1 {
        let scale = opts.scale;
        if sf & 0b10 != 0 {
            w /= scale;
        }
        if sf & 0b01 != 0 {
            h /= scale;
        }
    }
    let image = image.resize(w, h, FilterType::Nearest);

    let mut image = image.to_text(&opts.sym_set);

    if let Some(_) = &opts.background {
        // TODO: parse the color like `lanterna`
        image = image.with_background((255, 208, 187));
    }

    image.no_colors = opts.no_colors();
    image.reverse = opts.reverse();

    let image = image.to_string();
    let bytes = image.as_bytes();

    let mut out = BufWriter::with_capacity(bytes.len(), Box::new(io::stdout().lock()));

    out.write_all(bytes).expect("Can't write the output");
}
