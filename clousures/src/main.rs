fn main() {
    let x = 4;

    let equal_to_x = |z: i32| z == x;

    let y = 4;

    assert!(equal_to_x(y));
}
