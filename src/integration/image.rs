use crate::{size, Point, Size, Surface, SurfaceMut};
use core::ops::{Deref, DerefMut};
use image::Pixel;

pub use image::ImageBuffer;

impl<Pix, Container> Surface<Pix> for ImageBuffer<Pix, Container>
where
    Pix: Pixel,
    Container: Deref<Target = [Pix::Subpixel]>,
{
    #[inline]
    fn surface_size(&self) -> Size {
        size(self.width(), self.height())
    }

    #[inline]
    fn surface_get(&self, pt: Point) -> Option<&Pix> {
        self.get_pixel_checked(pt.x, pt.y)
    }
}

impl<Pix, Container> SurfaceMut<Pix> for ImageBuffer<Pix, Container>
where
    Pix: Pixel,
    Container: Deref<Target = [Pix::Subpixel]> + DerefMut,
{
    #[inline]
    fn surface_get_mut(&mut self, pt: Point) -> Option<&mut Pix> {
        self.get_pixel_mut_checked(pt.x, pt.y)
    }
}
