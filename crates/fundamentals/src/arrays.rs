use std::mem;

pub fn first_and_len(xs: &[i32]) -> (i32, usize) {
    (xs[0], xs.len())
}

pub fn array_size_in_bytes(xs: &[i32; 5]) -> usize {
    mem::size_of_val(xs)
}

pub fn first_tuple(pairs: &[(i32, i32); 3]) -> (i32, i32) {
    pairs[0]
}

pub fn second_of_second_tuple(pairs: &[(i32, i32); 3]) -> i32 {
    pairs[1].1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slices_expose_their_first_element_and_length() {
        let xs: [i32; 5] = [1, 2, 3, 4, 5];
        assert_eq!(first_and_len(&xs), (1, 5));
        assert_eq!(xs, [1, 2, 3, 4, 5])
    }

    #[test]
    fn fixed_size_arrays_report_their_byte_size() {
        let xs: [i32; 5] = [1, 2, 3, 4, 5];
        assert_eq!(array_size_in_bytes(&xs), 20);
    }

    #[test]
    fn arrays_of_tuples_are_indexable() {
        let array_of_tuples: [(i32, i32); 3] = [(1, 2), (3, 4), (5, 6)];
        assert_eq!(first_tuple(&array_of_tuples), (1, 2));
        assert_eq!(second_of_second_tuple(&array_of_tuples), 4);
    }
}
