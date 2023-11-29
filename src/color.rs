use std::fmt::Display;

use crate::Rgba;

pub(crate) const ANSI_ESCAPE_CLOSE: &str = "\u{001b}[0m";
const ANSI_FOREGROUND_ESCAPE: &str = "\u{001b}[38;2;";
const ANSI_BACKGROUND_ESCAPE: &str = "\u{001b}[48;2;";
const ANSI_COLOR_CODE_LEN: usize = 12;
pub const TRANSBARENT: ANSIColor = ANSIColor {
    inner: String::new(),
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ANSIColor {
    inner: String,
}

impl ANSIColor {
    pub fn new(r: u8, g: u8, b: u8) -> ANSIColor {
        let mut c = String::with_capacity(ANSI_COLOR_CODE_LEN);

        for i in [r, g, b] {
            c.push_str(&i.to_string());
            c.push(';');
        }
        let _ = c.pop();

        debug_assert_eq!(c.capacity(), ANSI_COLOR_CODE_LEN);

        ANSIColor { inner: c }
    }

    #[inline(always)]
    pub fn is_transparent(&self) -> bool {
        self.inner.is_empty()
    }
}

impl Display for ANSIColor {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_transparent() {
            return Ok(());
        }
        if f.sign_minus() {
            write!(f, "{ANSI_BACKGROUND_ESCAPE}{}m", self.inner)
        } else {
            write!(f, "{ANSI_FOREGROUND_ESCAPE}{}m", self.inner)
        }
    }
}

impl<T> From<(T, T, T)> for ANSIColor
where
    T: Into<u8>,
{
    fn from(v: (T, T, T)) -> Self {
        ANSIColor::new(v.0.into(), v.1.into(), v.2.into())
    }
}

impl From<Rgba> for ANSIColor {
    #[inline(always)]
    fn from(value: Rgba) -> Self {
        let Rgba { r, g, b, a } = value;
        if a == 0 {
            return TRANSBARENT;
        }
        ANSIColor::new(r, g, b)
    }
}
