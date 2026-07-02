fn lowestTriangle(trianglebase: i32, area: i32) -> i32 {
    let numerator = match trianglebase {
        num if num % 2 == 0 => trianglebase,
        _ => (trianglebase + 1) / 2,
    };
    (area + numerator - 1) / numerator
}

fn main() {
    println!("{}", lowestTriangle(73, 769025));
}
