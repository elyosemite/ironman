pub trait Animal {
    fn do_something(&self) -> String;
}

pub struct Dog;

impl Animal for Dog {
    fn do_something(&self) -> String {
        "Woof".to_string()
    }
}

pub fn make_a_sound(animal: &impl Animal) -> String {
    animal.do_something()
}

pub fn make_a_second_sound<T: Animal>(animal: &T) -> String {
    animal.do_something()
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn impl_trait_param_dispatches_to_the_impl() {
        assert_eq!(make_a_sound(&Dog), "Woof");
    }

    #[test]
    fn generic_trait_bound_dispatches_to_the_impl() {
        assert_eq!(make_a_second_sound(&Dog), "Woof");
    }

    #[test]
    fn different_types_can_share_a_trait() {
        let article = NewsArticle {
            headline: String::from("Penguins Win"),
            location: String::from("Pittsburgh"),
            author: String::from("Iceburgh"),
            content: String::from("..."),
        };
        let post = SocialPost {
            username: String::from("horse_books"),
            content: String::from("of course, ..."),
            reply: false,
            repost: false,
        };

        assert_eq!(article.summarize(), "Penguins Win, by Iceburgh (Pittsburgh)");
        assert_eq!(post.summarize(), "horse_books: of course, ...");
    }
}
