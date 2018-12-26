pub trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        // ( も っ と 読 む)
        format!("(Read more from {})", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        self.author.clone()
    }
    //fn summarize(&self) -> String {
    //    format!("{}, by {} ({})", self.headline, self.author, self.location)
    //}
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.summarize_author(), self.content)
    }
}

pub fn notify<T: Summary>(item: T) {
    println!("News: {}", item.summarize());
}

use std::fmt::Display;
struct Pair<T> {
    x: T,
    y: T,
}
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {
            x,
            y,
        }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) -> String {
        if self.x >= self.y {
            format!("The largest member is x = {}", self.x)
        } else {
            format!("The largest member is y = {}", self.y)
        }
    }
}


mod test {
    use super::*;

    #[test]
    fn test() {
        let tweet = Tweet {
            username: String::from("horse_ebooks"),
            // も ち ろ ん 、ご 存 知 か も し れ な い よ う に ね 、み な さ ん
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        };
        assert_eq!(tweet.summarize(), String::from("@horse_ebooks: of course, as you probably already know, people"));

        let news_article = NewsArticle {
            headline: String::from("news desu"),
            location: String::from("Japan"),
            author: String::from("torii"),
            content: String::from("news desu"),
        };

        assert_eq!(news_article.summarize(), String::from("(Read more from torii)"));
    }

    #[test]
    fn test_cmp_display() {
        let pair = Pair {
            x: 1.0,
            y: 2.0,
        };
        let ret = pair.cmp_display();
        assert_eq!(ret, String::from("The largest member is y = 2"));
    }
}

// TODO rustfmtを使えるようにすると幸せ
