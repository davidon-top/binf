use crate::*;

#[bitflag]
pub struct Test {
    a: bool,
    b: bool,
    c: bool,
    d: bool,
    e: bool,
    f: bool,
    g: bool,
    h: bool,
}

#[test]
fn macro_test() {
    let mut test = Test::new(0b10101010);
    assert_eq!(test.a(), false);
    assert_eq!(test.b(), true);
    assert_eq!(test.c(), false);
    assert_eq!(test.d(), true);
    assert_eq!(test.e(), false);
    assert_eq!(test.f(), true);
    assert_eq!(test.g(), false);
    assert_eq!(test.h(), true);
}

#[test]
fn macro_test_set() {
    let mut test = Test::new(0b10101010);
    test.set_a(true);
    test.set_b(false);
    test.set_c(true);
    test.set_d(false);
    test.set_e(true);
    test.set_f(false);
    test.set_g(true);
    test.set_h(false);
    assert_eq!(test.value(), 0b01010101);
}

#[test]
fn macro_test_iter() {
    let test = Test::new(0b10101010);
    let flags = test.flags();
    let mut iter = flags.iter();
    assert_eq!(iter.next(), Some(false).as_ref());
    assert_eq!(iter.next(), Some(true).as_ref());
    assert_eq!(iter.next(), Some(false).as_ref());
    assert_eq!(iter.next(), Some(true).as_ref());
    assert_eq!(iter.next(), Some(false).as_ref());
    assert_eq!(iter.next(), Some(true).as_ref());
    assert_eq!(iter.next(), Some(false).as_ref());
    assert_eq!(iter.next(), Some(true).as_ref());
    assert_eq!(iter.next(), None);
}

#[test]
fn u64_test() {
    let mut flags = 0b10101010u64;
    assert_eq!(flags.get_flag(0), false);
    assert_eq!(flags.get_flag(1), true);
    assert_eq!(flags.get_flag(2), false);
    assert_eq!(flags.get_flag(3), true);
    assert_eq!(flags.get_flag(4), false);
    assert_eq!(flags.get_flag(5), true);
    assert_eq!(flags.get_flag(6), false);
    assert_eq!(flags.get_flag(7), true);
    flags.set_flag(0, true);
    assert_eq!(flags, 0b10101011);
}
