use crate::Rgba;

/// An empty sympols set.
pub const EMPTY_SET: Sympols = Sympols::new(vec![]);

/// The default empty character(a.k.a. sympol). Reterund by [`Sympols::get`] when the sympols is [`EMPTY_SET`].
pub const EMPTY_CHAR: char = ' ';

/// The sympols (characters) that we will use to represent our pixels based on their color.
#[derive(Debug, PartialEq, PartialOrd, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Sympols {
    set: Vec<char>,
}

impl Sympols {
    /// Construct an new sympols set.
    pub const fn new(set: Vec<char>) -> Sympols {
        Sympols { set }
    }

    /// Construct a new empty set.
    pub const fn empty() -> Self {
        EMPTY_SET
    }

    /// Get the char that in a spicfic index, if the set is [`EMPTY_SET`] it'll always return an [`EMPTY_CHAR`].
    #[inline(always)]
    pub fn get(&self, i: usize) -> char {
        if self.is_empty() {
            return EMPTY_CHAR;
        }
        self.set[i]
    }

    /// The set length.
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.set.len()
    }

    /// Return true if the set is empty.
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.set.is_empty()
    }

    /// Calculate the index of the sympol in the set based on the [`RGBA`] value.
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

    /// Calculate the index if the sympol in the set based on th [`RGBA`] value, and return it.
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
