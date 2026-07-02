fn findPoint(px: i32, py: i32, qx: i32, qy: i32) -> Vec<i32> {
    let delta_x = qx - px;
    let delta_y = qy - py;
    vec![qx + delta_x, qy + delta_y]
}

fn main() {
    println!(findPoint(0, 0, 1, -1));
}
