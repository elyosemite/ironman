/* There are three types of structures ("structs") that can be created using the struct keyword:
 - Tuple structs, which are, basically, named tuples.
 - The classic C structs
 - Unit structs, which are field-less, are useful for generics.
*/

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// A unit struct
struct Unit;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another fields
#[allow(dead_code)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn main() {
    // Create struct with fields init shortland
    let name = String::from("Yuri Melo");
    let age = 24;
    let yuri = Person { name, age };

    // Print Debug struct
    println!("{:?}", yuri);
}
