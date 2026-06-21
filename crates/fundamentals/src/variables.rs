pub const SCORE_LIMIT: u32 = 100;

pub fn shadow_increment(x: i32) -> i32 {
    let x = x + 1;
    x
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shadowing_rebinds_the_name_to_a_new_value() {
        assert_eq!(shadow_increment(5), 6);
    }

    #[test]
    fn mut_bindings_can_be_reassigned() {
        let mut age = 3;
        assert_eq!(age, 3);

        age = 90;
        assert_eq!(age, 90);
    }

    #[test]
    fn constants_are_known_at_compile_time() {
        assert_eq!(SCORE_LIMIT, 100);
    }
}
