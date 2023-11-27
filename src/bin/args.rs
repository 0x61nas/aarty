use std::env;

pub struct Opts {
    /// The image to convert to ASCII art
    pub path: String,
    /// The character to use for drawing the image (lighter to darker)
    /// You can user one character if you uses the color mode
    pub sym_set: Vec<char>,
    /// The output scale (1 is the original size)
    pub scale: u32,
    /// Enstablish how much wide is the output images, in columns. Overrides `scale`
    pub width: Option<u32>,
    pub height: Option<u32>,
    /// The background color to use
    pub background: Option<String>,
    pub reverse: bool,
    pub colors: bool,
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
                opts.path = arg.clone();
                continue;
            }
            let arg = arg.trim_start_matches('-');
            match arg {
                "c" | "sympols" | "chars" => opts.sym_set = value!(arg)?.chars().collect(),
                "s" | "scale" => opts.scale = value!(parse; "scale", arg)?,
                "w" | "col" | "columans" | "width" => {
                    opts.width = Some(value!(parse; "width", arg)?)
                }
                "h" | "row" | "rows" | "height" => {
                    opts.height = Some(value!(parse; "height", arg)?)
                }
                "b" | "back" | "background" => opts.background = Some(value!(arg)?),
                "r" | "reverse" => opts.reverse = true,
                "u" | "color" | "colors" => opts.colors = true,
                unknown => return Err(format!("Unknown opthion {unknown}")),
            }
        }

        if opts.path.is_empty() {
            return Err("You must provide the image path/name".to_string());
        }
        Ok(opts)
    }
}

impl Default for Opts {
    fn default() -> Self {
        Opts {
            path: String::with_capacity(0),
            sym_set: vec![
                ' ', '.', ',', '-', '~', '!', ';', ':', '=', '*', '&', '%', '$', '@', '#',
            ],
            scale: 4,
            width: None,
            height: None,
            background: None,
            reverse: false,
            colors: false,
        }
    }
}