use crate::{color::ANSIColor, PixelImage, Rgba};

impl<T, P> PixelImage for T
where
    T: image::GenericImageView<Pixel = P>,
    P: Into<Rgba> + image::Pixel<Subpixel = u8>,
{
    #[inline(always)]
    fn dimensions(&self) -> (u32, u32) {
        self.dimensions()
    }

    #[inline(always)]
    fn get_pixel(&self, x: u32, y: u32) -> Rgba {
        self.get_pixel(x, y).into()
    }
}

//#[cfg(feature = "image")]
impl<T> From<T> for Rgba
where
    T: image::Pixel<Subpixel = u8>,
{
    fn from(value: T) -> Self {
        let value = value.to_rgba().0;
        Rgba {
            r: value[0],
            g: value[1],
            b: value[2],
            a: value[3],
        }
    }
}
impl From<image::Rgb<u8>> for ANSIColor {
    fn from(value: image::Rgb<u8>) -> Self {
        ANSIColor::new(value[0], value[1], value[2])
    }
}

impl From<image::Rgba<u8>> for ANSIColor {
    fn from(value: image::Rgba<u8>) -> Self {
        ANSIColor::new(value[0], value[1], value[2])
    }
}
