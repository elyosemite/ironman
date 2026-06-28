struct Sheep { naked: bool, name: &'static str }

trait Animal {
    fn new(name: &'static str) -> Self;
    fn name(&self) -> &'static str;
    fn noise(&self) -> &'static str;

    fn talk(&self) {
        println!("{} says {}", self.name(), self.noise());
    }
}

impl Sheep {
    fn is_naked(&self) -> bool {
        self.naked
    }

    fn shear(&mut self) {
        if self.is_naked() {
            println!("{} is already naked ...", self.name());
        } else {
            println!("{} gets a haircut!", self.name);
            self.naked = true;
        }
    }
}

impl Animal for Sheep {
    fn new(name: &'static str) -> Sheep {
        Sheep { name, naked: false }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn noise(&self) -> &'static str {
        if self.is_naked() {
            "I am naked, bro"
        } else {
            "I am not naked, bro"
        }
    }

    fn talk(&self) {
        println!("{} pauses briefly ...{}", self.name, self.noise())
    }
}

// --- impl Trait / generic trait bounds ---

trait Sound {
    fn sound(&self) -> &'static str;
}

struct Dog;

impl Sound for Dog {
    fn sound(&self) -> &'static str {
        "Woof"
    }
}

fn make_a_sound(animal: &impl Sound) -> &'static str {
    animal.sound()
}

fn make_a_second_sound<T: Sound>(animal: &T) -> &'static str {
    animal.sound()
}

// --- shared trait across multiple types ---

trait Summary {
    fn summarize(&self) -> String;
}

struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

struct SocialPost {
    username: String,
    content: String,
    reply: bool,
    repost: bool,
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
