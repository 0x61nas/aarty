pub mod args {
    use super::enums::*;
    use clap::{arg, ColorChoice, Parser};

    #[derive(Parser, Debug)]
    #[command(author, version, about, long_about = None, color = ColorChoice::Always)]
    pub struct Arguments {
        /// The art mode to use
        #[arg(short, long, default_value = "normal-ascii")]
        pub mode: Mode,
        #[arg(long, default_value = "stdout", alias = "mo")]
        pub output_method: OutputMethod,
        /// The image to convert to ASCII art
        pub image: String,
        /// The character to use for drawing the image (lighter to darker)
        /// You can user one character if you uses the color mode
        #[arg(short, long, default_value = " .,-~!;:=*&%$@#")]
        pub characters: String,
        /// The output scale (1 is the original size)
        #[arg(short, long, default_value = "4")]
        pub scale: u32,
        // Enstablish how much wide is the output images, in columns. Overrides `scale`
        #[arg(short, long, default_value= None )]
        pub width: Option<u32>,
        /// The background color to use
        #[arg(short, long, default_value = None)]
        pub background: Option<String>,
        /// The output file to write to (if output_method is file)
        #[arg(short, long, default_value = "ascii_image.txt")]
        pub output: String,
    }

    impl Arguments {
        pub fn validate(&self) -> Result<(), String> {
            if self.characters.is_empty() {
                return Err("No characters provided".to_string());
            } else if self.characters.len() == 1 {
                if self.mode == Mode::NormalAscii {
                    return Err("One character provided but mode is normal-ascii".to_string());
                }
            } else if self.characters.len() > 32 {
                return Err("Too many characters provided, max is 32".to_string());
            }
            Ok(())
        }
    }
}

pub mod enums {
    use clap::ValueEnum;

    #[derive(Copy, Clone, ValueEnum, Debug, PartialOrd, Eq, PartialEq)]
    pub enum Mode {
        /// Normal ASCII art
        #[clap(alias = "n")]
        NormalAscii,
        /// Colored ASCII art, the colors are based on the terminal colors
        #[clap(alias = "c")]
        Colored,
    }

    #[derive(Copy, Clone, ValueEnum, Debug, PartialOrd, Eq, PartialEq)]
    pub enum OutputMethod {
        /// Save the ascii art to a file
        #[clap(alias = "f")]
        File,
        /// Print the ascii art to the terminal
        #[clap(alias = "s")]
        Stdout,
    }
}
