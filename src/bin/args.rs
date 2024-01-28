use std::{env, num::NonZeroU8, process};

use aarty::{COLORS, REVERSE};
use image::imageops::FilterType;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Opts {
    /// The image to convert to ASCII art
    pub path: Option<String>,
    /// The character to use for drawing the image (lighter to darker)
    /// You can user one character if you uses the color mode
    pub sym_set: Vec<char>,
    /// The output scale (1 is the original size)
    pub scale: NonZeroU8,
    /// Enstablish how much wide is the output images, in columns. Overrides `scale`
    pub width: Option<u32>,
    pub height: Option<u32>,
    /// The background color to use
    pub background: Option<String>,
    pub flags: u8,
    pub sf: FilterType,
}

impl Opts {
    pub fn from_args() -> Result<Opts, String> {
        let mut args = env::args().skip(1);
        let mut opts = Opts::default();

        macro_rules! err {
            ($arg:ident) => {
                format!("Expected one argument after {}, found 0.", $arg)
            };
            (parse; $item:expr, $error:ident) => {
                format!("Can't parse the provided {}, because `{}`", $item, $error)
            };
        }

        macro_rules! value {
            ($arg:ident) => {
                args.next().ok_or(err!($arg))
            };
            (parse; $name:expr, $arg:ident) => {
                args
                    .next()
                    .ok_or(err!($arg))?
                    .parse()
                    .map_err(|e| err!(parse; $name, e))
            }
        }

        while let Some(arg) = args.next() {
            if !arg.starts_with('-') {
                opts.path = Some(arg);
                continue;
            }
            let arg = arg.to_lowercase();
            let arg = arg.trim_start_matches('-');
            match arg {
                "c" | "sympols" | "chars" => opts.sym_set = value!(arg)?.chars().collect(),
                "s" | "scale" => {
                    opts.scale = NonZeroU8::new(value!(parse; "scale", arg)?)
                        .ok_or_else(|| "The scale should be greaer than 0,".to_string())?
                }
                "w" | "col" | "columans" | "width" => {
                    opts.width = Some(value!(parse; "width", arg)?)
                }
                "h" | "row" | "rows" | "height" => {
                    opts.height = Some(value!(parse; "height", arg)?)
                }
                "b" | "back" | "background" => opts.background = Some(value!(arg)?),
                "r" | "reverse" => opts.flags |= REVERSE,
                "u" | "color" | "colors" => opts.flags |= COLORS,
                "sft" | "st" => opts.sf = FilterType::Triangle,
                "sfc" | "sc" => opts.sf = FilterType::CatmullRom,
                "sfg" | "sg" => opts.sf = FilterType::Gaussian,
                "sfl" | "sl" => opts.sf = FilterType::Lanczos3,
                "sfn" | "sn" => opts.sf = FilterType::Nearest,
                "v" | "version" => info(format!("aarty v{VERSION}")),
                unknown => return Err(format!("Unknown opthion {unknown}")),
            }
        }

        Ok(opts)
    }
}

impl Default for Opts {
    fn default() -> Self {
        Opts {
            path: None,
            sym_set: vec![
                ' ', '.', ',', '-', '~', '!', ';', ':', '=', '*', '&', '%', '$', '@', '#',
            ],
            scale: unsafe { NonZeroU8::new_unchecked(4) },
            width: None,
            height: None,
            background: None,
            flags: 0,
            sf: FilterType::Nearest,
        }
    }
}

#[cold]
fn info(msg: String) -> ! {
    println!("{msg}");
    process::exit(0)
}
