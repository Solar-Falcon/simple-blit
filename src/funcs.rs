use crate::{point, size, Point, Size, Surface, SurfaceMut};

/// Transformations that can be applied when blitting.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub enum Transform {
    /// Scales the destination.
    /// Only upscaling is supported.
    UpScale {
        /// Scale x factor.
        x: u32,
        /// Scale y factor.
        y: u32,
    },

    /// Rotates the destination 90 degrees clockwise.
    Rotate90Cw,
    /// Rotates the destination 90 degrees counter-clockwise.
    Rotate90Ccw,
    /// Rotates the destination 180 degrees.
    Rotate180,

    /// Flips the destination horizontally.
    FlipHorizontal,
    /// Flips the destination vertically.
    FlipVertical,
    /// /// Flips the destination horizontally and vertically.
    FlipBoth,
}

impl Transform {
    #[inline]
    #[allow(dead_code)]
    fn apply((pt, size): (Point, Size), this: &Self) -> (Point, Size) {
        use Transform::*;

        let pt = match this {
            UpScale { x, y } => point(pt.x * x, pt.y * y),

            FlipHorizontal => point(reversed(pt.x, size.x), pt.y),
            FlipVertical => point(pt.x, reversed(pt.y, size.y)),
            FlipBoth => point(reversed(pt.x, size.x), reversed(pt.y, size.y)),

            Rotate90Ccw => point(pt.y, reversed(pt.x, size.x)),
            Rotate90Cw => point(reversed(pt.y, size.y), pt.x),
            Rotate180 => point(reversed(pt.x, size.x), reversed(pt.y, size.y)),
        };

        (pt, Self::apply_size(size, this))
    }

    #[inline]
    fn unapply((pt, size): (Point, Size), this: &Self) -> (Point, Size) {
        use Transform::*;

        let pt = match this {
            UpScale { x, y } => point(pt.x / x, pt.y / y),

            // unchanged
            FlipHorizontal => point(reversed(pt.x, size.x), pt.y),
            FlipVertical => point(pt.x, reversed(pt.y, size.y)),
            FlipBoth => point(reversed(pt.x, size.x), reversed(pt.y, size.y)),
            Rotate180 => point(reversed(pt.x, size.x), reversed(pt.y, size.y)),

            // swapped between each other
            Rotate90Cw => point(pt.y, reversed(pt.x, size.x)),
            Rotate90Ccw => point(reversed(pt.y, size.y), pt.x),
        };

        (pt, Self::unapply_size(size, this))
    }

    #[inline]
    fn apply_size(s: Size, this: &Self) -> Size {
        use Transform::*;

        match this {
            UpScale { x, y } => size(s.x * x, s.y * y),
            Rotate90Cw | Rotate90Ccw => size(s.y, s.x),
            _ => s,
        }
    }

    #[inline]
    fn unapply_size(s: Size, this: &Self) -> Size {
        use Transform::*;

        match this {
            UpScale { x, y } => size(s.x / x, s.y / y),
            Rotate90Cw | Rotate90Ccw => size(s.y, s.x),
            _ => s,
        }
    }
}

#[inline]
fn reversed(coord: u32, size: u32) -> u32 {
    size.saturating_sub(coord).saturating_sub(1)
}

/// Blit part of one surface to another (generalized function).
///
/// You can use `sub_surface` or `offset_surface` functions to limit the copied area.
/// `f` is called for each pair of values, the last argument is the value's source position.
/// The transforms are done in order.
pub fn blit_with<D, S>(
    mut dest: impl SurfaceMut<D>,
    src: impl Surface<S>,
    transforms: &[Transform],
    mut func: impl FnMut(&mut D, &S, Point),
) {
    let copy_size = src.surface_size();
    let transformed_copy_size = transforms.iter().fold(copy_size, Transform::apply_size);

    for iy in 0..transformed_copy_size.y {
        for ix in 0..transformed_copy_size.x {
            let dest_val_pos = point(ix, iy);

            let dest = if let Some(dest) = dest.surface_get_mut(dest_val_pos) {
                dest
            } else {
                continue;
            };

            let (src_val_pos, _untransformed_copy_size) = transforms
                .iter()
                .rev()
                .fold((point(ix, iy), transformed_copy_size), Transform::unapply);

            let src = if let Some(src) = src.surface_get(src_val_pos) {
                src
            } else {
                continue;
            };

            (func)(dest, src, src_val_pos);
        }
    }
}

/// Blit part of one surface to another, cloning the values.
///
/// You can use `sub_surface` or `offset_surface` functions to limit the copied area.
/// The transforms are done in order.
#[inline]
pub fn blit<T: Clone>(dest: impl SurfaceMut<T>, src: impl Surface<T>, transforms: &[Transform]) {
    blit_with(dest, src, transforms, |dest, src, _| {
        dest.clone_from(src);
    });
}

/// Blit part of one surface to another, ignoring the `mask` values.
///
/// You can use `sub_surface` or `offset_surface` functions to limit the copied area.
/// The transforms are done in order.
#[inline]
pub fn blit_masked<T: Clone + PartialEq>(
    dest: impl SurfaceMut<T>,
    src: impl Surface<T>,
    transforms: &[Transform],
    mask: &T,
) {
    blit_with(dest, src, transforms, |dest, src, _| {
        if src != mask {
            dest.clone_from(src);
        }
    });
}

/// Blit part of one surface to another, converting the values.
///
/// You can use `sub_surface` or `offset_surface` functions to limit the copied area.
/// The transforms are done in order.
#[inline]
pub fn blit_convert<D: From<S>, S: Clone>(
    dest: impl SurfaceMut<D>,
    src: impl Surface<S>,
    transforms: &[Transform],
) {
    blit_with(dest, src, transforms, |dest, src, _| {
        *dest = D::from(src.clone());
    });
}
