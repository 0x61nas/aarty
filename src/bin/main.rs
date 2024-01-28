#[cfg(not(feature = "image"))]
// TODO: remove this
compile_error!("Needs `image` feature to compile the binary!");

use std::{
    io::{self, BufWriter, Cursor, Read},
    process,
};

use aarty::{convert_image_to_ascii, Config};
use image::{io::Reader, GenericImageView};

use crate::args::Opts;

mod args;

const G_ERR: i32 = 1;
const IO_ERR: i32 = 2;
const OP_ERR: i32 = 3;

fn main() {
    // Parse the arguments
    let opts = match Opts::from_args() {
        Ok(opts) => opts,
        Err(e) => {
            eprintln!("{e}");
            process::exit(G_ERR);
        }
    };

    let Ok(image) = (if let Some(path) = opts.path.as_ref() {
        match Reader::open(path) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("{e}");
                process::exit(IO_ERR);
            }
        }
        .decode()
    } else {
        const CAPACITY: usize = 2 * 1048576; // 2mb
        let mut buf = Vec::with_capacity(CAPACITY);
        if let Err(e) = io::stdin().lock().read_to_end(&mut buf) {
            eprintln!("{e}");
            process::exit(IO_ERR);
        }
        let Ok(reader) = Reader::new(Cursor::new(buf)).with_guessed_format() else {
            eprintln!("Can't read the input");
            process::exit(IO_ERR);
        };
        reader.decode()
    }) else {
        eprintln!("Failed to guess the input format or the input format was unsportted");
        process::exit(OP_ERR);
    };

    let (mut w, mut h) = image.dimensions();
    if let Some(width) = opts.width {
        w = width;
    }
    if let Some(height) = opts.height {
        h = height;
    }
    let image = if opts.scale.get() > 1 {
        let scale = opts.scale.get() as u32;
        image.resize(w / scale, h / scale, opts.sf)
    } else {
        image.resize_exact(w, h, opts.sf)
    };

    let mut config = Config::new(opts.sym_set.into()).with_flags(opts.flags);

    if let Some(_) = &opts.background {
        // TODO: parse the color like `lanterna`
        config = config.with_background((255, 208, 187));
    }

    let buf_size = config.calc_buf_size(w, h);

    let mut out = BufWriter::with_capacity(buf_size, Box::new(io::stdout().lock()));

    if let Err(e) = convert_image_to_ascii(&config, &image, &mut out) {
        eprintln!("Can't write the output: {e}");
        process::exit(IO_ERR);
    }
}
