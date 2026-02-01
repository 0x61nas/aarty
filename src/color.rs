use std::fmt::Display;

use crate::Rgba;

pub(crate) const ANSI_ESCAPE_CLOSE: &str = "\u{001b}[0m";
pub(crate) const ANSI_FOREGROUND_ESCAPE: &str = "\u{001b}[38;2;";
pub(crate) const ANSI_BACKGROUND_ESCAPE: &str = "\u{001b}[48;2;";
pub(crate) const ANSI_COLOR_CODE_LEN: usize = 12;
pub(crate) const TRANSPARENT: ANSIColor = ANSIColor {
    inner: [0u8; ANSI_COLOR_CODE_LEN],
};

/// ansi color.
#[derive(Debug, PartialEq, PartialOrd, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ANSIColor {
    inner: [u8; ANSI_COLOR_CODE_LEN],
}

impl ANSIColor {
    /// Construct a new instance.
    pub fn new(r: u8, g: u8, b: u8) -> ANSIColor {
        let mut c = [0u8; ANSI_COLOR_CODE_LEN];

        let mut cur = 0usize;
        for i in [r, g, b] {
            for ch in i.to_string().chars() {
                c[cur] = ch as u8;
                cur += 1;
            }
            c[cur] = b';';
            cur += 1;
        }
        c[cur - 1] = 0;

        debug_assert_eq!(c.len(), ANSI_COLOR_CODE_LEN);

        ANSIColor { inner: c }
    }

    /// return true if the color is transparent.
    #[inline(always)]
    pub fn is_transparent(&self) -> bool {
        self.inner[0] == 0
    }

    /// Return an ANSI escaped background color.
    pub fn as_background(&self) -> String {
        let len = self.calc_inner_len();
        unsafe {
            format!(
                "{ANSI_BACKGROUND_ESCAPE}{}m",
                std::str::from_utf8_unchecked(&self.inner[..len])
            )
        }
    }

    /// Return an ANSI escaped foreground color.
    pub fn as_foreground(&self) -> String {
        let len = self.calc_inner_len();
        unsafe {
            format!(
                "{ANSI_FOREGROUND_ESCAPE}{}m",
                std::str::from_utf8_unchecked(&self.inner[..len])
            )
        }
    }

    #[inline]
    fn calc_inner_len(&self) -> usize {
        let mut len = self.inner.len();
        for (i, c) in self.inner.into_iter().enumerate() {
            if c == 0 {
                len = i;
                break;
            }
        }
        len
    }

}

impl Display for ANSIColor {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_transparent() {
            return Ok(());
        }
        let len = self.calc_inner_len();
        if f.sign_minus() {
            unsafe {
                write!(f, "{ANSI_BACKGROUND_ESCAPE}{}m", std::str::from_utf8_unchecked(&self.inner[..len]))
            }
        } else {
            unsafe {
                write!(f, "{ANSI_FOREGROUND_ESCAPE}{}m", std::str::from_utf8_unchecked(&self.inner[..len]))
            }
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
        if a < 120 {
            return TRANSPARENT;
        }
        ANSIColor::new(r, g, b)
    }
}
