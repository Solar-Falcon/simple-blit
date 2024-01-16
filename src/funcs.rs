use crate::{Buffer, BufferMut};
use core::cmp::min;

/// Any special options that can be applied.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum BlitOptions {
    /// No special options.
    #[default]
    None,
    /// Flip the result horizontally.
    FlipHorizontal,
    /// Flip the result vertically.
    FlipVertical,
    /// Flip the result horizontally and vertically.
    FlipBoth,
}

/// Blit from one buffer to another.
///
/// Crops the rectangle if it doesn't fit.
#[inline]
pub fn blit<T: Clone>(
    dest: &mut (impl BufferMut<T> + ?Sized),
    dest_pos: (i32, i32),
    src: &(impl Buffer<T> + ?Sized),
    src_pos: (i32, i32),
    size: (u32, u32),
    opts: BlitOptions,
) {
    blit_with(dest, dest_pos, src, src_pos, size, opts, |dest, src, _| {
        dest.clone_from(src)
    });
}

/// Blit one whole buffer to another.
///
/// Crops the rectangle if it doesn't fit.
#[inline]
pub fn blit_full<T: Clone>(
    dest: &mut (impl BufferMut<T> + ?Sized),
    dest_pos: (i32, i32),
    src: &(impl Buffer<T> + ?Sized),
    opts: BlitOptions,
) {
    let size = (src.width(), src.height());
    blit(dest, dest_pos, src, (0, 0), size, opts);
}

/// Blit one whole buffer to another.
///
/// Crops the rectangle if it doesn't fit.
/// Values equal to `mask` will be skipped.
#[inline]
pub fn blit_masked<T: Clone + PartialEq>(
    dest: &mut (impl BufferMut<T> + ?Sized),
    dest_pos: (i32, i32),
    src: &(impl Buffer<T> + ?Sized),
    src_pos: (i32, i32),
    size: (u32, u32),
    mask: &T,
    opts: BlitOptions,
) {
    blit_with(dest, dest_pos, src, src_pos, size, opts, |dest, src, _| {
        if src != mask {
            dest.clone_from(src);
        }
    })
}

/// Blit one buffer to another.
///
/// Crops the rectangle if it doesn't fit.
/// Converts the values automatically.
#[inline]
pub fn blit_convert<T: From<U>, U: Clone>(
    dest: &mut (impl BufferMut<T> + ?Sized),
    dest_pos: (i32, i32),
    src: &(impl Buffer<U> + ?Sized),
    src_pos: (i32, i32),
    size: (u32, u32),
    opts: BlitOptions,
) {
    blit_with(dest, dest_pos, src, src_pos, size, opts, |dest, src, _| {
        *dest = src.clone().into();
    });
}

/// Blit one whole buffer to another (generalized function).
///
/// Crops the rectangle if it doesn't fit.
/// `f` is called for each pair of values, the last argument
/// is their position relative to the (already cropped if necessary) rectangle that is being blitted.
pub fn blit_with<T, U>(
    dest: &mut (impl BufferMut<T> + ?Sized),
    dest_pos: (i32, i32),
    src: &(impl Buffer<U> + ?Sized),
    src_pos: (i32, i32),
    size: (u32, u32),
    opts: BlitOptions,
    mut f: impl FnMut(&mut T, &U, (u32, u32)),
) {
    let (dx, dw) = if dest_pos.0 < 0 {
        (0, size.0.saturating_sub(dest_pos.0.unsigned_abs()))
    } else {
        (dest_pos.0 as u32, size.0)
    };

    let (dy, dh) = if dest_pos.1 < 0 {
        (0, size.1.saturating_sub(dest_pos.1.unsigned_abs()))
    } else {
        (dest_pos.1 as u32, size.1)
    };

    let (sx, sw) = if src_pos.0 < 0 {
        (0, size.0.saturating_sub(src_pos.0.unsigned_abs()))
    } else {
        (src_pos.0 as u32, size.0)
    };

    let (sy, sh) = if src_pos.1 < 0 {
        (0, size.1.saturating_sub(src_pos.1.unsigned_abs()))
    } else {
        (src_pos.1 as u32, size.1)
    };

    let copy_width = min(
        min(
            dest.width().saturating_sub(dx),
            src.width().saturating_sub(sx),
        ),
        min(sw, dw),
    );
    let copy_height = min(
        min(
            dest.height().saturating_sub(dy),
            src.height().saturating_sub(sy),
        ),
        min(sh, dh),
    );

    for iy in 0..copy_height {
        for ix in 0..copy_width {
            let (dst_ix, dst_iy) = match opts {
                BlitOptions::None => (ix, iy),
                BlitOptions::FlipHorizontal => (copy_width - ix - 1, iy),
                BlitOptions::FlipVertical => (ix, copy_height - iy - 1),
                BlitOptions::FlipBoth => (copy_width - ix - 1, copy_height - iy - 1),
            };

            f(
                dest.get_mut(dx + dst_ix, dy + dst_iy),
                src.get(sx + ix, sy + iy),
                (ix, iy),
            );
        }
    }
}
