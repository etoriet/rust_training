use section_trait::{Tweet, Summary};

fn main() {

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        // も ち ろ ん 、ご 存 知 か も し れ な い よ う に ね 、み な さ ん
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("{}", tweet.summarize());
}
