use crate::Rgba;

#[cfg(feature = "_no_ref")]
pub mod no_ref_impl;
#[cfg(not(feature = "_no_ref"))]
pub mod ref_impl;

const EMPTY_CHAR: char = ' ';

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Sympols<'a> {
    #[cfg(not(feature = "_no_ref"))]
    set: &'a [char],
    #[cfg(feature = "_no_ref")]
    set: Vec<char>,
    #[cfg(feature = "_no_ref")]
    _p: std::marker::PhantomData<&'a char>,
}

impl Sympols<'_> {
    #[inline(always)]
    pub fn get(&self, i: usize) -> char {
        if self.is_empty() {
            return EMPTY_CHAR;
        }
        self.set[i]
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.set.len()
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.set.is_empty()
    }

    #[inline]
    pub(crate) fn sym_index(&self, pixel: &Rgba) -> usize {
        if self.is_empty() {
            return 0;
        }
        let len = self.len();
        // FIXME: handle the alpha channel
        let mut idx = (pixel.r as usize + pixel.g as usize + pixel.b as usize) / 3;

        if idx == 0 {
            return 0;
        }

        if pixel.a < 120 {
            idx = pixel.a as usize % idx;
        }

        // I'll kill my self if this didn't work.
        idx /= 255 / len;
        if idx >= len {
            return len - 1;
        }
        idx
    }
}
