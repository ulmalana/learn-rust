use ch10_trait::Tweet;
use ch10_trait::Summary;
use ch10_trait::NewsArticle;


fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Penguins have once again won the championship againts the bears."),
    };

    println!("New article available: {}", article.summarize());

    //println!("Hello, world!");
}
