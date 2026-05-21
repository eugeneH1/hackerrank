fn queens_attack(n: i32, k: i32, r_q: i32, c_q: i32, obstacles: &[Vec<i32>]) -> i32 {
    let mut moves = 0;
    let skip_direction = [
        r_q == n,               // up
        (r_q == n || c_q == n), // up & right
        c_q == n,               // right
        (r_q == 1 || c_q == n), // down & right
        r_q == 1,               // down
        (r_q == 1 || c_q == 1), // down & left
        c_q == 1,               // left
        (r_q == n || c_q == 1), // up & left
    ];
    for i in 1..=8 {
        if skip_direction[i - 1] {
            continue;
        }
        match i {
            1 => {
                // up
                let obstacle = obstacles
                    .iter()
                    .filter(|obstacle| obstacle[0] > r_q && obstacle[1] == c_q)
                    .min_by_key(|obs| obs[0]); // min by r - vertical movement
                match obstacle {
                    Some(obstacle) => moves += obstacle[0] - r_q - 1,
                    None => moves += n - r_q,
                }
            }
            2 => {
                // up & right
                let obstacle = obstacles
                    .iter()
                    .filter(|obstacle| {
                        obstacle[0] > r_q
                            && obstacle[1] > c_q
                            && obstacle[0] - r_q == obstacle[1] - c_q
                    })
                    .min_by_key(|obs| obs[0]); // can use either they will be the same
                match obstacle {
                    Some(obstacle) => moves += obstacle[0] - r_q - 1,
                    None => moves += std::cmp::min(n - r_q, n - c_q),
                }
            }
            3 => {
                // right
                let obstacle = obstacles
                    .iter()
                    .filter(|obstacle| obstacle[1] > c_q && obstacle[0] == r_q)
                    .min_by_key(|obs| obs[1]); // min by c - horizontal movement
                match obstacle {
                    Some(obstacle) => moves += obstacle[1] - c_q - 1,
                    None => moves += n - c_q,
                }
            }
            4 => {
                // down & right
                let obstacle = obstacles
                    .iter()
                    .filter(|obstacle| {
                        obstacle[0] < r_q
                            && obstacle[1] > c_q
                            && r_q - obstacle[0] == obstacle[1] - c_q
                    })
                    .max_by_key(|obs| obs[0]); // > that's less q or < that more q 
                match obstacle {
                    Some(obstacle) => moves += r_q - obstacle[0] - 1,
                    None => moves += std::cmp::min(r_q - 1, n - c_q),
                }
            }
            5 => {
                // down
                let obstacle = obstacles
                    .iter()
                    .filter(|obstacle| obstacle[1] == c_q && obstacle[0] < r_q)
                    .max_by_key(|obs| obs[0]); // max by r - vertical movement
                match obstacle {
                    Some(obstacle) => moves += r_q - obstacle[0] - 1,
                    None => moves += r_q - 1,
                }
            }
            6 => {
                // down & left
                let obstacle = obstacles
                    .iter()
                    .filter(|obstacle| {
                        obstacle[0] < r_q
                            && obstacle[1] < c_q
                            && r_q - obstacle[0] == c_q - obstacle[1]
                    })
                    .max_by_key(|obs| obs[0]); // max by value dec, min by value inc
                match obstacle {
                    Some(obstacle) => moves += c_q - obstacle[1] - 1,
                    None => moves += std::cmp::min(r_q - 1, c_q - 1),
                }
            }
            7 => {
                // left
                let obstacle = obstacles
                    .iter()
                    .filter(|obstacle| obstacle[1] < c_q && obstacle[0] == r_q)
                    .max_by_key(|obs| obs[1]); // max by c - horizontal movement
                match obstacle {
                    Some(obstacle) => moves += c_q - obstacle[1] - 1,
                    None => moves += c_q - 1,
                }
            }
            8 => {
                // up & left
                let obstacle = obstacles
                    .iter()
                    .filter(|obstacle| {
                        obstacle[0] > r_q
                            && obstacle[1] < c_q
                            && obstacle[0] - r_q == c_q - obstacle[1]
                    })
                    .min_by_key(|obs| obs[0]); // max by value dec, min by value inc
                match obstacle {
                    Some(obstacle) => moves += c_q - obstacle[1] - 1,
                    None => moves += std::cmp::min(n - r_q, c_q - 1),
                }
            }
            _ => {} // Still need this default fallback
        }
    }
    moves
}

fn queens_attack_optimised(n: i32, _k: i32, r_q: i32, c_q: i32, obstacles: &[Vec<i32>]) -> i32 {
    // Distance to closest obstacle or board edge in all 8 directions
    // Order: Up, Up-Right, Right, Down-Right, Down, Down-Left, Left, Up-Left
    let mut closest_up = n - r_q;
    let mut closest_ur = std::cmp::min(n - r_q, n - c_q);
    let mut closest_right = n - c_q;
    let mut closest_dr = std::cmp::min(r_q - 1, n - c_q);
    let mut closest_down = r_q - 1;
    let mut closest_dl = std::cmp::min(r_q - 1, c_q - 1);
    let mut closest_left = c_q - 1;
    let mut closest_ul = std::cmp::min(n - r_q, c_q - 1);

    for obs in obstacles {
        let r_o = obs[0];
        let c_o = obs[1];

        if c_o == c_q {
            // Vertical movements
            if r_o > r_q {
                closest_up = std::cmp::min(closest_up, r_o - r_q - 1);
            } else {
                closest_down = std::cmp::min(closest_down, r_q - r_o - 1);
            }
        } else if r_o == r_q {
            // Horizontal movements
            if c_o > c_q {
                closest_right = std::cmp::min(closest_right, c_o - c_q - 1);
            } else {
                closest_left = std::cmp::min(closest_left, c_q - c_o - 1);
            }
        } else if (r_o - r_q).abs() == (c_o - c_q).abs() {
            // Diagonal movements
            if r_o > r_q && c_o > c_q {
                closest_ur = std::cmp::min(closest_ur, r_o - r_q - 1);
            } else if r_o < r_q && c_o > c_q {
                closest_dr = std::cmp::min(closest_dr, r_q - r_o - 1);
            } else if r_o < r_q && c_o < c_q {
                closest_dl = std::cmp::min(closest_dl, r_q - r_o - 1);
            } else if r_o > r_q && c_o < c_q {
                closest_ul = std::cmp::min(closest_ul, r_o - r_q - 1);
            }
        }
    }

    closest_up
        + closest_ur
        + closest_right
        + closest_dr
        + closest_down
        + closest_dl
        + closest_left
        + closest_ul
}

fn main() {}
