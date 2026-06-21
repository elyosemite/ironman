/// There are three kinds of structs in Rust:
/// - Classic C-like structs (`Person`, `Point`, `Rectangle`)
/// - Tuple structs, basically named tuples (`Pair`)
/// - Unit structs, field-less, useful for generics (`Unit`)

#[derive(Debug, PartialEq)]
pub struct Person {
    pub name: String,
    pub age: u8,
}

pub struct Point {
    pub x: f32,
    pub y: f32,
}

pub struct Rectangle {
    pub top_left: Point,
    pub bottom_right: Point,
}

pub struct Unit;

pub struct Pair(pub i32, pub f32);

pub fn new_person(name: &str, age: u8) -> Person {
    Person { name: name.to_string(), age }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn field_init_shorthand_builds_the_struct() {
        let name = String::from("Yuri Melo");
        let age = 24;
        let yuri = Person { name, age };
        assert_eq!(yuri, new_person("Yuri Melo", 24));
    }

    #[test]
    fn derive_debug_formats_the_struct() {
        let yuri = new_person("Yuri Melo", 24);
        assert_eq!(format!("{:?}", yuri), "Person { name: \"Yuri Melo\", age: 24 }");
    }

    #[test]
    fn structs_can_nest_other_structs() {
        let rect = Rectangle {
            top_left: Point { x: 0.0, y: 1.0 },
            bottom_right: Point { x: 1.0, y: 0.0 },
        };
        assert_eq!(rect.top_left.y, 1.0);
        assert_eq!(rect.bottom_right.x, 1.0);
    }

    #[test]
    fn tuple_structs_are_accessed_positionally() {
        let pair = Pair(3, 1.5);
        assert_eq!(pair.0, 3);
        assert_eq!(pair.1, 1.5);
    }

    #[test]
    fn unit_structs_compile_with_no_fields() {
        let _ = Unit;
    }
}
