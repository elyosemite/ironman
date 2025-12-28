fn main() {
    // This is Immutable by default
    let some_person_age = 35;

    // This variable is mutable
    let mut my_son_age = 3;
    println!("{0}", my_son_age);
    my_son_age = 90;

    // Declare a constant
    const SCORE_LIMIT: u32 = 100;

    // Shadowing
    let x = 5;
    let x = x + 1; // x is now 6

    println!(
        "{0}, {1}, {2}, {3}",
        some_person_age, my_son_age, SCORE_LIMIT, x
    );

    eprintln!("This is an error!!");
}
