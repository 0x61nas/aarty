use crate::args::{args::Arguments, enums::Mode};
use colored::{ColoredString, Colorize};
use image::{DynamicImage, GenericImageView};

use std::io::{self, BufWriter, Write};

pub fn generate_ascii<W: Write>(
    image: DynamicImage,
    args: &Arguments,
    mut buffer: BufWriter<W>,
) -> io::Result<()> {
    let characters = args.characters.chars().collect::<Vec<char>>();
    let (width, height) = image.dimensions();

    for y in 0..height {
        for x in 0..width {
            if y % (args.scale * 2) == 0 && x % args.scale == 0 {
                let element = get_character(
                    image.get_pixel(x, y),
                    &characters,
                    args.mode,
                    &args.background,
                );

                buffer.write_all(format!("{element}").as_bytes())?;
            }
        }
        // Add a new line at the end of each row
        if y % (args.scale * 2) == 0 {
            buffer.write_all("\n".as_bytes())?;
        }
    }

    Ok(())
}

fn get_character(
    pixel: image::Rgba<u8>,
    characters: &Vec<char>,
    mode: Mode,
    background: &Option<String>,
) -> ColoredString {
    let intent = if pixel[3] == 0 {
        0
    } else {
        pixel[0] / 3 + pixel[1] / 3 + pixel[2] / 3
    };

    let ch = characters[(intent / (32 + 7 - (7 + (characters.len() - 7)) as u8)) as usize];

    let ch = String::from(ch);

    let ch = match mode {
        Mode::NormalAscii => ColoredString::from(&*ch),
        Mode::Colored => ch.truecolor(pixel[0], pixel[1], pixel[2]),
    };

    match background {
        Some(bg) => ch.on_color(bg.to_string()),
        None => ch,
    }
}
