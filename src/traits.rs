trait Animal {
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