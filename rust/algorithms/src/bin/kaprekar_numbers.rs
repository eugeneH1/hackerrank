fn kaprekarNumbers(p: i32, q: i32) {
    let mut k_numbers: Vec<i32> = Vec::with_capacity(((q - p) / 2) as usize);
    for i in p..=q {
        let d = i.to_string().len();
        let square: i64 = (i as i64).pow(2);
        let square_string = square.to_string();
        let left_index = square_string.len() - d;
        let left_str = &square_string[left_index..];
        let right_str = &square_string[..left_index];
        let left = left_str.parse().unwrap_or(0);
        let right = right_str.parse().unwrap_or(0);
        if left + right == i {
            k_numbers.push(i);
        }
    }
    if k_numbers.is_empty() {
        print!("INVALID RANGE");
    } else {
        k_numbers.iter().for_each(|n| print!("{} ", n));
    }
}

fn kaprekar_numbers(p: i32, q: i32) {
    let mut k_numbers: Vec<i32> = Vec::with_capacity(((q - p) / 2) as usize);
    for i in p..=q {
        let d = i.ilog10() + 1;
        // let d = i.to_string().len();
        let base = (10 as i32).pow(d as u32);
        let square = (i as i64).pow(2);
        let left = square / base as i64;
        let right = square % base as i64;
        if left + right == i as i64 {
            k_numbers.push(i);
        }
    }
    if k_numbers.is_empty() {
        print!("INVALID RANGE");
    } else {
        k_numbers.iter().for_each(|n| print!("{} ", n));
    }
}

fn main() {
    kaprekar_numbers(1, 99999);
}
