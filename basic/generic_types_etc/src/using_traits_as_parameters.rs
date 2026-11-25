#[allow(dead_code)]
pub trait Summary {
    fn summarize(&self) -> String;
}

#[allow(dead_code)]
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

#[allow(dead_code)]
pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

impl Summary for SocialPost {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

#[allow(dead_code)]
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
