use std::collections::HashMap;

fn missingNumbers(arr: &[i32], brr: &[i32]) -> Vec<i32> {
    let mut b_freq_map: HashMap<i32, i32> = HashMap::new();
    let mut a_freq_map: HashMap<i32, i32> = HashMap::new();
    let mut result: Vec<i32> = Vec::new();

    for &x in brr {
        *b_freq_map.entry(x).or_insert(0) += 1;
    }

    for &x in arr {
        *a_freq_map.entry(x).or_insert(0) += 1;
    }

    result
}

fn missing_numbers(arr: &[i32], brr: &[i32]) -> Vec<i32> {
    let mut arr_vec = arr.to_vec();
    let mut brr_vec = brr.to_vec();

    arr_vec.sort();
    brr_vec.sort();

    let mut i = 0;
    let mut j = 0;

    while i < arr_vec.len() && j < brr_vec.len() {
        if arr_vec[i] == brr_vec[j] {
            brr_vec.remove(j);
            i += 1;
        } else {
            if brr_vec[j] < arr_vec[i] {
                j += 1;
            } else if brr_vec[j] > arr_vec[i] {
                i += 1;
            } else {
                i += 1;
                j += 1;
            }
        }
    }

    // brr_vec.iter().for_each(|f| print!("{}", f));
    brr_vec.dedup();
    brr_vec
}

fn main() {
    let arr = vec![203, 204, 205, 206, 207, 208, 203, 204, 205, 206];
    let brr = vec![
        203, 204, 204, 205, 206, 207, 205, 208, 203, 206, 205, 206, 204,
    ];
    missing_numbers(&arr, &brr);
}
