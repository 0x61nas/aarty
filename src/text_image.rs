use core::fmt;
use std::fmt::Display;

use crate::{Config, FragmentInfo, FragmentWriter, PixelImage};

#[cfg(feature = "colors")]
use crate::color::{ANSIColor, ANSI_ESCAPE_CLOSE};

/// Trait to convert an imgae to ASCII art.
pub trait ToTextImage {
    /// constructs a [`TextImage`] instance and use it with [`crate::convert_image_to_ascii`] and return it.
    fn to_text(&self, cfg: Config) -> TextImage;
}

impl<T> ToTextImage for T
where
    T: PixelImage,
{
    #[inline]
    fn to_text(&self, cfg: Config) -> TextImage {
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
    /// The config.
    pub config: Config,
    fragments: Vec<IndexdFragment>,
    /// The columans number.
    pub row_len: usize,
}

/// Represent the fragment by the sympol index in the sympols set.
/// it can be useful by reduce the size that required to store a [`crate::Fragment`] by 3 bytes less.
#[derive(Debug, PartialEq, PartialOrd, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub struct IndexdFragment {
    /// The sympol index in the provided sympols set.
    pub sym_index: u8,
    /// The symplol foregruond color.
    #[cfg(feature = "colors")]
    pub fg: ANSIColor,
}

impl IndexdFragment {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub const fn new(sym_index: u8) -> Self {
        Self {
            sym_index,
            #[cfg(feature = "colors")]
            fg: crate::color::TRANSBARENT,
        }
    }
}

#[cfg(feature = "colors")]
impl IndexdFragment {
    /// Set the foreground color.
    #[inline]
    #[must_use]
    pub fn with_foreground(mut self, fg: impl Into<ANSIColor>) -> Self {
        self.fg = fg.into();
        self
    }

    /// Construct a new instance with foreground color.
    #[inline]
    #[must_use]
    pub const fn new_with_foreground(sym_index: u8, fg: ANSIColor) -> Self {
        Self { sym_index, fg }
    }
}

impl From<FragmentInfo> for IndexdFragment {
    #[inline]
    fn from(v: FragmentInfo) -> Self {
        Self {
            sym_index: v.sym_index as u8,
            #[cfg(feature = "colors")]
            fg: v.fg,
        }
    }
}

/// Represent pixel in tty context.
#[derive(Debug, PartialEq, PartialOrd, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Fragment {
    ch: char,
    #[cfg(feature = "colors")]
    fg: ANSIColor,
}

impl Fragment {
    /// Construct a new instance.
    #[inline(always)]
    pub const fn new(ch: char) -> Self {
        Self {
            ch,
            #[cfg(feature = "colors")]
            fg: crate::color::TRANSBARENT,
        }
    }

    /// Get the character.
    #[inline(always)]
    pub const fn sym(&self) -> char {
        self.ch
    }
}

#[cfg(feature = "colors")]
impl Fragment {
    /// Set the foreground color.
    #[inline]
    #[must_use]
    pub fn with_foreground(mut self, fg: impl Into<ANSIColor>) -> Self {
        self.fg = fg.into();
        self
    }

    /// Construct a new instance with foreground color.
    #[inline(always)]
    pub const fn new_with_foueground(ch: char, fg: ANSIColor) -> Self {
        Self { ch, fg }
    }

    /// Get the fragment foreground.
    #[inline(always)]
    pub const fn foreground(&self) -> &ANSIColor {
        &self.fg
    }
}

impl From<FragmentInfo> for Fragment {
    fn from(v: FragmentInfo) -> Self {
        Self {
            ch: v.sym,
            #[cfg(feature = "colors")]
            fg: v.fg,
        }
    }
}

impl TextImage {
    /// Construct a new instance.
    pub fn new(cfg: Config, w: u32, h: u32) -> Self {
        Self {
            config: cfg,
            fragments: Vec::with_capacity(w as usize * h as usize),
            row_len: w as usize,
        }
    }

    /// Get the fragment at a specific courdents.
    ///
    /// this may return [`None`] if the specifiyed courdenates is out of range.
    ///
    /// See: [`TextImage::fragment_at_unchecked`]
    pub fn fragment_at(&self, x: u32, y: u32) -> Option<Fragment> {
        self.get(x as usize * self.row_len + y as usize)
    }

    /// Get the fragment at a specific index.
    ///
    /// this may return [`None`] if the specifiyed index is out of range.
    ///
    /// See: [`TextImage::get_unchecked`]
    pub fn get(&self, idx: usize) -> Option<Fragment> {
        if idx < self.len() {
            return Some(unsafe { self.get_unchecked(idx) });
        }
        None
    }

    /// Get the fragment at a specific index without the i range checking overhead
    ///
    /// # Safety
    /// The caller must check from that the index is in the range.
    pub unsafe fn get_unchecked(&self, idx: usize) -> Fragment {
        let fragment = self.fragments.get_unchecked(idx);
        Fragment {
            ch: self.config.sympols.get(fragment.sym_index as usize),
            #[cfg(feature = "colors")]
            fg: fragment.fg.clone(),
        }
    }

    /// Get the fragment at a specific courdenates without the i range checking overhead
    ///
    /// # Safety
    /// The caller must check from the courdents that its in the range.
    pub unsafe fn fragment_at_unchecked(&self, x: u32, y: u32) -> Fragment {
        self.get_unchecked(x as usize * self.row_len + y as usize)
    }

    /// Insert a new fragment at index.
    #[inline]
    pub fn insert(&mut self, idx: usize, fragment: IndexdFragment) {
        self.fragments.insert(idx, fragment);
    }

    /// Insert a new fragment at courdetates.
    #[inline]
    pub fn put(&mut self, x: u32, y: u32, fragment: IndexdFragment) {
        self.insert(x as usize * self.row_len + y as usize, fragment);
    }

    /// The fragments inner array len.
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.fragments.len()
    }

    /// Return true if the inner fragment array is empty.
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline(always)]
    fn _fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut i = 0;
        for frag in &self.fragments {
            if i == self.row_len {
                i = 0;
                writeln!(f)?;
            }
            f.write_str(&self.config.sympols.get(frag.sym_index as usize).to_string())?;
            i += 1;
        }
        Ok(())
    }
}

#[cfg(feature = "colors")]
impl TextImage {
    #[inline(always)]
    fn _background(&self, f: &mut fmt::Formatter<'_>) -> std::result::Result<bool, fmt::Error> {
        if let Some(bc) = &self.config.background {
            write!(f, "{bc:-}")?;
            return Ok(true);
        }
        Ok(false)
    }

    #[inline]
    fn _color_fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let has_background = {
            if self.config.reversed() {
                let mut r = false;
                if let Some(bc) = &self.config.background {
                    if !bc.is_transparent() {
                        write!(f, "{bc}")?;
                        r = true;
                    }
                }
                r
            } else {
                self._background(f)?
            }
        };

        let mut i = 0;
        for frag in &self.fragments {
            if i == self.row_len {
                i = 0;
                writeln!(f)?;
            }
            i += 1;

            if self.config.reversed() {
                write!(f, "{c:-}", c = frag.fg)
            } else {
                write!(f, "{c}", c = frag.fg)
            }?;

            write!(
                f,
                "{ch}{ANSI_ESCAPE_CLOSE}",
                ch = self.config.sympols.get(frag.sym_index as usize)
            )?;
        }

        if has_background {
            f.write_str(ANSI_ESCAPE_CLOSE)?;
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
