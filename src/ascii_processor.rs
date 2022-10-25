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
    let actual_scale = calculate_scale(args, (width, height));

    for y in 0..height {
        for x in 0..width {
            if y % (actual_scale * 2) == 0 && x % actual_scale == 0 {
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
        if y % (actual_scale * 2) == 0 {
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

///
/// Determine which scale to use in presence of `width` parameters,
/// otherwise uses regular `scale` parameter as default
///
fn calculate_scale(args: &Arguments, dimensions: (u32, u32)) -> u32 {
    args.width.map_or_else(|| args.scale, |v| dimensions.0 / v)
}

#[cfg(test)]
mod test {
    use crate::args::{
        args::Arguments,
        enums::{Mode, OutputMethod},
    };

    use super::calculate_scale;

    const DIMENSIONS: (u32, u32) = (100, 100);

    #[test]
    fn test_scale() {
        let args = Arguments {
            mode: Mode::NormalAscii,
            output_method: OutputMethod::Stdout,
            image: "".into(),
            characters: "".into(),
            scale: 4,
            width: Some(10),
            background: None,
            output: "".into(),
        };

        let scale = calculate_scale(&args, DIMENSIONS);
        assert_eq!(scale, 10);
    }

    #[test]
    fn test_default_scale() {
        let args = Arguments {
            mode: Mode::NormalAscii,
            output_method: OutputMethod::Stdout,
            image: "".into(),
            characters: "".into(),
            scale: 4,
            width: None,
            background: None,
            output: "".into(),
        };

        let scale = calculate_scale(&args, DIMENSIONS);
        assert_eq!(scale, 4);
    }
}
