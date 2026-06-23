pub fn my_own_tuple() -> (i32, f64, char) {
    (42, 3.14, 'a')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tuples_can_be_destructured() {
        let (x, y, z) = my_own_tuple();
        assert_eq!((x, y, z), (42, 3.14, 'a'));
    }
}
