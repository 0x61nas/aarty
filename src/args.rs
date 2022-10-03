pub mod args {
    use clap::{Parser, arg, ValueEnum, ColorChoice};

    #[derive(Parser)]
    #[command(author, version, about, long_about=None, color=ColorChoice::Always)]
    pub struct Arguments {
        /// The art mode to use
        #[arg(short, long, default_value="normal-ascii")]
        pub mode: Mode,
        #[arg(long, default_value="stdout", alias="mo")]
        pub output_method: OutputMethod,
        /// The image to convert to ASCII art
        pub image: String,
        /// The output file to write to (if output_method is file)
        #[arg(short, long, default_value=Some("ascii_image.txt"))]
        pub output: Option<String>,
    }

    #[derive(Copy, Clone, ValueEnum, Debug, PartialOrd, Eq, PartialEq)]
    pub enum Mode {
        /// Normal ASCII art
        #[clap(alias = "n")]
        NormalAscii,
        /// Colored ASCII art, the colors are based on the terminal colors
        #[clap(alias = "c")]
        COLORED,
    }

    #[derive(Copy, Clone, ValueEnum, Debug, PartialOrd, Eq, PartialEq)]
    pub enum OutputMethod {
        /// Save the ascii art to a file
        #[clap(alias = "f")]
        File,
        #[clap(alias = "c")]
        /// Copy the ascii art to the clipboard
        Clipboard,
        /// Print the ascii art to the terminal
        #[clap(alias = "s")]
        Stdout,
    }
}
