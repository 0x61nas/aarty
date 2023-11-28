#[cfg(feature = "colors")]
pub mod color;

use image::{GenericImageView, Pixel, Rgba};

#[cfg(feature = "colors")]
use color::{Color, ANSI_BACKGROUND_ESCAPE, ANSI_ESCAPE_CLOSE, ANSI_FOREGROUND_ESCAPE};

use std::fmt::{self, Display};

/// Represent the ASCII art.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TextImage {
    #[cfg(feature = "colors")]
    bc: Option<Color>,
    fragments: Vec<Fragment>,
    /// The columans number.
    pub row_len: usize,
    /// Don't use colors.
    #[cfg(feature = "colors")]
    pub no_colors: bool,
    /// Reverse the forgruond color with the background.
    #[cfg(feature = "reverse")]
    pub reverse: bool,
}

impl TextImage {
    /// Set the background color.
    ///
    /// # Examples
    /// ```
    /// # use aarty::*;
    /// let ascii = image::open("images/ok_hand.png").unwrap()
    ///         .to_text(" .,-~!;:=*&%$@#".chars().collect())
    ///        .with_background((255, 255, 255));
    ///
    ///```
    #[inline(always)]
    pub fn with_background<C: Into<Color>>(mut self, bc: C) -> Self {
        self.bc = Some(bc.into());
        self
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.fragments.len()
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline(always)]
    fn _background(&self, f: &mut fmt::Formatter<'_>) -> std::result::Result<bool, fmt::Error> {
        if let Some(bc) = &self.bc {
            write!(f, "{ANSI_BACKGROUND_ESCAPE}{bc}m")?;
            return Ok(true);
        }
        Ok(false)
    }

    #[cfg(feature = "colors")]
    #[inline]
    fn _color_fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let has_background = {
            #[cfg(feature = "reverse")]
            if self.reverse {
                let mut r = false;
                if let Some(bc) = &self.bc {
                    write!(f, "{ANSI_FOREGROUND_ESCAPE}{bc}m")?;
                    r = true;
                }
                r
            } else {
                self._background(f)?
            }
            #[cfg(not(feature = "reverse"))]
            self._background(f)?
        };

        let mut i = 0;
        for frag in &self.fragments {
            i += 1;
            if i == self.row_len {
                i = 0;
                writeln!(f)?;
            }

            #[cfg(feature = "reverse")]
            if self.reverse {
                write!(f, "{ANSI_BACKGROUND_ESCAPE}{bc}m", bc = frag.fg)?;
            } else {
                write!(f, "{ANSI_FOREGROUND_ESCAPE}{fg}m", fg = frag.fg)?;
            }
            #[cfg(not(feature = "reverse"))]
            write!(f, "{ANSI_FOREGROUND_ESCAPE}{fg}m", fg = frag.fg)?;

            write!(f, "{ch}{ANSI_ESCAPE_CLOSE}", ch = frag.ch)?;
        }

        if has_background {
            f.write_str(ANSI_ESCAPE_CLOSE)?;
        }

        Ok(())
    }

    #[inline(always)]
    fn _fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut i = 0;
        for frag in &self.fragments {
            i += 1;
            if i == self.row_len {
                i = 0;
                writeln!(f)?;
            }
            f.write_str(&frag.ch.to_string())?;
        }
        Ok(())
    }
}

impl Display for TextImage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        #[cfg(feature = "colors")]
        {
            if self.no_colors {
                self._fmt(f)
            } else {
                self._color_fmt(f)
            }
        }

        #[cfg(not(feature = "colors"))]
        self._fmt(f)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
struct Fragment {
    ch: char,
    #[cfg(feature = "colors")]
    fg: Color,
}

impl Fragment {
    #[cfg(not(feature = "colors"))]
    #[inline(always)]
    fn new(ch: char) -> Fragment {
        Fragment { ch }
    }

    #[cfg(feature = "colors")]
    #[inline(always)]
    fn new(ch: char, fc: Color) -> Fragment {
        Fragment { ch, fg: fc }
    }
}

/// Trait to convert an imgae to ASCII art.
pub trait ToTextImage {
    fn to_text(&self, set: &[char]) -> TextImage;
}

impl<T, P> ToTextImage for T
where
    T: GenericImageView<Pixel = P>,
    P: Into<Color> + Pixel<Subpixel = u8>,
{
    #[inline(always)]
    fn to_text(&self, set: &[char]) -> TextImage {
        crate::convert_image_to_ascii(self, set)
    }
}

/// Covert the image into ASCII art.
///
/// **Note** You might want to resize the image before use this function if you want to fit the result
/// on one page.
///
/// #Arguments
/// - image: The image to convert.
/// - set: the ASCII sympols to draw the image with (from lighter to darker)
pub fn convert_image_to_ascii<I, P>(image: &I, set: &[char]) -> TextImage
where
    I: GenericImageView<Pixel = P>,
    P: Into<Color> + Pixel<Subpixel = u8>,
{
    let (width, height) = image.dimensions();
    // TODO: make sure that the _capacity_ are correct
    let mut fragments = Vec::with_capacity((width * height) as usize);
    for y in 0..height {
        for x in 0..width {
            let pixel = Pixel::to_rgba(&image.get_pixel(x, y));
            #[cfg(not(feature = "colors"))]
            fragments.push(Fragment::new(get_character(pixel, set)));

            #[cfg(feature = "colors")]
            fragments.push(Fragment::new(get_character(pixel, set), pixel.into()));
        }
    }

    TextImage {
        fragments,
        #[cfg(feature = "colors")]
        bc: None,
        row_len: width as usize,
        no_colors: false,
        #[cfg(feature = "reverse")]
        reverse: false,
    }
}

#[inline(always)]
fn get_character(pixel: Rgba<u8>, characters: &[char]) -> char {
    // TODO: handle the zeros case
    let intent = if pixel[3] == 0 {
        0
    } else {
        pixel[0] / 3 + pixel[1] / 3 + pixel[2] / 3
    };

    characters[(intent / (32 + 7 - (7 + (characters.len() - 7)) as u8)) as usize]
}
