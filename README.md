# simple-blit

Provides very simple blitting.

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
    // construct a buffer which holds width and height
    &mut GenericBuffer::new(&mut dest, 5, 5).unwrap(),
    // where to blit
    (1, 1),
    &GenericBuffer::new(&src, 4, 4).unwrap(),
    // where to blit from
    (0, 0),
    // size of the area
    (3, 3),
    // no flips or anything
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

## Features

* `pixels-integration` (off by default): implements `Buffer` and `BufferMut` for [`Pixels`](https://docs.rs/pixels/0.13.0/pixels/struct.Pixels.html).
* `serde` (off by default): implements `Serialize` and `Deserialize` for `GenericBuffer` and `BlitOptions`.
