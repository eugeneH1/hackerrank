fn difference(arr: &[Vec<i32>]) -> i32 {
    let mut a = 0;
    let mut b = 0;
    for (x, row) in arr.iter().enumerate() {
        for (y, value) in row.iter().enumerate() {
            if x == y {
                a += value;
            }
            if x + y == arr.len() - 1 {
                b += value;
            }
        }
    }
    (a - b).abs()
}

fn main() {
    let matrix = vec![vec![11, 2, 4], vec![4, 5, 6], vec![10, 8, -12]];
    println!("{}", difference(&matrix));
}
