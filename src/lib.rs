fn lib() {
    let mut a = 123;
    let mut b = 456;
    // clippy error: almost_swapped
    a = b;
    b = a;
    a
}
