use std::{
    io::{self, BufWriter, Cursor, Read, Write},
    process,
};

use aarty::{convert_image_to_ascii, Config, ToTextImage};
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

    let config = Config {
        sympols: (&opts.sym_set).into(),
        bc: {
            if let Some(_) = &opts.background {
                // TODO: parse the color like `lanterna`
                Some((255, 208, 187).into())
            } else {
                None
            }
        },
        flags: opts.flags,
    };

    let buf_size = config.calc_buf_size(w, h);

    // #[cfg(not(feature = "_no_ref"))]
    // let mut image = image.to_text((&opts.sym_set).into());

    // #[cfg(feature = "_no_ref")]
    // let mut image = image.to_text(opts.sym_set.into());

    // if let Some(_) = &opts.background {
    //     // TODO: parse the color like `lanterna`
    //     image = image.with_background((255, 208, 187));
    // }

    // image.flags |= opts.flags;

    // let image = image.to_string();
    // let bytes = image.as_bytes();

    let mut out = BufWriter::with_capacity(buf_size, Box::new(io::stdout().lock()));

    if let Err(e) = convert_image_to_ascii(&config, &image, &mut out) {
        eprintln!("Can't write the output: {e}");
        process::exit(IO_ERR);
    }
}
