use crate::{point, size, Point, Size, Surface, SurfaceMut};

/// Transformations that can be applied when blitting.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
/// `f` is called for each pair of values, the last argument is the value's source position.
/// The transforms are done in order.
pub fn blit_with<D, S>(
    dest: &mut (impl SurfaceMut<D> + ?Sized),
    dest_pos: Point,
    src: &(impl Surface<S> + ?Sized),
    src_pos: Point,
    copy_size: Size,
    transforms: &[Transform],
    mut func: impl FnMut(&mut D, &S, Point),
) {
    let transformed_copy_size = transforms.iter().fold(copy_size, Transform::apply_size);

    for iy in 0..transformed_copy_size.y {
        for ix in 0..transformed_copy_size.x {
            let dest_val_pos = point(ix.saturating_add(dest_pos.x), iy.saturating_add(dest_pos.y));

            let dest = if let Some(dest) = dest.surface_get_mut(dest_val_pos) {
                dest
            } else {
                continue;
            };

            let (untransformed_pos, _untransformed_copy_size) = transforms
                .iter()
                .rev()
                .fold((point(ix, iy), transformed_copy_size), Transform::unapply);

            let src_val_pos = point(
                untransformed_pos.x.saturating_add(src_pos.x),
                untransformed_pos.y.saturating_add(src_pos.y),
            );

            let src = if let Some(src) = src.surface_get(src_val_pos) {
                src
            } else {
                continue;
            };

            (func)(dest, src, untransformed_pos);
        }
    }
}

/// Blit part of one surface to another, cloning the values.
#[inline]
pub fn blit<T: Clone>(
    dest: &mut (impl SurfaceMut<T> + ?Sized),
    dest_pos: Point,
    src: &(impl Surface<T> + ?Sized),
    src_pos: Point,
    copy_size: Size,
    transforms: &[Transform],
) {
    blit_with(
        dest,
        dest_pos,
        src,
        src_pos,
        copy_size,
        transforms,
        |dest, src, _| {
            dest.clone_from(src);
        },
    );
}

/// Blit one whole surface to another.
#[inline]
pub fn blit_whole<T: Clone>(
    dest: &mut (impl SurfaceMut<T> + ?Sized),
    dest_pos: Point,
    src: &(impl Surface<T> + ?Sized),
    src_pos: Point,
    transforms: &[Transform],
) {
    blit(dest, dest_pos, src, src_pos, src.surface_size(), transforms);
}

/// Blit part of one surface to another, ignoring the `mask` values.
#[inline]
pub fn blit_masked<T: Clone + PartialEq>(
    dest: &mut (impl SurfaceMut<T> + ?Sized),
    dest_pos: Point,
    src: &(impl Surface<T> + ?Sized),
    src_pos: Point,
    copy_size: Size,
    transforms: &[Transform],
    mask: &T,
) {
    blit_with(
        dest,
        dest_pos,
        src,
        src_pos,
        copy_size,
        transforms,
        |dest, src, _| {
            if src != mask {
                dest.clone_from(src);
            }
        },
    );
}

/// Blit part of one surface to another, converting the values.
#[inline]
pub fn blit_convert<D: From<S>, S: Clone>(
    dest: &mut (impl SurfaceMut<D> + ?Sized),
    dest_pos: Point,
    src: &(impl Surface<S> + ?Sized),
    src_pos: Point,
    copy_size: Size,
    transforms: &[Transform],
) {
    blit_with(
        dest,
        dest_pos,
        src,
        src_pos,
        copy_size,
        transforms,
        |dest, src, _| {
            *dest = D::from(src.clone());
        },
    );
}
