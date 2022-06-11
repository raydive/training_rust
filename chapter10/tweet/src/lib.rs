pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

/*
$ cargo run                                                                                                                                                              [git][main] -? 
   Compiling tweet v0.1.0 (/Users/arata_n/dev/training_rust/chapter10/tweet)
warning: unused import: `Tweet`
 --> src/main.rs:1:41
  |
1 | use tweet::{self, NewsArticle, Summary, Tweet};
  |                                         ^^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

warning: `tweet` (bin "tweet") generated 1 warning
    Finished dev [unoptimized + debuginfo] target(s) in 0.14s
     Running `target/debug/tweet`
New article available! Penguins win the Stanley Cup Championship!, by Iceburgh (Pittsburgh, PA, USA)

$ cargo run                                                                                                                                                              [git][main] -? 
   Compiling tweet v0.1.0 (/Users/arata_n/dev/training_rust/chapter10/tweet)
warning: unused import: `Tweet`
 --> src/main.rs:1:41
  |
1 | use tweet::{self, NewsArticle, Summary, Tweet};
  |                                         ^^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

warning: `tweet` (bin "tweet") generated 1 warning
    Finished dev [unoptimized + debuginfo] target(s) in 0.22s
     Running `target/debug/tweet`
New article available! (Read more...)
*/
impl Summary for NewsArticle {
    // fn summarize(&self) -> String {
    //     format!("{}, by {} ({})", self.headline, self.author, self.location)
    // }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
