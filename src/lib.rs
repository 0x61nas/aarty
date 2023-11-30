#[cfg(feature = "colors")]
pub mod color;
#[cfg(feature = "image")]
pub mod impl_image;

#[cfg(feature = "colors")]
use color::ANSIColor;

use std::fmt::{self, Display};

/// Use colos.
pub const COLORS: u8 = 0b1;
/// Reverse the forgruond color with the background.
pub const REVERSE: u8 = 0b10;

/// Represent the ASCII art.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TextImage<'a> {
    pub set: &'a Vec<char>,
    #[cfg(feature = "colors")]
    bc: Option<ANSIColor>,
    fragments: Vec<Fragment>,
    /// The columans number.
    pub row_len: usize,
    pub flags: u8,
}

impl TextImage<'_> {
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
    pub fn with_background<C: Into<ANSIColor>>(mut self, bc: C) -> Self {
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
    pub fn reverse(&self) -> bool {
        self.flags & REVERSE == REVERSE
    }

    #[inline(always)]
    pub fn colors(&self) -> bool {
        self.flags & COLORS == COLORS
    }

    #[inline(always)]
    fn _background(&self, f: &mut fmt::Formatter<'_>) -> std::result::Result<bool, fmt::Error> {
        if let Some(bc) = &self.bc {
            write!(f, "{bc:-}")?;
            return Ok(true);
        }
        Ok(false)
    }

    #[cfg(feature = "colors")]
    #[inline]
    fn _color_fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use color::ANSI_ESCAPE_CLOSE;

        let has_background = {
            #[cfg(feature = "reverse")]
            if self.reverse() {
                let mut r = false;
                if let Some(bc) = &self.bc {
                    if !bc.is_transparent() {
                        write!(f, "{bc}")?;
                        r = true;
                    }
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
            if self.reverse() {
                write!(f, "{c:-}", c = frag.fg)
            } else {
                write!(f, "{c}", c = frag.fg)
            }?;
            #[cfg(not(feature = "reverse"))]
            write!(f, "{c}", c = frag.fg)?;

            write!(
                f,
                "{ch}{ANSI_ESCAPE_CLOSE}",
                ch = self.set[frag.ch_index as usize]
            )?;
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
            f.write_str(&self.set[frag.ch_index as usize].to_string())?;
        }
        Ok(())
    }
}

impl Display for TextImage<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        #[cfg(feature = "colors")]
        {
            if self.colors() {
                self._color_fmt(f)
            } else {
                self._fmt(f)
            }
        }

        #[cfg(not(feature = "colors"))]
        self._fmt(f)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
struct Fragment {
    ch_index: u8,
    #[cfg(feature = "colors")]
    fg: ANSIColor,
}

impl Fragment {
    #[cfg(not(feature = "colors"))]
    #[inline(always)]
    fn new(ch_index: u8) -> Fragment {
        Fragment { ch_index }
    }

    #[cfg(feature = "colors")]
    #[inline(always)]
    fn new(ch_index: u8, fc: ANSIColor) -> Fragment {
        Fragment { ch_index, fg: fc }
    }
}

/// Trait to convert an imgae to ASCII art.
pub trait ToTextImage {
    fn to_text<'a>(&self, set: &'a Vec<char>) -> TextImage<'a>;
}

impl<T> ToTextImage for T
where
    T: PixelImage,
{
    #[inline(always)]
    fn to_text<'a>(&self, set: &'a Vec<char>) -> TextImage<'a> {
        crate::convert_image_to_ascii(self, set)
    }
}

pub trait PixelImage {
    fn dimensions(&self) -> (u32, u32);
    #[cfg(feature = "colors")]
    fn get_pixel(&self, x: u32, y: u32) -> Rgba;
}

#[cfg(feature = "colors")]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
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
/// #Arguments
/// - image: The image to convert.
/// - set: the ASCII sympols to draw the image with (from lighter to darker)
pub fn convert_image_to_ascii<'a, I>(image: &I, set: &'a Vec<char>) -> TextImage<'a>
where
    I: PixelImage,
{
    let (width, height) = image.dimensions();
    let frag_cap = (width * height) as usize;
    let mut fragments = Vec::with_capacity(frag_cap);
    for y in 0..height {
        for x in 0..width {
            let pixel = image.get_pixel(x, y);
            #[cfg(not(feature = "colors"))]
            fragments.push(Fragment::new(get_character(pixel, set)));

            #[cfg(feature = "colors")]
            fragments.push(Fragment::new(
                get_character(pixel.clone(), set),
                pixel.into(),
            ));
        }
    }
    // make sure that the `fragments` vec didn't grow up (debug only)
    debug_assert_eq!(fragments.capacity(), frag_cap);

    TextImage {
        set,
        fragments,
        #[cfg(feature = "colors")]
        bc: None,
        row_len: width as usize,
        flags: 0,
    }
}

#[inline(always)]
fn get_character(pixel: Rgba, characters: &[char]) -> u8 {
    if characters.is_empty() {
        panic!("The set can't be empty"); // TODO: handle this
    }
    let intent = if pixel.a == 0 {
        0
    } else {
        pixel.r / 3 + pixel.g / 3 + pixel.b / 3
    };

    if intent == 0 {
        return 0;
    }

    // I'll kill my self if this didn't work.
    intent % characters.len() as u8
}
