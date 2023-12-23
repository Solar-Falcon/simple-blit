#![no_std]
#![warn(missing_docs)]
#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]

use core::{
    cmp::min,
    marker::PhantomData,
    ops::{Deref, DerefMut, Index, IndexMut},
};

/// Blit from one buffer to another.
///
/// Crops the rectangle if it doesn't fit.
#[inline]
pub fn blit<T: Clone>(
    dest: &mut impl BufferMut<T>,
    dest_pos: (i32, i32),
    src: &impl Buffer<T>,
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
pub fn blit_full<T: Clone>(
    dest: &mut impl BufferMut<T>,
    dest_pos: (i32, i32),
    src: &impl Buffer<T>,
) {
    let size = (src.width(), src.height());
    blit(dest, dest_pos, src, (0, 0), size);
}

/// Blit one whole buffer to another.
///
/// Crops the rectangle if it doesn't fit.
/// Values equal to `mask` will be skipped.
#[inline]
pub fn blit_masked<T: Clone + PartialEq>(
    dest: &mut impl BufferMut<T>,
    dest_pos: (i32, i32),
    src: &impl Buffer<T>,
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
pub fn blit_with<T, U>(
    dest: &mut impl BufferMut<T>,
    dest_pos: (i32, i32),
    src: &impl Buffer<U>,
    src_pos: (i32, i32),
    size: (u32, u32),
    mut f: impl FnMut(&mut T, &U, (i32, i32)),
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
    ) as usize;
    let copy_height = min(
        min(
            dest.height().saturating_sub(dy),
            src.height().saturating_sub(sy),
        ),
        min(dh, sh),
    ) as usize;

    for iy in 0..copy_height {
        for ix in 0..copy_width {
            f(
                dest.get_mut(dx + ix as u32, dy + iy as u32),
                src.get(sx + ix as u32, sy + iy as u32),
                (ix as _, iy as _),
            );
        }
    }
}

/// Generic buffer with width and height.
pub struct GenericBuffer<Slice, Item> {
    slice: Slice,
    width: u32,
    height: u32,
    ghost: PhantomData<Item>,
}

impl<Slice, Item> GenericBuffer<Slice, Item>
where
    Slice: AsRef<[Item]>,
{
    /// Construct a new buffer.
    ///
    /// Returns `None` if `slice.len() != width * height`.
    #[inline]
    pub fn new(slice: Slice, width: u32, height: u32) -> Option<Self> {
        if slice.as_ref().len() == (width * height) as _ {
            Some(Self {
                slice,
                width,
                height,
                ghost: PhantomData,
            })
        } else {
            None
        }
    }

    /// Constructs a new buffer.
    ///
    /// Infers the height from slice length and width.
    #[inline]
    pub fn new_infer(slice: Slice, width: u32) -> Self {
        Self {
            height: slice.as_ref().len() as u32 / width,
            slice,
            width,
            ghost: PhantomData,
        }
    }
}

impl<Slice, Item> Deref for GenericBuffer<Slice, Item>
where
    Slice: AsRef<[Item]>,
{
    type Target = [Item];

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.slice.as_ref()
    }
}

impl<Slice, Item> DerefMut for GenericBuffer<Slice, Item>
where
    Slice: AsRef<[Item]> + AsMut<[Item]>,
{
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.slice.as_mut()
    }
}

impl<Slice, Item> Buffer<Item> for GenericBuffer<Slice, Item>
where
    Slice: AsRef<[Item]>,
{
    #[inline]
    fn width(&self) -> u32 {
        self.width
    }

    #[inline]
    fn height(&self) -> u32 {
        self.height
    }

    #[inline]
    fn get(&self, x: u32, y: u32) -> &Item {
        self.slice.as_ref().index((y * self.width + x) as usize)
    }
}

impl<Slice, Item> BufferMut<Item> for GenericBuffer<Slice, Item>
where
    Slice: AsRef<[Item]> + AsMut<[Item]>,
{
    #[inline]
    fn get_mut(&mut self, x: u32, y: u32) -> &mut Item {
        self.slice.as_mut().index_mut((y * self.width + x) as usize)
    }
}

/// 2D immutable buffer trait.
pub trait Buffer<T> {
    /// Buffer width
    fn width(&self) -> u32;
    /// Buffer height
    fn height(&self) -> u32;

    /// Get a value at (x, y).
    /// This function must not panic when `x < self.width() && y < self.height()` (unless you want blit functions to panic).
    /// It will not be called with values outside of that range.
    fn get(&self, x: u32, y: u32) -> &T;
}

/// 2D mutable buffer trait.
pub trait BufferMut<T>: Buffer<T> {
    /// Get a mutable value at (x, y).
    /// This function must not panic when `x < self.width() && y < self.height()` (unless you want blit functions to panic).
    /// It will not be called with values outside of that range.
    fn get_mut(&mut self, x: u32, y: u32) -> &mut T;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        let mut dest = [0_u8; 25];

        let mut dest_buf = GenericBuffer::new(&mut dest, 5, 5).unwrap();

        let src = [1_u8; 16];

        let src_buf = GenericBuffer::new(&src, 4, 4).unwrap();

        blit(&mut dest_buf, (1, 1), &src_buf, (0, 0), (3, 3));

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

        let mut dest_buf = GenericBuffer::new(&mut dest, 5, 5).unwrap();

        let src = [1_u8; 16];

        let src_buf = GenericBuffer::new(&src, 4, 4).unwrap();

        blit(&mut dest_buf, (-1, -1), &src_buf, (0, 0), (4, 4));

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

        let mut dest_buf = GenericBuffer::new(&mut dest, 5, 5).unwrap();

        let src = [1_u8; 16];

        let src_buf = GenericBuffer::new(&src, 4, 4).unwrap();

        blit(&mut dest_buf, (-1, -1), &src_buf, (-1, -1), (6, 6));

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
