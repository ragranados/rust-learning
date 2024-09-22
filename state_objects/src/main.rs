use state_objects::example_with_state_parttern::Post;


fn main() {
    let mut post_one = Post::new();

    post_one.add_text("Hi, this is my first post!");

    println!("Post content: {}", post_one.content());

    post_one.request_review();

    println!("Post content: {}", post_one.content());

    post_one.aprove();

    println!("Post content: {}", post_one.content());
}
