#![no_std]
#![warn(missing_docs)]
#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]

use core::{
    cmp::min,
    ops::{Deref, DerefMut},
};

/// Blit from one buffer to another.
///
/// Crops the rectangle if it doesn't fit.
#[inline]
pub fn blit<'a, T: Clone + 'a>(
    dest: impl AsMut<BufferMut<'a, T>>,
    dest_pos: (i32, i32),
    src: impl AsRef<Buffer<'a, T>>,
    src_pos: (i32, i32),
    size: (u32, u32),
) {
    blit_with(dest, dest_pos, src, src_pos, size, |dest, src, _| {
        dest.clone_from(src)
    });
}

/// Blit one whole buffer to another.
///
/// Crops the rectangle if it doesn't fit.
#[inline]
pub fn blit_full<'a, T: Clone + 'a>(
    dest: impl AsMut<BufferMut<'a, T>>,
    dest_pos: (i32, i32),
    src: impl AsRef<Buffer<'a, T>>,
) {
    let size = (src.as_ref().width, src.as_ref().height);
    blit(dest, dest_pos, src, (0, 0), size);
}

/// Blit one whole buffer to another.
///
/// Crops the rectangle if it doesn't fit.
/// Values equal to `mask` will be skipped.
#[inline]
pub fn blit_masked<'a, T: Clone + PartialEq + 'a>(
    dest: BufferMut<'a, T>,
    dest_pos: (i32, i32),
    src: impl AsRef<Buffer<'a, T>>,
    src_pos: (i32, i32),
    size: (u32, u32),
    mask: &T,
) {
    blit_with(dest, dest_pos, src, src_pos, size, |dest, src, _| {
        if src != mask {
            dest.clone_from(src);
        }
    })
}

/// Blit one whole buffer to another (generalized function).
///
/// Crops the rectangle if it doesn't fit.
/// `f` is called for each pair of values, the last argument
/// is their position relative to the (already cropped if necessary) rectangle that is being blitted.
pub fn blit_with<'a, T: 'a, U: 'a>(
    mut dest: impl AsMut<BufferMut<'a, T>>,
    dest_pos: (i32, i32),
    src: impl AsRef<Buffer<'a, U>>,
    src_pos: (i32, i32),
    size: (u32, u32),
    mut f: impl FnMut(&mut T, &U, (i32, i32)),
) {
    let dest = dest.as_mut();
    let src = src.as_ref();

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
        min(dest.width.saturating_sub(dx), src.width.saturating_sub(sx)),
        min(sw, dw),
    ) as usize;
    let copy_height = min(
        min(
            dest.height.saturating_sub(dy),
            src.height.saturating_sub(sy),
        ),
        min(dh, sh),
    ) as usize;

    for iy in 0..copy_height {
        let dest_offset = (iy + dy as usize) * dest.width as usize + dx as usize;
        let src_offset = (iy + sy as usize) * src.width as usize + sx as usize;

        for ix in 0..copy_width {
            f(
                &mut dest[dest_offset + ix],
                &src[src_offset + ix],
                (ix as _, iy as _),
            )
        }
    }
}

/// Immutable buffer with width and height
pub struct Buffer<'a, T> {
    slice: &'a [T],
    width: u32,
    height: u32,
}

impl<'a, T> Buffer<'a, T> {
    /// Construct a new buffer
    ///
    /// Returns `None` if `slice.len() != width * height`
    #[inline]
    pub fn new(slice: &'a [T], width: u32, height: u32) -> Option<Self> {
        if slice.len() == (width * height) as _ {
            Some(Self {
                slice,
                width,
                height,
            })
        } else {
            None
        }
    }

    /// Constructs a new buffer
    ///
    /// Infers the height from slice length and width
    #[inline]
    pub fn new_infer(slice: &'a [T], width: u32) -> Self {
        Self {
            slice,
            width,
            height: slice.len() as u32 / width,
        }
    }

    /// Get the buffer width
    #[inline]
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Get the buffer height
    #[inline]
    pub fn height(&self) -> u32 {
        self.height
    }
}

impl<'a, T> Deref for Buffer<'a, T> {
    type Target = [T];

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.slice
    }
}

impl<'a, T> AsRef<Self> for Buffer<'a, T> {
    #[inline]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl<'a, T> AsMut<Self> for Buffer<'a, T> {
    #[inline]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

/// Mutable buffer with width and height
pub struct BufferMut<'a, T> {
    slice: &'a mut [T],
    width: u32,
    height: u32,
}

impl<'a, T> BufferMut<'a, T> {
    /// Construct a new buffer
    ///
    /// Returns `None` if `slice.len() != width * height`
    #[inline]
    pub fn new(slice: &'a mut [T], width: u32, height: u32) -> Option<Self> {
        if slice.len() == (width * height) as _ {
            Some(Self {
                slice,
                width,
                height,
            })
        } else {
            None
        }
    }

    /// Constructs a new buffer
    ///
    /// Infers the height from slice length and width
    #[inline]
    pub fn new_infer(slice: &'a mut [T], width: u32) -> Self {
        Self {
            height: slice.len() as u32 / width,
            slice,
            width,
        }
    }

    /// Get the buffer width
    #[inline]
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Get the buffer height
    #[inline]
    pub fn height(&self) -> u32 {
        self.height
    }
}

impl<'a, T> Deref for BufferMut<'a, T> {
    type Target = [T];

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.slice
    }
}

impl<'a, T> DerefMut for BufferMut<'a, T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.slice
    }
}

impl<'a, T> AsRef<Self> for BufferMut<'a, T> {
    #[inline]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl<'a, T> AsMut<Self> for BufferMut<'a, T> {
    #[inline]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        let mut dest = [0_u8; 25];

        let dest_buf = BufferMut::new(&mut dest, 5, 5).unwrap();

        let src = [1_u8; 16];

        let src_buf = Buffer::new(&src, 4, 4).unwrap();

        blit(dest_buf, (1, 1), src_buf, (0, 0), (3, 3));

        #[rustfmt::skip]
        let correct: [u8; 25] = [
            0, 0, 0, 0, 0,
            0, 1, 1, 1, 0,
            0, 1, 1, 1, 0,
            0, 1, 1, 1, 0,
            0, 0, 0, 0, 0,
        ];

        assert_eq!(dest, correct);
    }

    #[test]
    fn dest_oob() {
        let mut dest = [0_u8; 25];

        let dest_buf = BufferMut::new(&mut dest, 5, 5).unwrap();

        let src = [1_u8; 16];

        let src_buf = Buffer::new(&src, 4, 4).unwrap();

        blit(dest_buf, (-1, -1), src_buf, (0, 0), (4, 4));

        #[rustfmt::skip]
        let correct: [u8; 25] = [
            1, 1, 1, 0, 0,
            1, 1, 1, 0, 0,
            1, 1, 1, 0, 0,
            0, 0, 0, 0, 0,
            0, 0, 0, 0, 0,
        ];

        assert_eq!(dest, correct);
    }

    #[test]
    fn too_small() {
        let mut dest = [0_u8; 25];

        let dest_buf = BufferMut::new(&mut dest, 5, 5).unwrap();

        let src = [1_u8; 16];

        let src_buf = Buffer::new(&src, 4, 4).unwrap();

        blit(dest_buf, (-1, -1), src_buf, (-1, -1), (6, 6));

        #[rustfmt::skip]
        let correct: [u8; 25] = [
            1, 1, 1, 1, 0,
            1, 1, 1, 1, 0,
            1, 1, 1, 1, 0,
            1, 1, 1, 1, 0,
            0, 0, 0, 0, 0,
        ];

        assert_eq!(dest, correct);
    }
}
