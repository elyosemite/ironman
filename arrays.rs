use std::mem;

fn analyze_slice(slice: &[i32]) {
    println!("First Element  of the slice: {}", slice[0]);
    println!("The slice has {} Elements", slice.len());
}

fn analyze_slice_two(slice: &[i64]) {
    println!("First element of the slice: {}", slice[0]);
    println!("Second element of the slice: {}", slice[1]);
    println!("The slice has {} elements", slice.len());
}

fn main() {
    // Fixed-size array
    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    let zs: [i64; 7] = [7, 6, 5, 4, 3, 2, 1];

    let ys: [i64; 500] = [0; 500];

    println!("First Element of the array: {}", xs[0]);
    println!("second Element of the array: {}", xs[1]);
    println!("number of Elements in array: {}", xs.len());
    println!("array occupies {} bytes", mem::size_of_val(&xs));

    // Arrays can be automatically borrowed as slices
    println!("borrow the whole array as a slice");
    analyze_slice(&xs);
    analyze_slice_two(&ys);
    analyze_slice_two(&zs);
    analyze_slice_two(&zs[0..5]);
}
