use crate::*;

#[test]
fn simple() {
    let mut dest = [0_u8; 25];

    let mut dest_buf = GenericBuffer::new(&mut dest, 5, 5).unwrap();

    let src = [1_u8; 16];

    let src_buf = GenericBuffer::new(&src, 4, 4).unwrap();

    blit(
        &mut dest_buf,
        (1, 1),
        &src_buf,
        (0, 0),
        (3, 3),
        BlitOptions::None,
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
fn flip() {
    let mut dest = [0_u8; 25];

    let mut dest_buf = GenericBuffer::new(&mut dest, 5, 5).unwrap();

    #[rustfmt::skip]
    let src: [u8; 9] = [
        1, 2, 3,
        1, 2, 3,
        1, 2, 3,
    ];

    let src_buf = GenericBuffer::new(&src, 3, 3).unwrap();

    blit_full(&mut dest_buf, (1, 1), &src_buf, BlitOptions::FlipHorizontal);

    #[rustfmt::skip]
    let correct: [u8; 25] = [
        0, 0, 0, 0, 0,
        0, 3, 2, 1, 0,
        0, 3, 2, 1, 0,
        0, 3, 2, 1, 0,
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

    blit(
        &mut dest_buf,
        (-1, -1),
        &src_buf,
        (0, 0),
        (4, 4),
        BlitOptions::None,
    );

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

    blit(
        &mut dest_buf,
        (-1, -1),
        &src_buf,
        (-1, -1),
        (6, 6),
        BlitOptions::None,
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
