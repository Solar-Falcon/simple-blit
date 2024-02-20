use crate::{size, Point, Size, Surface, SurfaceMut};
use rgb::AsPixels;

pub use pixels::Pixels;
pub use rgb::RGBA8;

impl Surface<RGBA8> for Pixels {
    #[inline]
    fn surface_size(&self) -> Size {
        size(self.texture().width(), self.texture().height())
    }

    #[inline]
    fn surface_get(&self, pt: Point) -> Option<&RGBA8> {
        self.frame()
            .as_pixels()
            .get((pt.y * self.texture().width() + pt.x) as usize)
    }
}

impl SurfaceMut<RGBA8> for Pixels {
    #[inline]
    fn surface_get_mut(&mut self, pt: Point) -> Option<&mut RGBA8> {
        let width = self.texture().width();

        self.frame_mut()
            .as_pixels_mut()
            .get_mut((pt.y * width + pt.x) as usize)
    }
}
