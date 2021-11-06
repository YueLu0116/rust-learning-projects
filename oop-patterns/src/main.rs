use oop_patterns::Post;

fn main() {
    let mut post = Post::new();
    post.add_text("Github Copilot is amazing!");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("Github Copilot is amazing!", post.content());

    println!("{}", post.content());
}
