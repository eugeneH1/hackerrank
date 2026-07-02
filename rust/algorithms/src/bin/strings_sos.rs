fn marsExploration(s: &str) -> i32 {
    let mask: Vec<u8> = vec![83, 79, 83];
    let mut count = 0;
    for chunk in s.as_bytes().chunks(3) {
        for i in 0..3 {
            if mask[i] != chunk[i] {
                count += 1;
            }
        }
    }
    count
}

fn marsExploration2(s: &str) -> i32 {
    let mask = b"SOS";
    s.as_bytes()
        .iter()
        .zip(mask.iter().cycle())
        .filter(|(chunk_byte, mask_byte)| chunk_byte != mask_byte)
        .count() as i32
}

fn mars_exploration(s: &str) -> i32 {
    let mask = vec!['S', 'O', 'S'];
    // s.as_bytes().windows(3).fold()
    0
}

fn main() {
    let s = "SOSSPSSQSSOR";
    print!("{}", marsExploration(s));
}
