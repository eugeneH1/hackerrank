fn reverse_array(arr: &[i32]) -> Vec<i32> {
    let mut result = arr.to_vec();
    result.reverse();
    result
}

fn main() {
    reverse_array(&[1, 2, 3]);
}
