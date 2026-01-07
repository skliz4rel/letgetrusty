use std::fmt::Debug;
use std::fmt::Display;

fn main() {
    //call news Articule obj
    let newsart: NewsArticle = NewsArticle {
        author: String::from("Jide Akindejoye"),
        headline: String::from("Trump bombed sokoto state"),
        content: String::from("This is viable content not flex news"),
    };

    println!("Summary of the news article \n {}", newsart.summarize());

    //Call the Tweet obj to action
    let tweet: Tweet = Tweet {
        username: String::from("skliz4rel@gmail.com"),
        content: String::from("This is the content of the tweet"),
        reply: true,
        retweet: true,
    };

    println!("Summary of the Tweet \n {}", tweet.summarize());

    //passing the traits as an argument below
    notify(&tweet);
    notify(&newsart);
}

//This is like an abstract class in java. Can be implemented by multiple structs
pub trait Summary {
    fn summarize(&self) -> String {
        return String::from("Read More...");
    }
}

pub struct NewsArticle {
    pub author: String,
    pub headline: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        return format!("Summary from author {}", &self.author);
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        return format!("Summary from author {}", &self.username);
    }
}

//Passing traits as an argument
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

//passing obj that impl multiple traits

pub fn some_function<T, U>() -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    0
}
