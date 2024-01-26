use crate::Rgba;

const EMPTY_CHAR: char = ' ';

#[derive(Debug, PartialEq, PartialOrd, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Sympols {
    set: Vec<char>,
}

impl Sympols {
    pub const fn new(set: Vec<char>) -> Sympols {
        Sympols { set }
    }
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

    #[inline]
    pub(crate) fn sym(&self, pixel: &Rgba) -> char {
        self.get(self.sym_index(pixel))
    }

    #[inline]
    pub(crate) fn sym_and_index(&self, pixel: &Rgba) -> (char, usize) {
        let idx = self.sym_index(pixel);
        (self.get(idx), idx)
    }
}

impl From<&[char]> for Sympols {
    fn from(value: &[char]) -> Self {
        Self::new(value.into())
    }
}

impl From<Vec<char>> for Sympols {
    fn from(value: Vec<char>) -> Self {
        Self::new(value)
    }
}
