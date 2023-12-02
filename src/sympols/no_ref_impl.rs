use std::marker::PhantomData;

use super::Sympols;

impl Sympols<'_> {
    pub fn new(set: Vec<char>) -> Sympols<'static> {
        Sympols {
            set,
            _p: PhantomData,
        }
    }
}

impl From<&[char]> for Sympols<'_> {
    fn from(value: &[char]) -> Self {
        Self::new(value.into())
    }
}

impl From<Vec<char>> for Sympols<'_> {
    fn from(value: Vec<char>) -> Self {
        Self::new(value)
    }
}
