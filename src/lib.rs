#[cfg(feature = "colors")]
pub mod color;
#[cfg(feature = "image")]
pub mod impl_image;
pub mod sympols;
#[cfg(feature = "text_image")]
pub mod text_image;

#[cfg(feature = "colors")]
use color::ANSIColor;
#[cfg(feature = "colors")]
use color::{ANSI_BACKGROUND_ESCAPE, ANSI_ESCAPE_CLOSE, ANSI_FOREGROUND_ESCAPE};
use sympols::Sympols;

use std::{error::Error, io::Write, mem};

/// Use colos.
pub const COLORS: u8 = 0b1;
/// Reverse the forgruond color with the background.
pub const REVERSE: u8 = 0b10;

pub trait PixelImage {
    fn dimensions(&self) -> (u32, u32);
    fn get_pixel(&self, x: u32, y: u32) -> Rgba;
}

pub trait FragmentWriter {
    #[cfg(feature = "colors")]
    fn background(&mut self, bc: &ANSIColor) -> Result<bool, Box<dyn Error>>;
    fn write_fragment(&mut self, info: FragmentInfo) -> Result<(), Box<dyn Error>>;
    #[cfg(feature = "colors")]
    fn write_colored_fragment(
        &mut self,
        info: FragmentInfo,
        bc: Option<&ANSIColor>,
        fc: Option<&ANSIColor>,
    ) -> Result<(), Box<dyn Error>>;
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

#[derive(Debug, PartialEq, PartialOrd, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Config {
    pub sympols: Sympols,
    #[cfg(feature = "colors")]
    pub background: Option<ANSIColor>,
    /// The columans number.
    pub flags: u8,
}

impl Config {
    pub const fn reversed(&self) -> bool {
        self.flags & REVERSE == REVERSE
    }

    pub const fn use_colors(&self) -> bool {
        self.flags & COLORS == COLORS
    }

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

#[derive(Debug, PartialEq, PartialOrd, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FragmentInfo {
    pub sym: char,
    pub sym_index: usize,
    #[cfg(feature = "colors")]
    pub fg: ANSIColor,
}

#[cfg(feature = "colors")]
#[derive(Debug, PartialEq, PartialOrd, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Rgba {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

/// Covert the image into ASCII art.
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
