fn marcsCakewalk(calorie: &[i32]) -> i64 {
    let mut sorted = calorie.to_vec();
    sorted.sort_by_key(|&x| std::cmp::Reverse(x));
    sorted
        .iter()
        .enumerate()
        .map(|(i, &c)| (1_i64 << i) * c as i64)
        .sum()
}

fn main() {
    marcsCakewalk(&[1, 3, 2]);
}
