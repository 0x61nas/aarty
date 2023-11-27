use std::{fmt::Display, ops::Index};

use image::{Rgb, Rgba};

pub(crate) const ANSI_ESCAPE_CLOSE: &str = "\u{001b}[0m";
pub(crate) const ANSI_FOREGROUND_ESCAPE: &str = "\u{001b}[38;2;";
pub(crate) const ANSI_BACKGROUND_ESCAPE: &str = "\u{001b}[48;2;";
const ANSI_COLOR_CODE_LEN: usize = 12;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Color {
    /// Red.
    pub r: u8,
    /// Green.
    pub g: u8,
    /// Blue.
    pub b: u8,
}

pub trait ANSIColor {
    fn ansi_color(&self) -> String;
}

impl Index<usize> for Color {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.r,
            1 => &self.g,
            2 => &self.b,
            _ => panic!("Index overflow"),
        }
    }
}

impl ANSIColor for Color {
    fn ansi_color(&self) -> String {
        let mut c = String::with_capacity(ANSI_COLOR_CODE_LEN);

        for i in 0..3 {
            c.push_str(&self[i].to_string());
            c.push(';');
        }
        let _ = c.pop();

        debug_assert_eq!(c.capacity(), ANSI_COLOR_CODE_LEN);

        c
    }
}

impl Display for Color {
    #[inline(always)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.ansi_color())
    }
}

impl From<Rgb<u8>> for Color {
    fn from(value: Rgb<u8>) -> Self {
        Color {
            r: value[0],
            g: value[1],
            b: value[2],
        }
    }
}

impl From<Rgba<u8>> for Color {
    fn from(value: Rgba<u8>) -> Self {
        Color {
            r: value[0],
            g: value[1],
            b: value[2],
        }
    }
}

impl<T> From<(T, T, T)> for Color
where
    T: Into<u8>,
{
    fn from(v: (T, T, T)) -> Self {
        Color {
            r: v.0.into(),
            g: v.1.into(),
            b: v.2.into(),
        }
    }
}
