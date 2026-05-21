fn hourglass_sum(arr: &[Vec<i32>]) -> i32 {
    let shape: [[i32; 2]; 7] = [[0, 0], [0, 1], [0, 2], [1, 1], [2, 0], [2, 1], [2, 2]];
    let limit = arr.len() - 3 + 1;

    for i in 0..=limit {
        println!("{}", i);
    }
    0
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
    hourglass_sum(&matrix);
}
