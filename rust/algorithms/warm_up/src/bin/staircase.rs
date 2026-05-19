fn print_stairs(n: i32) {
    let n_usize = n as usize;
    for i in 1..=n {
        let line = "#".repeat(i as usize);
        println!("{:>n_usize$}", line);
    }
}

fn main() {
    print_stairs(4);
}
