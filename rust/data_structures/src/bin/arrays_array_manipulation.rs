fn array_manipulation(n: i32, queries: &[Vec<i32>]) -> i64 {
    let mut height_map: Vec<i64> = vec![0; (n + 2) as usize];
    for q in queries {
        let (a, b, k) = (q[0], q[1], q[2]);
        height_map[a as usize] += k as i64;
        height_map[(b + 1) as usize] -= k as i64;
    }
    height_map
        .iter()
        .fold((0, 0), |(current, max), &h| {
            let next_sum = current + h;
            (next_sum, max.max(next_sum))
        })
        .1
}

fn main() {}
