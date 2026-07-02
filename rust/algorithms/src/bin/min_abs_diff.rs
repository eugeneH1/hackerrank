use std::cmp::min;

fn minimumAbsoluteDifference(arr: &[i32]) -> i32 {
    let mut result = i32::MAX;
    for i in 0..arr.len() {
        for j in (i + 1)..arr.len() {
            let _val = (arr[i] - arr[j]).abs();
            result = min(result, (arr[i] - arr[j]).abs())
        }
    }
    result
}

fn minimum_absolute_difference(arr: &[i32]) -> i32 {
    (0..arr.len()) // [0, 1, 2]
        .flat_map(|i| ((i + 1)..arr.len()).map(move |j| (i, j)))
        .map(|(i, j)| (arr[i] - arr[j]).abs())
        .min()
        .unwrap()
}

fn min_abs_dif(arr: &[i32]) -> i32 {
    let mut sorted = arr.to_vec();
    sorted.sort();
    sorted
        .windows(2)
        .map(|win| (win[0] - win[1]).abs())
        .min()
        .unwrap()
}

fn main() {
    minimumAbsoluteDifference(&[3, -7, 0]);
    print!("{}", (-7 as i32 - 0 as i32).abs());
}
