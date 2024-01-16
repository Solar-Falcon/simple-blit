use core::{
    marker::PhantomData,
    ops::{Deref, DerefMut, Index, IndexMut},
};

#[cfg(feature = "pixels-integration")]
pub use pixels::Pixels;
#[cfg(feature = "pixels-integration")]
use rgb::AsPixels;
#[cfg(feature = "pixels-integration")]
pub use rgb::RGBA8;

/// Generic buffer with width and height.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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

/// A 'buffer' that holds a single value, like a plain-colored rectangle.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SingleValueBuffer<T> {
    /// Buffer width.
    pub width: u32,
    /// Buffer height.
    pub height: u32,
    /// Stored value, likely a color.
    pub value: T,
}

impl<T> SingleValueBuffer<T> {
    /// Construct a new buffer.
    #[inline]
    pub const fn new(width: u32, height: u32, value: T) -> Self {
        Self {
            width,
            height,
            value,
        }
    }
}

impl<T> Buffer<T> for SingleValueBuffer<T> {
    #[inline]
    fn width(&self) -> u32 {
        self.width
    }

    #[inline]
    fn height(&self) -> u32 {
        self.height
    }

    #[inline]
    fn get(&self, _x: u32, _y: u32) -> &T {
        &self.value
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

#[cfg(feature = "pixels-integration")]
impl Buffer<RGBA8> for Pixels {
    #[inline]
    fn width(&self) -> u32 {
        self.texture().width()
    }

    #[inline]
    fn height(&self) -> u32 {
        self.texture().height()
    }

    #[inline]
    fn get(&self, x: u32, y: u32) -> &RGBA8 {
        self.frame()
            .as_pixels()
            .index((y * self.width() + x) as usize)
    }
}

#[cfg(feature = "pixels-integration")]
impl BufferMut<RGBA8> for Pixels {
    #[inline]
    fn get_mut(&mut self, x: u32, y: u32) -> &mut RGBA8 {
        let width = self.width();

        self.frame_mut()
            .as_pixels_mut()
            .index_mut((y * width + x) as usize)
    }
}
