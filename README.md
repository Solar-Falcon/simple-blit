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
    (3, 3)
);

assert_eq!(dest, [
    0, 0, 0, 0, 0,
    0, 1, 1, 1, 0,
    0, 1, 1, 1, 0,
    0, 1, 1, 1, 0,
    0, 0, 0, 0, 0,
]);
```
