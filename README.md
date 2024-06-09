# simple-blit

Provides simple blitting from one surface to another with some possible transformations.

## Example

```rust
use simple_blit::*;

let mut dest: [u8; 25] = [
    0, 0, 0, 0, 0,
    0, 0, 0, 0, 0,
    0, 0, 0, 0, 0,
    0, 0, 0, 0, 0,
    0, 0, 0, 0, 0,
];

let src: [u8; 16] = [
    1, 1, 1, 1,
    1, 1, 1, 1,
    1, 1, 1, 1,
    1, 1, 1, 1,
];

blit(
    // construct a surface which holds width and height
    GenericSurface::new(&mut dest, size(5, 5))
        .unwrap()
        // offset on the destination
        .offset_surface_mut(point(1, 1)),
    // you can borrow the surface if you don't want to drop it
    // (the destination has to be borrowed mutably of course)
    &GenericSurface::new(&src, size(4, 4))
        .unwrap()
        .sub_surface(
            point(0, 0), // source offset
            size(3, 3)   // size of the area to copy
        ),
    // no transformations
    Default::default(),
);

assert_eq!(dest, [
    0, 0, 0, 0, 0,
    0, 1, 1, 1, 0,
    0, 1, 1, 1, 0,
    0, 1, 1, 1, 0,
    0, 0, 0, 0, 0,
]);
```

## Cargo features

* `pixels-integration` (off by default): implements `Surface` and `SurfaceMut` for [`Pixels`](https://docs.rs/pixels/0.13.0/pixels/struct.Pixels.html).
* `image-integration` (off by default): implements `Surface` and `SurfaceMut` for [`ImageSurface`](https://docs.rs/image/latest/image/struct.ImageSurface.html)
* `serde` (off by default): implements `Serialize` and `Deserialize` for surface types and `Transform`.

## License

As of version 1.0.0, this crate's license has been changed from MIT to MIT-0 (aka MIT No Attribution).
