use super::Sympols;

impl<'a> Sympols<'a> {
    pub fn new(set: &'a [char]) -> Sympols {
        Sympols { set }
    }
}

impl<'a> From<&'a [char]> for Sympols<'a> {
    fn from(value: &'a [char]) -> Self {
        Self::new(value)
    }
}

impl<'a> From<&'a Vec<char>> for Sympols<'a> {
    fn from(value: &'a Vec<char>) -> Self {
        Self::new(value)
    }
}
