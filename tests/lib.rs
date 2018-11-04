#![feature(generators)]

#[macro_use]
extern crate iter_compr;

#[test]
fn simple() {
    let mut iter = iter_compr![x; x in 0..3];
    assert_eq!(iter.next(), Some(0));
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), None);
}

#[test]
fn simple2() {
    let mut iter = iter_compr![(x, y); x in 0..3, y in 5..7];
    assert_eq!(iter.next(), Some((0, 5)));
    assert_eq!(iter.next(), Some((0, 6)));
    assert_eq!(iter.next(), Some((1, 5)));
    assert_eq!(iter.next(), Some((1, 6)));
    assert_eq!(iter.next(), Some((2, 5)));
    assert_eq!(iter.next(), Some((2, 6)));
    assert_eq!(iter.next(), None);
}

#[test]
fn complex() {
    let mut iter = iter_compr![
        (x, y, z);
        x in 0..6,
        y in 1..x,
        x % y == 0,
        let z = x / y,
        _ in 0..2
    ];
    assert_eq!(iter.next(), Some((2, 1, 2)));
    assert_eq!(iter.next(), Some((2, 1, 2)));
    assert_eq!(iter.next(), Some((3, 1, 3)));
    assert_eq!(iter.next(), Some((3, 1, 3)));
    assert_eq!(iter.next(), Some((4, 1, 4)));
    assert_eq!(iter.next(), Some((4, 1, 4)));
    assert_eq!(iter.next(), Some((4, 2, 2)));
    assert_eq!(iter.next(), Some((4, 2, 2)));
    assert_eq!(iter.next(), Some((5, 1, 5)));
    assert_eq!(iter.next(), Some((5, 1, 5)));
    assert_eq!(iter.next(), None);
}