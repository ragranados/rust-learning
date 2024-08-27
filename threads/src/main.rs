use std::thread;

fn main() {
    // main_example();
    move_example()
}

fn main_example() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(std::time::Duration::from_millis(1));
        }
    });

    // handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(std::time::Duration::from_millis(1));
    }

    //results a result, so unwrap is used to get the result.
    //This will block the main thread until the spawned thread finishes.
    // handle.join().unwrap();
}

fn move_example() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {v:?}");
    });

    handle.join().unwrap();
}
