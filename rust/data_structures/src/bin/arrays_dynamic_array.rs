fn dynamic_array(n: i32, queries: &[Vec<i32>]) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    let mut arr: Vec<Vec<i32>> = vec![vec![]; n as usize];
    let mut last_answer = 0;
    for query in queries {
        let mut idx = 0;
        idx = (query[1] ^ last_answer) % n;
        if query[0] == 1 {
            arr[idx as usize].push(query[2]);
        } else if query[0] == 2 {
            last_answer = arr[idx as usize][(query[2] as usize % arr[idx as usize].len()) as usize];
            result.push(last_answer);
        }
    }
    result
}

fn main() {
    let n = 2;
    let qs: Vec<Vec<i32>> = [[1, 0, 5], [1, 1, 7], [1, 0, 3], [2, 1, 0], [2, 1, 1]]
        .iter()
        .map(|row| row.to_vec())
        .collect();
    println!("result array: {:?}", dynamic_array(n, &qs));
}
