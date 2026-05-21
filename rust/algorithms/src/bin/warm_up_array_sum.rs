fn array_sum(arr: &[i32]) -> i32 {
    let mut sum = 0;
    for val in arr {
        sum += *val;
    }
    sum
}

fn array_sum_func(ar: &[i32]) -> i32 {
    ar.iter().sum()
}

fn main() {
    let a = [3, 1, 2];
    let result = array_sum(&a);
    println!("{}", result);
}
