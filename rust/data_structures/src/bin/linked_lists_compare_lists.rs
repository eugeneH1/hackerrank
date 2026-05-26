use std::fmt::Write;
use std::io::{self, BufRead};
use std::{env, fs};

struct SinglyLinkedListNode {
    data: i32,
    next: Option<Box<SinglyLinkedListNode>>,
}

fn compare_lists_naive(a: &SinglyLinkedListNode, b: &SinglyLinkedListNode) -> i32 {
    let mut node_a: &SinglyLinkedListNode = a;
    let mut node_b: &SinglyLinkedListNode = b;

    while node_a.data == node_b.data {
        if node_a.next.is_some() && node_b.next.is_some() {
            node_a = node_a.next.as_deref().unwrap();
            node_b = node_b.next.as_deref().unwrap();
        } else if node_b.next.is_some() || node_a.next.is_some() {
            return 0;
        } else {
            return 1;
        }
    }
    0
}

fn compare_lists(a: &SinglyLinkedListNode, b: &SinglyLinkedListNode) -> i32 {
    let mut node_a = a;
    let mut node_b = b;

    loop {
        // 1. If data doesn't match, they aren't identical
        if node_a.data != node_b.data {
            return 0;
        }

        // 2. Pattern match on BOTH 'next' fields simultaneously
        match (&node_a.next, &node_b.next) {
            (Some(next_a), Some(next_b)) => {
                // Both have a next node, advance the pointers safely
                node_a = next_a;
                node_b = next_b;
            }
            (None, None) => {
                // Both reached the end at the exact same time -> identical!
                return 1;
            }
            _ => {
                // One is None and the other is Some -> length mismatch!
                return 0;
            }
        }
    }
}

fn main() {
    let output_path = env::var("OUTPUT_PATH").expect("Couldn't find output path");

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let t: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();

    let mut output_buffer = String::new();
    for _ in 0..t {
        let n: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();

        let mut head_1: Option<Box<SinglyLinkedListNode>> = None;
        let mut current_1 = &mut head_1;
        for _ in 0..n {
            let val: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();
            let new_node = Box::new(SinglyLinkedListNode {
                data: val,
                next: None,
            });
            *current_1 = Some(new_node);
            current_1 = match current_1 {
                Some(node) => &mut node.next,
                None => unreachable!(),
            };
        }

        let m: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();
        let mut head_2: Option<Box<SinglyLinkedListNode>> = None;
        let mut current_2 = &mut head_2;
        for _ in 0..m {
            let val: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();
            let new_node = Box::new(SinglyLinkedListNode {
                data: val,
                next: None,
            });
            *current_2 = Some(new_node);
            current_2 = match current_2 {
                Some(node) => &mut node.next,
                None => unreachable!(),
            };
        }

        let result = compare_lists(&head_1.unwrap(), &head_2.unwrap());
        writeln!(output_buffer, "{}", result).unwrap();
    }

    fs::write(output_path, output_buffer).expect("Failed to write file");
}
