pub trait Summary{
    fn summarize(&self) -> String;

    fn from(&self) -> String{
        String::from("from unknown...")
    }
}

pub struct NewsArticle{
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle{
    fn summarize(&self) -> String{
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }

    fn from(&self) -> String{
        format!("{}", self.author)
    }
}

pub struct Tweet{
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

impl Tweet{
    pub fn tweet(from: &str, content: &str, reply: bool, retweet:bool) -> Self{
        if content.len() < 280{
            return Self{
                username: String::from(from),
                content: String::from(content),
                reply: reply,
                retweet:retweet,
            }
        }
        panic!("Tweets need to be under 280 characters");
    }
}

impl Summary for Tweet{
    fn summarize(&self) -> String{
        format!("{}: {}", self.username, self.content)
    }
}
