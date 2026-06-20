fn main() {
    println!("{} days", 31);
    println!("{} days", 31i64);

    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // As can named arguments
    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );

    println!(
        "{} of {:b} people know binary, the other half doesn't",
        1, 1498
    );

    // You can right-align text with a specified width. This will output
    // "     1". 5 white space and a "1".
    println!("{number:>width$}", number = 1, width = 5);

    // You can pad numbers with extra zeroes. This will output "000001"
    println!("{number:0>width$}", number = 1, width = 6);
}
