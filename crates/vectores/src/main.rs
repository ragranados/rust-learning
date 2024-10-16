fn main() {
    let v: Vec<i32> = Vec::new();

    // v.push(1);

    let mut v = vec![1, 2, 3, 4, 5, 6, 7];

    v.push(3);

    let first = &v[1];

    for item in &v {
        println!("{item}");
    }

    let first = &v[1];
    
}
