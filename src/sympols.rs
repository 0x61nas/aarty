use crate::Rgba;

#[cfg(feature = "_no_ref")]
pub mod no_ref_impl;
#[cfg(not(feature = "_no_ref"))]
pub mod ref_impl;

const EMPTY_CHAR: char = ' ';

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
#[cfg_attr(feature = "impl_serde", derive(serde::Serialize, serde::Deserialize))]
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
        // FIXME: handle the alpha channel
        let intent = if pixel.a == 0 {
            0
        } else {
            pixel.r / 3 + pixel.g / 3 + pixel.b / 3
        } as usize;

        if intent == 0 {
            return 0;
        }

        // I'll kill my self if this didn't work.
        intent % self.len()
    }
}
