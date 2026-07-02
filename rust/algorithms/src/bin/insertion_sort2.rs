fn insertionSort2(arr: &[i32]) {
    let mut result = arr.to_vec();

    for i in 1..arr.len() {
        insertion(&mut result[0..=i]);
        for el in &result {
            print!("{} ", el);
        }
        println!();
    }
}

fn insertion(arr: &mut [i32]) {
    let n: usize = arr.len();
    // if n <= 1 {
    //     return;
    // }

    let element = arr[n - 1]; // 9

    for i in (0..n - 1).rev() {
        //8, 10, 9
        // i = 1, i = 0
        if arr[i] > element {
            //arr[1] = 10, arr[0] = 8
            arr[i + 1] = arr[i]; // [8, 10, 10]
            arr[i] = element; //[8, 9, 10]
        } else {
            arr[i + 1] = element;
            break;
        }
    }
}

fn main() {
    let arr = vec![9, 8, 6, 5, 7, 2];
    // 1,5,6,8,7,2
    // 8 > 7 -> 1, 5, 6, 8, 8, 2
    // 6 < 7 -> so insert after index of 6 or index(6) + 1 -> 1,5,6,7,8,2
    insertionSort2(&arr);
}
