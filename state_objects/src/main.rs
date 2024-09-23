use state_objects::changes_to_fit_rust::Post as PostRust;
use state_objects::example_with_enums::Post as PostEnum;
use state_objects::example_with_state_parttern::Post;

fn main() {
    // example_with_state_parttern();
    // example_with_enums();
    changes_to_fit_rust();
}

fn example_with_state_parttern() {
    println!("Example with State Patern: ");

    let mut post_one = Post::new();

    post_one.add_text("Hi, this is my first post!");

    println!("Post content: {}", post_one.content());

    post_one.request_review();

    println!("Post content: {}", post_one.content());
    post_one.print_state();

    post_one.reject();
    post_one.add_text(" Added?");

    println!("Post content: {}", post_one.content());
    post_one.print_state();

    post_one.request_review();

    println!("Post content: {}", post_one.content());

    post_one.aprove();
    post_one.add_text("Not Added?");

    println!("Post content: {}", post_one.content());

    post_one.aprove();

    println!("Post content: {}", post_one.content());
    post_one.print_state();
}

fn example_with_enums() {
    //Second example
    println!("Example with Enums: ");

    let mut post_two = PostEnum::new();

    post_two.add_text("Segundo post!");

    println!("Post content: {}", post_two.content());

    post_two.request_review();

    println!("Post content: {}", post_two.content());

    post_two.aprove();

    println!("Post content: {}", post_two.content());

    post_two.request_review();

    // println!("Post content: {}", post_two.content());
}

fn changes_to_fit_rust() {
    let mut post = PostRust::new();

    post.add_text("I ate a salad for lunch today");

    let post = post.request_review();

    let post = post.reject();

    let post = post.request_review();

    let post = post.request_review();

    let post = post.aprove();

    println!("{}", post.content());
}
