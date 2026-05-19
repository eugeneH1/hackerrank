fn min_max(arr: &[i32]) {
    let total_sum: i64 = arr.iter().map(|&x| x as i64).sum();
    let mut min = i64::MAX;
    let mut max = i64::MIN;

    for x in arr.iter().map(|&n| n as i64) {
        if x < min {
            min = x;
        }
        if x > max {
            max = x;
        }
    }
    (min, max) = (total_sum - max, total_sum - min);
    println!("{} {}", min, max);
}

fn main() {
    min_max(&[1, 2, 3, 4, 5]);
}
