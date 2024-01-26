use core::fmt;
use std::fmt::Display;

use crate::{Config, FragmentInfo, FragmentWriter, PixelImage};

#[cfg(feature = "colors")]
use crate::color::{ANSIColor, ANSI_ESCAPE_CLOSE};

/// Trait to convert an imgae to ASCII art.
pub trait ToTextImage {
    fn to_text(&self, cfg: Config) -> TextImage;
}

impl<T> ToTextImage for T
where
    T: PixelImage,
{
    #[inline]
    fn to_text<'a>(&self, cfg: Config) -> TextImage {
        let (w, h) = self.dimensions();
        let mut buf = TextImage::new(cfg.clone(), w, h);
        crate::convert_image_to_ascii(&cfg, self, &mut buf).expect("Unretchable");
        buf
    }
}

/// Represent the ASCII art.
#[derive(Debug, PartialEq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TextImage {
    pub config: Config,
    fragments: Vec<IndexdFragment>,
    /// The columans number.
    pub row_len: usize,
}
#[derive(Debug, PartialEq, PartialOrd, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IndexdFragment {
    ch_index: u8,
    #[cfg(feature = "colors")]
    fg: ANSIColor,
}

impl From<FragmentInfo> for IndexdFragment {
    #[inline]
    fn from(v: FragmentInfo) -> Self {
        Self {
            ch_index: v.sym_index as u8,
            #[cfg(feature = "colors")]
            fg: v.fg,
        }
    }
}

impl TextImage {
    pub fn new(cfg: Config, w: u32, h: u32) -> Self {
        Self {
            config: cfg,
            fragments: Vec::with_capacity(w as usize * h as usize),
            row_len: w as usize,
        }
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
        if let Some(bc) = &self.config.bc {
            write!(f, "{bc:-}")?;
            return Ok(true);
        }
        Ok(false)
    }

    #[cfg(feature = "colors")]
    #[inline]
    fn _color_fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let has_background = {
            #[cfg(feature = "reverse")]
            if self.config.reversed() {
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
            if self.config.reversed() {
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

impl Display for TextImage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        #[cfg(feature = "colors")]
        {
            if self.config.use_colors() {
                self._color_fmt(f)
            } else {
                self._fmt(f)
            }
        }

        #[cfg(not(feature = "colors"))]
        self._fmt(f)
    }
}

impl FragmentWriter for TextImage {
    #[cfg(feature = "colors")]
    #[inline(always)]
    fn background(&mut self, _: &ANSIColor) -> Result<bool, Box<dyn std::error::Error>> {
        // Nah, I don't care, I have my configs :p
        //  but pretent like if you care so it will skip the swap operation.
        Ok(true)
    }

    fn write_fragment(&mut self, info: FragmentInfo) -> Result<(), Box<dyn std::error::Error>> {
        self.fragments.push(info.into());
        Ok(())
    }

    #[cfg(feature = "colors")]
    fn write_colored_fragment(
        &mut self,
        info: FragmentInfo,
        _: Option<&ANSIColor>,
        _: Option<&ANSIColor>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.fragments.push(info.into());
        Ok(())
    }

    #[inline(always)]
    fn write_bytes(&mut self, _bytes: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        // Just ignore them
        Ok(())
    }
}
