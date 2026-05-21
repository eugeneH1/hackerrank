use std::cmp::Ordering;

fn plus_minus(arr: Vec<i32>) {
    let n = arr.len() as f64;
    let mut pos = 0;
    let mut neg = 0;
    let mut zero = 0;

    for num in arr {
        match num.cmp(&0) {
            Ordering::Greater => pos += 1,
            Ordering::Less => neg += 1,
            Ordering::Equal => zero += 1,
        }
    }
    println!("{:.6}", pos as f64 / n);
    println!("{:.6}", neg as f64 / n);
    println!("{:.6}", zero as f64 / n);
}

fn main() {
    plus_minus([1, 1, 0, -1, -1].to_vec());
}
