pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

pub struct NewsPaper {
    pub heading: String,
    pub content: String,
    pub author: String,
}

pub trait Summary {
    fn summarize(&self) -> String;
}

//? You can include a trait like
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}", self.username)
    }
}

impl Summary for NewsPaper {
    fn summarize(&self) -> String {
        format!("{}", self.heading)
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("KAKAAKAKAKA"),
        content: String::from(""),
        reply: true,
        retweet: false,
    };

    let news = NewsPaper {
        author: String::from("Blah kekeke"),
        content: String::from("Lorem ipsum"),
        heading: String::from("The wild pareadox"),
    };

    println!("{:?}", news.summarize());
    print!("{:?}", tweet.summarize());
}
