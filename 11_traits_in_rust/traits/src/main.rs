#![allow(dead_code, unused_variables)]
use std::fmt::{Display, Debug};

pub struct NewsArticle {
    pub author: String,
    pub headline: String,
    pub content: String
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {}", self.headline, self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
pub trait Summary {
    fn summarize(&self) -> String{
        String::from("Read more...")
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

fn some_function<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
        U: Clone + Debug
{
    0
}
fn main() {
    let tweet = Tweet{
        username: String::from("@joaoareias"),
        content: String::from("Hello, world!"),
        reply: false,
        retweet: false
    };

    let article = NewsArticle {
        author: String::from("João Areias"),
        headline: String::from("Lorem Ipsum"),
        content: String::from("Lorem Ipsum dolor sit amet")
    };
    
    notify(&tweet);
    notify(&article);
}
