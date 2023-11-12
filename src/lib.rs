#![no_std]
#![warn(missing_docs)]
#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]

use core::{
    cmp::min,
    ops::{Deref, DerefMut},
};

/// Blit from one buffer to another
///
/// Crops the rectangle if it doesn't fit
pub fn blit<'a, T: Copy>(
    mut dest: BufferMut<'a, T>,
    dest_pos: (i32, i32),
    src: Buffer<'a, T>,
    src_pos: (i32, i32),
    size: (u32, u32),
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

    for (iy, line) in src
        .chunks_exact(src.width as _)
        .enumerate()
        .skip(sy as _)
        .take(copy_height)
    {
        let dest_offset = (iy + dy as usize) * dest.width as usize + dx as usize;
        let src_line_offset = sx as usize;

        dest[dest_offset..(dest_offset + copy_width)]
            .copy_from_slice(&line[src_line_offset..(src_line_offset + copy_width)]);
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
            1, 1, 1, 1, 0,
        ];

        assert_eq!(dest, correct);
    }
}
