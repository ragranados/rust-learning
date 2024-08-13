fn main() {
    let v1 = vec![1, 2, 3, 4, 5];
    let mut v1_iter = v1.iter();

    //for loop takes ownership of the iterator
    for i in v1_iter {
        println!("{i}");
    }

    // this code will also works
    // for i in v1.iter() {
    //     println!("{i}");
    // }

    let mut v2 = vec![1, 2, 3, 4, 5];

    for i in v2.iter_mut() {
        *i += 1;
    }

    println!("{:?}", v2);

    //above code, but with iterator adaptors

    let v1: Vec<_> = v1.iter().map(|x| x + 1).collect();
    println!("{:?}", v1);
}

#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }
}
