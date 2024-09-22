use state_objects::example_with_enums::Post as PostEnum;
use state_objects::example_with_state_parttern::Post;

fn main() {
    println!("Example with State Patern: ");

    let mut post_one = Post::new();

    post_one.add_text("Hi, this is my first post!");

    println!("Post content: {}", post_one.content());

    post_one.request_review();

    println!("Post content: {}", post_one.content());

    post_one.aprove();

    println!("Post content: {}", post_one.content());

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
