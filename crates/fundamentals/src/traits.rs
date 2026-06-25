struct Sheep { naked: bool, name: &'static str }

trait Animal {
    // Associated function signature; 'Self' refers to the implementor type
    fn new(name: &'static str) -> Self;

    // Method signatures; these will return a string
    fn name(&self) -> &'static str;
    fn noise(&self) -> &'static str;

    // Traits can provide default method definitions
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
            // Implementor methods can use the imeplementos, triat methods.
            println!("{} is already naked ...", self.name());
        } else {
            println!("{} gets a haricut!", self.name);
            self.naked = true;
        }
    }
}

impl Animal for Sheep {
    fn new(name: &'static str) -> Sheep {
        Sheep { name: name, naked: false }
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
