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

mod test {
    use super::{Tweet, NewsArticle, Summary};

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
}

// TODO rustfmtを使えるようにすると幸せ
