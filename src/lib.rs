#[cfg(feature = "colors")]
pub mod color;
#[cfg(feature = "image")]
pub mod impl_image;
pub mod sympols;

#[cfg(feature = "colors")]
use color::ANSIColor;
use color::{ANSI_BACKGROUND_ESCAPE, ANSI_ESCAPE_CLOSE, ANSI_FOREGROUND_ESCAPE};
use sympols::Sympols;

use std::{
    error::Error,
    fmt::{self, Display},
    io::Write,
    mem,
};

/// Use colos.
pub const COLORS: u8 = 0b1;
/// Reverse the forgruond color with the background.
pub const REVERSE: u8 = 0b10;

/// Represent the ASCII art.
#[derive(Debug, PartialEq, PartialOrd, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BufferdWriter<'a> {
    pub config: Config<'a>,
    fragments: Vec<IndexdFragment>,
    /// The columans number.
    pub row_len: usize,
    pub flags: u8,
}

impl BufferdWriter<'_> {
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
        if let Some(bc) = &self.config.bc {
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
                if let Some(bc) = &self.config.bc {
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
            if i == self.row_len {
                i = 0;
                writeln!(f)?;
            }
            i += 1;

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
                ch = self.config.sympols.get(frag.ch_index as usize)
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
            if i == self.row_len {
                i = 0;
                writeln!(f)?;
            }
            f.write_str(&self.config.sympols.get(frag.ch_index as usize).to_string())?;
            i += 1;
        }
        Ok(())
    }
}

impl Display for BufferdWriter<'_> {
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

pub trait FragmentWriter {
    fn background(&mut self, bc: &ANSIColor) -> Result<bool, Box<dyn Error>>;
    fn write_fragment(&mut self, fragment: Fragment) -> Result<(), Box<dyn Error>>;
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
    fn write_fragment(&mut self, fragment: Fragment) -> Result<(), Box<dyn Error>> {
        self.write_all(fragment.ch.to_string().as_bytes())?;
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
pub struct Config<'a> {
    pub sympols: Sympols<'a>,
    #[cfg(feature = "colors")]
    pub bc: Option<ANSIColor>,
    /// The columans number.
    pub flags: u8,
}

impl Config<'_> {
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
pub struct IndexdFragment {
    ch_index: u8,
    #[cfg(feature = "colors")]
    fg: ANSIColor,
}

impl IndexdFragment {
    #[cfg(not(feature = "colors"))]
    #[inline(always)]
    fn new(ch_index: u8) -> Self {
        Self { ch_index }
    }

    #[cfg(feature = "colors")]
    #[inline(always)]
    fn new(ch_index: u8, fc: ANSIColor) -> Self {
        Self { ch_index, fg: fc }
    }
}

pub struct Fragment {
    ch: char,
    #[cfg(feature = "colors")]
    fg: ANSIColor,
}

impl Fragment {
    #[cfg(not(feature = "colors"))]
    #[inline(always)]
    fn new(ch: char) -> Self {
        Self { ch }
    }

    #[cfg(feature = "colors")]
    #[inline(always)]
    fn new(ch: char, fc: ANSIColor) -> Self {
        Self { ch, fg: fc }
    }
}

pub struct FragmentInfo {
    sym: char,
    sym_index: usize,
}

/// Trait to convert an imgae to ASCII art.
pub trait ToTextImage {
    fn to_text<'a>(&self, set: Sympols<'a>) -> BufferdWriter<'a>;
}

// impl<T> ToTextImage for T
// where
//     T: PixelImage,
// {
//     #[inline(always)]
//     fn to_text<'a>(&self, set: Sympols<'a>) -> TextImage<'a> {
//         crate::convert_image_to_ascii(self, set)
//     }
// }

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
    // let frag_cap = (width * height) as usize;
    // let mut fragments = Vec::with_capacity(frag_cap);
    let ansi_close = if let Some(bc) = &config.bc {
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
                    let fi = FragmentInfo { sym, sym_index };
                    let mut fc = Some(ANSIColor::from(pixel));
                    let mut bc = config.bc.clone();
                    if !ansi_close && config.reversed() {
                        mem::swap(&mut bc, &mut fc);
                    }
                    out.write_colored_fragment(fi, bc.as_ref(), fc.as_ref())?;
                }
            } else {
                out.write_fragment(Fragment::new(config.sympols.sym(&pixel), pixel.into()))?;
            }
        }
        out.write_bytes("\n".as_bytes())?;
    }

    if ansi_close {
        out.write_bytes(ANSI_ESCAPE_CLOSE.as_bytes())?;
    }

    Ok(())
}
