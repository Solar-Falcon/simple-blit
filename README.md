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
    &mut GenericSurface::new(&mut dest, size(5, 5)).unwrap(),
    // where to blit
    point(1, 1),
    &GenericSurface::new(&src, size(4, 4)).unwrap(),
    // where to blit from
    point(0, 0),
    // size of the area
    size(3, 3),
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
