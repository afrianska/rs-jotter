/*
#[allow(dead_code)]
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}
*/
#[allow(dead_code)]
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

/*
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}
*/

/*
impl Summary for NewsArticle {
    // Try defaul implementation sto summary with remove this impl
    /*
    fn summarize(&self) -> String {
        format!(
            "{}, by {} ({}), {}",
            self.headline, self.author, self.location, self.content
        )
    }
    */
}
*/

#[allow(dead_code)]
pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

impl Summary for SocialPost {
    /*fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
    */

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}
