fn quickSort(arr: &[i32]) -> Vec<i32> {
    let pivot = arr[0];
    let mut left = Vec::new();
    let mut right = Vec::new();
    for &el in arr {
        if el < pivot {
            left.push(el);
        } else {
            right.push(el);
        }
    }
    left.extend(right);
    left
}

fn main() {
    let _test = quickSort(&[4, 5, 3, 7, 2]);
}
