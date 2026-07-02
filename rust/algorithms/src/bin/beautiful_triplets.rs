use std::collections::HashMap;

fn beautifulTriplets(d: i32, arr: &[i32]) -> i32 {
    let mut count = 0;
    let mut pairs: HashMap<i32, [i32; 2]> = HashMap::new();
    let mut indexes: HashMap<i32, usize> = HashMap::new();
    for (i, &x) in arr.iter().enumerate() {
        pairs.insert(x, [x + d, x + (2 * d)]);
        indexes.insert(x, i);
    }

    for &el in arr {
        if let Some(&[j, k]) = pairs.get(&el) {
            if indexes.contains_key(&j) && indexes.contains_key(&k) {
                count += 1;
            }
        }
    }
    count
}

fn beautiful_triplets(d: i32, arr: &[i32]) -> i32 {
    let mut count = 0;
    let mut freq_map: HashMap<i32, i32> = HashMap::new();

    for &i in arr {
        *freq_map.entry(i).or_insert(0) += 1;
    }

    for (&i, &i_count) in &freq_map {
        let j = i + d;
        let k = i + (2 * d);
        if let (Some(j_count), Some(k_count)) = (freq_map.get(&j), freq_map.get(&k)) {
            count += i_count * j_count * k_count;
        }
    }
    count
}

fn main() {
    let arr = [2, 2, 3, 4, 5];
    let brr = [2, 2, 3, 3, 4, 4]; // 0,2,4 - 0,2,5 - 0,3,4 - 0,3,5 - 1,2,4 - 1,2,5 - 1,3,4 - 1,3,5
    print!("{}", beautiful_triplets(1, &arr));
}
