extern crate alloc;

use self::predefined::Predefined;
use crate::{blit, blit_whole, point, size, GenericSurface, Surface, Transform};
use alloc::format;
use proptest::{
    prelude::prop,
    prop_assert_eq,
    test_runner::{Config, TestRunner},
};

mod predefined {
    use crate::{size, GenericSurface, Transform};

    #[rustfmt::skip]
    const TOP_LEFT: [u8; 9] = [
        1, 2, 3,
        2, 3, 4,
        3, 4, 5,
    ];

    #[rustfmt::skip]
    const TOP_LEFT_2X: [u8; 36] = [
        1, 1, 2, 2, 3, 3,
        1, 1, 2, 2, 3, 3,
        2, 2, 3, 3, 4, 4,
        2, 2, 3, 3, 4, 4,
        3, 3, 4, 4, 5, 5,
        3, 3, 4, 4, 5, 5,
    ];

    #[rustfmt::skip]
    const TOP_RIGHT: [u8; 9] = [
        3, 2, 1,
        4, 3, 2,
        5, 4, 3,
    ];

    #[rustfmt::skip]
    const TOP_RIGHT_2X: [u8; 36] = [
        3, 3, 2, 2, 1, 1,
        3, 3, 2, 2, 1, 1,
        4, 4, 3, 3, 2, 2,
        4, 4, 3, 3, 2, 2,
        5, 5, 4, 4, 3, 3,
        5, 5, 4, 4, 3, 3,
    ];

    #[rustfmt::skip]
    const BOTTOM_LEFT: [u8; 9] = [
        3, 4, 5,
        2, 3, 4,
        1, 2, 3,
    ];

    #[rustfmt::skip]
    const BOTTOM_LEFT_2X: [u8; 36] = [
        3, 3, 4, 4, 5, 5,
        3, 3, 4, 4, 5, 5,
        2, 2, 3, 3, 4, 4,
        2, 2, 3, 3, 4, 4,
        1, 1, 2, 2, 3, 3,
        1, 1, 2, 2, 3, 3,
    ];

    #[rustfmt::skip]
    const BOTTOM_RIGHT: [u8; 9] = [
        5, 4, 3,
        4, 3, 2,
        3, 2, 1,
    ];

    #[rustfmt::skip]
    const BOTTOM_RIGHT_2X: [u8; 36] = [
        5, 5, 4, 4, 3, 3,
        5, 5, 4, 4, 3, 3,
        4, 4, 3, 3, 2, 2,
        4, 4, 3, 3, 2, 2,
        3, 3, 2, 2, 1, 1,
        3, 3, 2, 2, 1, 1,
    ];

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Predefined {
        TopLeft(bool),
        TopRight(bool),
        BottomLeft(bool),
        BottomRight(bool),
    }

    impl Predefined {
        #[inline]
        pub fn is_scaled(self) -> bool {
            match self {
                Predefined::TopLeft(s) => s,
                Predefined::TopRight(s) => s,
                Predefined::BottomLeft(s) => s,
                Predefined::BottomRight(s) => s,
            }
        }

        pub fn surface(self) -> GenericSurface<&'static [u8], u8> {
            use Predefined::*;

            let (slice, size) = match self {
                TopLeft(false) => (&TOP_LEFT[..], size(3, 3)),
                TopLeft(true) => (&TOP_LEFT_2X[..], size(6, 6)),
                TopRight(false) => (&TOP_RIGHT[..], size(3, 3)),
                TopRight(true) => (&TOP_RIGHT_2X[..], size(6, 6)),
                BottomLeft(false) => (&BOTTOM_LEFT[..], size(3, 3)),
                BottomLeft(true) => (&BOTTOM_LEFT_2X[..], size(6, 6)),
                BottomRight(false) => (&BOTTOM_RIGHT[..], size(3, 3)),
                BottomRight(true) => (&BOTTOM_RIGHT_2X[..], size(6, 6)),
            };

            GenericSurface::new(slice, size).unwrap()
        }

        pub fn transform(self, tr: Transform) -> Self {
            use Predefined::*;
            use Transform::*;

            match tr {
                UpScale { x: 1, y: 1 } => self,
                UpScale { x: 2, y: 2 } if self == TopLeft(false) => TopLeft(true),
                UpScale { x: 2, y: 2 } if self == TopRight(false) => TopRight(true),
                UpScale { x: 2, y: 2 } if self == BottomLeft(false) => BottomRight(true),
                UpScale { x: 2, y: 2 } if self == BottomRight(false) => BottomRight(true),
                UpScale { .. } => panic!("not supported for auto tests"),

                FlipHorizontal if matches!(self, TopLeft(_)) => TopRight(self.is_scaled()),
                FlipHorizontal if matches!(self, TopRight(_)) => TopLeft(self.is_scaled()),
                FlipHorizontal if matches!(self, BottomLeft(_)) => BottomRight(self.is_scaled()),
                FlipHorizontal if matches!(self, BottomRight(_)) => BottomLeft(self.is_scaled()),
                FlipHorizontal => unreachable!(),

                FlipVertical if matches!(self, TopLeft(_)) => BottomLeft(self.is_scaled()),
                FlipVertical if matches!(self, TopRight(_)) => BottomRight(self.is_scaled()),
                FlipVertical if matches!(self, BottomLeft(_)) => TopLeft(self.is_scaled()),
                FlipVertical if matches!(self, BottomRight(_)) => TopRight(self.is_scaled()),
                FlipVertical => unreachable!(),

                FlipBoth if matches!(self, TopLeft(_)) => BottomRight(self.is_scaled()),
                FlipBoth if matches!(self, TopRight(_)) => BottomLeft(self.is_scaled()),
                FlipBoth if matches!(self, BottomLeft(_)) => TopRight(self.is_scaled()),
                FlipBoth if matches!(self, BottomRight(_)) => TopLeft(self.is_scaled()),
                FlipBoth => unreachable!(),

                Rotate90Cw if matches!(self, TopLeft(_)) => TopRight(self.is_scaled()),
                Rotate90Cw if matches!(self, TopRight(_)) => BottomRight(self.is_scaled()),
                Rotate90Cw if matches!(self, BottomRight(_)) => BottomLeft(self.is_scaled()),
                Rotate90Cw if matches!(self, BottomLeft(_)) => TopLeft(self.is_scaled()),
                Rotate90Cw => unreachable!(),

                Rotate90Ccw if matches!(self, TopLeft(_)) => BottomLeft(self.is_scaled()),
                Rotate90Ccw if matches!(self, BottomLeft(_)) => BottomRight(self.is_scaled()),
                Rotate90Ccw if matches!(self, BottomRight(_)) => TopRight(self.is_scaled()),
                Rotate90Ccw if matches!(self, TopRight(_)) => TopLeft(self.is_scaled()),
                Rotate90Ccw => unreachable!(),

                Rotate180 if matches!(self, TopLeft(_)) => BottomRight(self.is_scaled()),
                Rotate180 if matches!(self, TopRight(_)) => BottomLeft(self.is_scaled()),
                Rotate180 if matches!(self, BottomLeft(_)) => TopRight(self.is_scaled()),
                Rotate180 if matches!(self, BottomRight(_)) => TopLeft(self.is_scaled()),
                Rotate180 => unreachable!(),
            }
        }
    }
}

#[test]
fn transforms() {
    let transforms = prop::collection::vec(
        prop::sample::select(
            &[
                Transform::FlipHorizontal,
                Transform::FlipVertical,
                Transform::FlipBoth,
                Transform::Rotate90Cw,
                Transform::Rotate90Ccw,
                Transform::Rotate180,
            ][..],
        ),
        0..=12,
    );

    let sources = prop::sample::select(
        &[
            Predefined::TopLeft(false),
            Predefined::TopRight(false),
            Predefined::BottomLeft(false),
            Predefined::BottomRight(false),
        ][..],
    );

    let mut runner = TestRunner::new(Config::with_cases(10_000));

    let result = runner.run(&(sources, transforms), |(src, transforms)| {
        let desired = transforms
            .iter()
            .copied()
            .fold(src, |src, tr| src.transform(tr))
            .surface();

        let mut dest_array = [0_u8; 9];
        let mut dest_array_scaled = [0_u8; 36];
        let mut dest;

        if desired.surface_size() == size(3, 3) {
            dest = GenericSurface::new(&mut dest_array[..], size(3, 3)).unwrap();
        } else {
            dest = GenericSurface::new(&mut dest_array_scaled[..], size(6, 6)).unwrap();
        }

        blit_whole(
            &mut dest,
            point(0, 0),
            &src.surface(),
            point(0, 0),
            &transforms,
        );

        prop_assert_eq!(&*dest, &*desired);

        Ok(())
    });

    match result {
        Ok(()) => {}
        Err(error) => {
            panic!("{error}");
        }
    }
}

#[test]
fn simple() {
    let mut dest = [0_u8; 25];

    let mut dest_buf = GenericSurface::new(&mut dest, size(5, 5)).unwrap();

    let src = [1_u8; 16];

    let src_buf = GenericSurface::new(&src, size(4, 4)).unwrap();

    blit(
        &mut dest_buf,
        point(1, 1),
        &src_buf,
        point(0, 0),
        size(3, 3),
        Default::default(),
    );

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
fn too_small() {
    let mut dest = [0_u8; 25];

    let mut dest_buf = GenericSurface::new(&mut dest, size(5, 5)).unwrap();

    let src = [1_u8; 16];

    let src_buf = GenericSurface::new(&src, size(4, 4)).unwrap();

    blit(
        &mut dest_buf,
        point(0, 0),
        &src_buf,
        point(0, 0),
        size(6, 6),
        Default::default(),
    );

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

#[test]
fn test_subsurface() {
    let mut dest = [0_u8; 25];
    
    let mut dest_buf = GenericSurface::new(&mut dest, size(5, 5)).unwrap().into_sub_surface(point(1, 1), size(2, 2));

    let src = [1_u8; 16];

    let src_buf = GenericSurface::new(&src, size(4, 4)).unwrap();

    blit_whole(
        &mut dest_buf,
        point(0, 0),
        &src_buf,
        point(0, 0),
        &[],
    );

    #[rustfmt::skip]
    let correct: [u8; 25] = [
        0, 0, 0, 0, 0,
        0, 1, 1, 0, 0,
        0, 1, 1, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
    ];

    assert_eq!(dest, correct);
}
