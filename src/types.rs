use core::{
    marker::PhantomData,
    ops::{Deref, DerefMut, Index, IndexMut},
};

/// Point type.
pub type Point = mint::Point2<u32>;
/// Size type.
pub type Size = mint::Vector2<u32>;

/// Quickly construct a `Point`.
#[inline]
pub const fn point(x: u32, y: u32) -> Point {
    Point { x, y }
}

/// Quickly construct a `Size`.
#[inline]
pub const fn size(x: u32, y: u32) -> Size {
    Size { x, y }
}

/// 2D immutable surface trait.
pub trait Surface<T> {
    /// Surface size.
    fn surface_size(&self) -> Size;

    /// Get a value at (pt.x, pt.y).
    fn surface_get(&self, pt: Point) -> Option<&T>;
}

/// 2D mutable surface trait.
pub trait SurfaceMut<T>: Surface<T> {
    /// Get a mutable value at (pt.x, pt.y).
    fn surface_get_mut(&mut self, pt: Point) -> Option<&mut T>;
}

/// Generic surface with width and height.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GenericSurface<Slice, Item> {
    slice: Slice,
    size: Size,
    ghost: PhantomData<Item>,
}

impl<Slice, Item> GenericSurface<Slice, Item>
where
    Slice: AsRef<[Item]>,
{
    /// Construct a new surface.
    ///
    /// Returns `None` if `slice.len() != size.x * size.y`.
    #[inline]
    pub fn new(slice: Slice, size: Size) -> Option<Self> {
        if slice.as_ref().len() == (size.x * size.y) as _ {
            Some(Self {
                slice,
                size,
                ghost: PhantomData,
            })
        } else {
            None
        }
    }

    /// Constructs a new surface.
    ///
    /// Infers the height from slice length and width.
    #[inline]
    pub fn new_infer(slice: Slice, width: u32) -> Self {
        Self {
            size: size(width, slice.as_ref().len() as u32 / width),
            slice,
            ghost: PhantomData,
        }
    }
}

impl<Slice, Item> Deref for GenericSurface<Slice, Item>
where
    Slice: AsRef<[Item]>,
{
    type Target = [Item];

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.slice.as_ref()
    }
}

impl<Slice, Item> DerefMut for GenericSurface<Slice, Item>
where
    Slice: AsRef<[Item]> + AsMut<[Item]>,
{
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.slice.as_mut()
    }
}

impl<Slice, Item> Surface<Item> for GenericSurface<Slice, Item>
where
    Slice: AsRef<[Item]>,
{
    #[inline]
    fn surface_size(&self) -> Size {
        self.size
    }

    #[inline]
    fn surface_get(&self, pt: Point) -> Option<&Item> {
        if pt.x < self.size.x && pt.y < self.size.y {
            Some(
                self.slice
                    .as_ref()
                    .index((pt.y * self.size.x + pt.x) as usize),
            )
        } else {
            None
        }
    }
}

impl<Slice, Item> SurfaceMut<Item> for GenericSurface<Slice, Item>
where
    Slice: AsRef<[Item]> + AsMut<[Item]>,
{
    #[inline]
    fn surface_get_mut(&mut self, pt: Point) -> Option<&mut Item> {
        if pt.x < self.size.x && pt.y < self.size.y {
            Some(
                self.slice
                    .as_mut()
                    .index_mut((pt.y * self.size.x + pt.x) as usize),
            )
        } else {
            None
        }
    }
}

/// A 'surface' that holds a single value, like a plain-colored rectangle.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SingleValueSurface<T> {
    /// Surface size.
    pub size: Size,
    /// Stored value, likely a color.
    pub value: T,
}

impl<T> SingleValueSurface<T> {
    /// Construct a new surface.
    #[inline]
    pub const fn new(value: T, size: Size) -> Self {
        Self { size, value }
    }
}

impl<T> Surface<T> for SingleValueSurface<T> {
    #[inline]
    fn surface_size(&self) -> Size {
        self.size
    }

    #[inline]
    fn surface_get(&self, _pt: Point) -> Option<&T> {
        Some(&self.value)
    }
}

impl<T> SurfaceMut<T> for SingleValueSurface<T> {
    #[inline]
    fn surface_get_mut(&mut self, _pt: Point) -> Option<&mut T> {
        Some(&mut self.value)
    }
}
