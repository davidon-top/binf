use binf::BitFlags;

#[test]
fn standalone() {
    let flags = BitFlags::<u8>::new(0b10101010);
    assert_eq!(*flags, 0b10101010);
}
