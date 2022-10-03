use image::{GenericImageView, DynamicImage};
use colored::{ColoredString, Colorize};
use crate::args::{
    args::Arguments,
    enums::Mode,
};

pub fn generate_ascii(image: DynamicImage, args: &Arguments) -> Vec<ColoredString> {
    let characters = args.characters.chars().collect::<Vec<char>>();
    trace!("Characters: {:?}, length: {}", characters, characters.len());
    let mut output = Vec::new();
    let (width, height) = image.dimensions();

    for y in 0..height {
        for x in 0..width {
            if y % (args.scale * 2) == 0 && x % args.scale == 0 {
                output.push(get_character(
                    image.get_pixel(x, y),
                    &characters, args.mode,
                    &args.background,
                ));
            }
        }
        // Add a new line at the end of each row
        if y % (args.scale * 2) == 0 {
            output.push("\n".into());
        }
    }

    output
}

fn get_character(
    pixel: image::Rgba<u8>,
    characters: &Vec<char>, mode: Mode,
    background: &Option<String>,
) -> ColoredString {
    let intent = if pixel[3] == 0 { 0 } else { pixel[0] / 3 + pixel[1] / 3 + pixel[2] / 3 };

    let ch = characters[(intent / (32 + 7 - (7 + (characters.len() - 7)) as u8)) as usize];

    let ch = String::from(ch);

    let ch = match mode {
        Mode::NormalAscii => ColoredString::from(&*ch),
        Mode::COLORED => {
            ch.to_string()
                .truecolor(pixel[0], pixel[1], pixel[2])
        }
    };

    match background {
        Some(bg) => ch.on_color(bg.to_string()),
        None => ch
    }
}
