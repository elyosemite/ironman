pub fn my_own_tuple() -> (i32, f64, char) {
    (42, 3.14, 'a')
}

fn main() {
    let (x, y, z) = my_own_tuple();
    println!("Tuple: ({x}, {y}, {z})");
}
