use std::cmp;

fn hourglass_sum(arr: &[Vec<i32>]) -> i32 {
    let shape: [[usize; 2]; 7] = [[0, 0], [0, 1], [0, 2], [1, 1], [2, 0], [2, 1], [2, 2]];
    let limit = 4;
    let mut max = i32::MIN;

    for i in 0..limit {
        for j in 0..limit {
            let mut current = 0;
            for hourglass in shape {
                current += arr[i + hourglass[0]][j + hourglass[1]];
            }
            max = std::cmp::max(current, max);
        }
    }
    max
}

fn main() {
    let matrix: Vec<Vec<i32>> = vec![
        vec![-9, -9, -9, 1, 1, 1],
        vec![0, -9, 0, 4, 3, 2],
        vec![-9, -9, -9, 1, 2, 3],
        vec![0, 0, 8, 6, 6, 0],
        vec![0, 0, 0, -2, 0, 0],
        vec![0, 0, 1, 2, 4, 0],
    ];
    print!("{}", hourglass_sum(&matrix));
}
