pub trait Animal {
    fn do_something(&self);
}

struct Dog;

impl Animal for Dog {
    fn do_something(&self) {
        println!("Woof");
    }
}

fn make_a_sound(animal: &impl Animal) {
    animal.do_something();
}

fn make_a_second_sound<T: Animal>(animal: &T) {
    animal.do_something();
}

// another example
pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

impl Summary for SocialPost {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub fn show_social_post() {
    let post = SocialPost {
        username: String::from("horse_books"),
        content: String::from(
            "of course, ..."
        ),
        reply: false,
        repost: false,
    };
    println!("1 new post: {}", post.summarize());
}
