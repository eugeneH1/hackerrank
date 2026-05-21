fn compareTriplets(a: &[i32], b: &[i32]) -> Vec<i32> {
    let result = a.iter().zip(b.iter()).fold([0, 0], |mut scores, (val_a, val_b)| {
        if val_b > val_a {
            scores[1] += 1;
        }
        if val_a > val_b {
            scores[0] += 1;
        }
        scores
    });
    result.to_vec()
}

fn main() {
    
}
