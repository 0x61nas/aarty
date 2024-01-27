//! # **aarty**
//! mini freamwork to render images in the terminals/ttys.
//!
//! [![crates.io](https://img.shields.io/crates/v/aarty.svg)](https://crates.io/crates/aarty)
//! [![docs.rs](https://docs.rs/aarty/badge.svg)](https://docs.rs/aarty)
//! [![downloads](https://img.shields.io/crates/d/aarty.svg)](https://crates.io/crates/aarty)
//! [![license](https://img.shields.io/crates/l/aarty.svg)](https://github.com/0x61nas/aarty/blob/aurora/LICENSE)
//!
//! # Examples
//! ```no_run
//! # use aarty::*;
//! # use std::io::{self, BufWriter};
//! let cfg = Config {
//!     sympols: vec![' ', '.', ',', '-', '~', '!', '*', '%', '$', '@', '#'].into(),
//!     background: None,
//!     flags: 0,
//! };
//! let image = image::open("mylove.jpg").unwrap();
//! let (w, h) = image.dimensions();
//!
//! let mut out = BufWriter::with_capacity(cfg.calc_buf_size(w, h), io::stdout().lock());
//!
//! convert_image_to_ascii(&cfg, &image, &mut out).expect("IO error");
//! ```
//! Enable the foreground colors
//! ```no_run
//! # use aarty::*;
//! let cfg = Config {
//!     sympols: vec![' ', '.', ',', '-', '~', '!', '*', '%', '$', '@', '#'].into(),
//!     background: None,
//!     flags: COLORS,
//! };
//! // ...
//! ```
//! Reverse them with the background color
//! ```no_run
//! # use aarty::*;
//! let cfg = Config {
//!     sympols: Sympols::empty(),
//!     background: Some((232, 209, 204).into()),
//!     flags: COLORS | REVERSE,
//! };
//! // ...
//! ```
//! If you wanna build a rebresentesion in memory so you can modify it or use it multiple times, then you may found that implement [`FragmentWriter`]
//! for such a structher is useful.
//! ```no_run
//! # use aarty::*;
//! struct TerminalFrame {
//!     fragments: Vec<(char, ANSIColor)>,
//!    cfg: Config,
//! }
//!
//! impl FragmentWriter for TerminalFrame {
//!     fn background(&mut self, _: &ANSIColor) -> Result<bool, Box<dyn std::error::Error>> {
//!         // Nah, I don't care, I have my configs :p
//!         //  but pretent like if you care so it will skip the swap operation.
//!         Ok(true)
//!     }
//!
//!     fn write_fragment(&mut self, info: FragmentInfo) -> Result<(), Box<dyn std::error::Error>> {
//!         self.fragments.push((info.sym, info.fg));
//!         Ok(())
//!     }
//!
//!     fn write_colored_fragment(
//!         &mut self,
//!         info: FragmentInfo,
//!         _: Option<&ANSIColor>,
//!         _: Option<&ANSIColor>,
//!     ) -> Result<(), Box<dyn std::error::Error>> {
//!         self.write_fragment(info)
//!     }
//!
//!     fn write_bytes(&mut self, _bytes: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
//!         // Just ignore them
//!         Ok(())
//!     }
//! }
//!
//! // So you can use it as a buffer
//! let cfg = Config {
//!     sympols: vec!['.', ',', '0', '1', '2', '3', '4', '5', '6', '8'].into(),
//!     background: None,
//!     flags: 0,
//! };
//! let image = image::open("mylove.jpg").unwrap();
//! let (w, h) = image.dimensions();
//! let mut frame = TerminalFrame {
//!     fragments: Vec::with_capacity(w * h),
//!     cfg: cfg.clone(),
//! };
//! aarty::convert_image_to_ascii(&cfg, &image, &mut frame).expect("Write error");
//! ... Do whatever you want with this object
//! ```
//! But be aware if you take this way, you'll have to implement the rendaring mechanism when its its the time to print the image (a.k.a. rendering it).
//!
//! For such this case, we have [`TextImage`], which basically dose the same thing as the code above but in more ergnomic way, And it does implement the rendering mechanism, so you can just print it, and it will render the image properly.
//! You can enable this type with `text_image` feature, which is enabled by default.
//!
//! The `text_image` feature also include the [`ToTextImage`] trait, which provide an ergonomic way to construct an [`TextImage`] object.
//! ```no_run
//! # use aarty::*;
//! use aarty::ToTextImage;
//! let cfg = Config {
//!     sympols: Sympols::empty(),
//!     background: Some((232, 209, 204).into()),
//!     flags: COLORS | REVERSE,
//! };
//! let image = image::open("mylove.jpg").unwrap().to_text(cfg);
//! println!("{image}");
//!```
//! > You have to enable the `image` feature for this to work.
//!
//! # The binary
//! We offer a simple binary that's implement the most of this crate features. You can build it with the build command or if u use cargo then you can install it via `cargo install aarty`.
//!
//! > **Note**
//! > for more information about the binary and how to use it, you can run `aarty --help` or see this [document](./docs/bin.md).
//!
//!
//! # Contributing
//! I'm happy to accept any contributions, just consider reading the [CONTRIBUTING.md](https://github.com/0x61nas/aarty/blob/aurora/CONTRIBUTING.md) guide first.
//!
//! > the main keywords are: **signed commits**, **conventional commits**, **no emojis**, **linear history**, **the PR shouldn't have more than tree commits most of the time**
//!
//! # License
//! This project is licensed under [MIT license][mit].
//!
//! [mit]: https://github.com/0x61nas/aarty/blob/aurora/LICENSE
//!

/// ANSI color.
#[cfg(feature = "colors")]
pub mod color;
/// The traits implimaantions for the [`image`] crate intigration.
#[cfg(feature = "image")]
pub mod impl_image;
/// The [`Sympols`] struct.
pub mod sympols;
/// The [`ToTextImage`] and [`TextImage`] stuff.
#[cfg(feature = "text_image")]
pub mod text_image;
// Re-exports
#[cfg(feature = "colors")]
pub use color::ANSIColor;
pub use sympols::Sympols;
#[cfg(feature = "text_image")]
pub use text_image::{Fragment, IndexdFragment, TextImage, ToTextImage};

#[cfg(feature = "colors")]
use color::{ANSI_BACKGROUND_ESCAPE, ANSI_ESCAPE_CLOSE, ANSI_FOREGROUND_ESCAPE};
use std::{error::Error, io::Write, mem};

/// Use colors flag.
pub const COLORS: u8 = 0b1;
/// Reverse the forgruond color with the background.
pub const REVERSE: u8 = 0b10;

/// Trait that represent the (normal) images, that we wanna transform them.
pub trait PixelImage {
    /// Get the image dimensions (the width, and height).
    fn dimensions(&self) -> (u32, u32);
    /// Get the RGBA value of a specific pixel.
    fn get_pixel(&self, x: u32, y: u32) -> Rgba;
}

/// A trait for objects that can used as a buffer (out) with [`convert_image_to_ascii`]
pub trait FragmentWriter {
    /// Rseves the background of the image and return a boolean that iindcates if they which from the cally to call [`FragmentWriter::write_bytes`] and send the `ANSI CLOSE` escape code or no.
    #[cfg(feature = "colors")]
    fn background(&mut self, bc: &ANSIColor) -> Result<bool, Box<dyn Error>>;

    /// Write a fragment to te buffer.
    fn write_fragment(&mut self, info: FragmentInfo) -> Result<(), Box<dyn Error>>;

    /// Write a fragment that may have a different background/foreground.
    #[cfg(feature = "colors")]
    fn write_colored_fragment(
        &mut self,
        info: FragmentInfo,
        bc: Option<&ANSIColor>,
        fc: Option<&ANSIColor>,
    ) -> Result<(), Box<dyn Error>>;

    /// Write raw bytes to the buffer, tipclly used for the `ANSI CcLOSE` escapecode when the image is  finished, or to write `\n` after each row.
    fn write_bytes(&mut self, bytes: &[u8]) -> Result<(), Box<dyn Error>>;
}

impl<W: Write> FragmentWriter for W {
    #[inline]
    fn background(&mut self, bc: &ANSIColor) -> Result<bool, Box<dyn Error>> {
        self.write_bytes(bc.to_string().as_bytes())?;
        Ok(true)
    }

    #[inline]
    fn write_bytes(&mut self, bytes: &[u8]) -> Result<(), Box<dyn Error>> {
        self.write_all(bytes)?;
        Ok(())
    }

    #[inline]
    fn write_fragment(&mut self, info: FragmentInfo) -> Result<(), Box<dyn Error>> {
        self.write_all(info.sym.to_string().as_bytes())?;
        Ok(())
    }

    #[cfg(feature = "colors")]
    #[inline]
    fn write_colored_fragment(
        &mut self,
        info: FragmentInfo,
        bc: Option<&ANSIColor>,
        fc: Option<&ANSIColor>,
    ) -> Result<(), Box<dyn Error>> {
        if let Some(bc) = bc {
            self.write_all(bc.as_background().as_bytes())?;
        }
        if let Some(fc) = fc {
            self.write_all(fc.as_foreground().as_bytes())?;
        }

        self.write_fmt(format_args!("{}", info.sym))?;

        if bc.is_some() {
            self.write_all(ANSI_ESCAPE_CLOSE.as_bytes())?;
        }
        if fc.is_some() {
            self.write_all(ANSI_ESCAPE_CLOSE.as_bytes())?;
        }
        Ok(())
    }
}

/// the main config structure
#[derive(Debug, PartialEq, PartialOrd, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Config {
    /// The sympols (characters) set.
    pub sympols: Sympols,
    /// The result image background, useful if the image has a transparent parts or if you plan to use the [`REVERSE`] flag.
    #[cfg(feature = "colors")]
    pub background: Option<ANSIColor>,
    /// the boolean flags.
    pub flags: u8,
}

impl Config {
    /// return true if the [`REVERSE`] flag is set.
    pub const fn reversed(&self) -> bool {
        self.flags & REVERSE == REVERSE
    }

    /// return true if the [`COLORS`] flag is set.
    pub const fn use_colors(&self) -> bool {
        self.flags & COLORS == COLORS
    }

    /// Calculate how much space the raw representation would use (the worst case).
    pub const fn calc_buf_size(&self, w: u32, h: u32) -> usize {
        let mut res = w as usize * h as usize;
        if self.use_colors() {
            //XXX: cheack from this.
            res = (res
                * (ANSI_ESCAPE_CLOSE.len()
                    + ANSI_FOREGROUND_ESCAPE.len()
                    + ANSI_BACKGROUND_ESCAPE.len()))
                * (3 * 3);
        }
        res
    }
}

/// the fragment (a.k.a. pixel) information.
#[derive(Debug, PartialEq, PartialOrd, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FragmentInfo {
    /// the character that had chosen from the sympols set to represent this pixel.
    pub sym: char,
    /// The sympol index in the sympols set (might be useful if you want to store this info in memory
    /// and happen that you know that the symplos set size arn't gonna more then [`u8::MAX`])
    pub sym_index: usize,
    /// The pixel color in ANSI representation.
    #[cfg(feature = "colors")]
    pub fg: ANSIColor,
}

/// RGBA pixel.
#[derive(Debug, PartialEq, PartialOrd, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Rgba {
    /// Red.
    r: u8,
    /// Green.
    g: u8,
    /// Blue.
    b: u8,
    /// Alpha.
    a: u8,
}

/// Convert the image into ASCII art based on the [`Config`] and write it to the [`FragmentWriter`].
///
/// **Note** You might want to resize the image before use this function if you want to fit the result
/// on one page.
///
pub fn convert_image_to_ascii<I, W>(
    config: &Config,
    image: &I,
    out: &mut W,
) -> Result<(), Box<dyn Error>>
where
    I: PixelImage,
    W: FragmentWriter,
{
    let (width, height) = image.dimensions();
    let ansi_close = if let Some(bc) = &config.background {
        if !config.reversed() {
            out.background(bc)?
        } else {
            false
        }
    } else {
        false
    };

    let colored = cfg!(feature = "colors") && config.use_colors();

    for y in 0..height {
        for x in 0..width {
            let pixel = image.get_pixel(x, y);
            if colored {
                #[cfg(feature = "colors")]
                {
                    let (sym, sym_index) = config.sympols.sym_and_index(&pixel);
                    let fi = FragmentInfo {
                        sym,
                        sym_index,
                        fg: ANSIColor::from(pixel),
                    };
                    let mut fg = Some(fi.fg.clone());
                    let mut bc = config.background.clone();
                    if !ansi_close && config.reversed() {
                        mem::swap(&mut bc, &mut fg);
                    }
                    out.write_colored_fragment(fi, bc.as_ref(), fg.as_ref())?;
                }
            } else {
                let (sym, sym_index) = config.sympols.sym_and_index(&pixel);
                out.write_fragment(FragmentInfo {
                    sym,
                    sym_index,
                    #[cfg(feature = "colors")]
                    fg: pixel.into(),
                })?;
            }
        }
        out.write_bytes("\n".as_bytes())?;
    }

    if ansi_close {
        out.write_bytes(ANSI_ESCAPE_CLOSE.as_bytes())?;
    }

    Ok(())
}
