fn insertionSort1(n: i32, arr: &[i32]) {
    let mut result = arr.to_vec();
    let element = result[arr.len() - 1];

    let mut insert_at = 0;
    for i in (0..arr.len() - 1).rev() {
        if arr[i] > element {
            result[i + 1] = arr[i];
        } else {
            insert_at = i + 1;
            break;
        }
        result.iter().for_each(|f| print!("{} ", f));
        print!("\n");
    }
    result[insert_at] = element;
    result.iter().for_each(|f| print!("{} ", f));
}

fn main() {
    // 5
    // let arr = vec![1, 3, 5, 9, 13, 22, 27, 35, 46, 51, 55, 83, 87, 23];
    // let arr = vec![3, 5, 9, 13, 22, 27, 35, 46, 51, 55, 83, 87, 23, 1];
    let arr = vec![2, 3, 4, 5, 6, 7, 8, 9, 10, 1];
    insertionSort1(5, &arr);
}
