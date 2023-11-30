use std::{
    io::{self, BufWriter, Cursor, Read, Write},
    process,
};

use aarty::ToTextImage;
use image::{imageops::FilterType, io::Reader, GenericImageView};

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
        const CAPACITY: usize = 1024 * 2; // 2mb
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

    image.flags |= opts.flags;

    let image = image.to_string();
    let bytes = image.as_bytes();

    let mut out = BufWriter::with_capacity(bytes.len(), Box::new(io::stdout().lock()));

    if let Err(e) = out.write_all(bytes) {
        eprintln!("Can't write the output: {e}");
        process::exit(IO_ERR);
    }
}
