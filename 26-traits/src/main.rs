use traits::*;

fn main() {
    let article = NewsArticle {
        headline: String::from("random text"),
        author: String::from("random text"),
        location: String::from("random text"),
        ..Default::default()
    };
    let post = SocialPost {
        username: String::from("random text"),
        content: String::from("random text"),
        repost: false,
        reply: None,
    };
    println!("{}", article.summarize());
    println!("{}", post.summarize2());

    notify2(&article);
    println!("{}", article.headline);
    notify(&post);
}
