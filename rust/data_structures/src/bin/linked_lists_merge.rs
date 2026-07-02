use std::fmt::Write;
use std::io::{self, BufRead};
use std::{env, fs};

struct Node {
    data: i32,
    next: Option<Box<Node>>,
}

fn merge(mut a: Option<Box<Node>>, mut b: Option<Box<Node>>) -> Node {
    let mut dummy = Node {
        data: 0,
        next: None,
    };
    let mut current = &mut dummy;
    while a.is_some() && b.is_some() {
        println!(
            "a: {}, b: {}",
            a.as_ref().unwrap().data,
            b.as_ref().unwrap().data
        );
        if a.as_ref().unwrap().data < b.as_ref().unwrap().data {
            let mut next_node = a.take();
            match next_node.as_mut().unwrap().next {
                Some(_) => a = next_node.as_mut().unwrap().next.take(),
                None => (),
            }
            current.next = next_node;
        } else {
            let mut next_node = b.take();
            match next_node.as_mut().unwrap().next {
                Some(_) => b = next_node.as_mut().unwrap().next.take(),
                None => (),
            }
            current.next = next_node;
        }
        current = current.next.as_mut().unwrap();
    }
    let head = dummy.next.take();
    *head.unwrap()
}

fn write_list(head: Node, buffer: &mut String) {
    let mut current = head;
    while current.next.is_some() {
        write!(buffer, "{}", current.data).expect("well fuck");
        current = *current.next.unwrap();
    }
}

fn main() {
    let output_path = match env::var("OUTPUT_PATH") {
        Ok(val) => val,
        Err(_) => String::from("merge.txt"),
    };

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let t: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();

    let mut output_buffer = String::new();
    for _ in 0..t {
        let n: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();

        let mut head_1: Option<Box<Node>> = None;
        let mut current_1 = &mut head_1;
        for _ in 0..n {
            let val: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();
            let new_node = Box::new(Node {
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
        let mut head_2: Option<Box<Node>> = None;
        let mut current_2 = &mut head_2;
        for _ in 0..m {
            let val: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();
            let new_node = Box::new(Node {
                data: val,
                next: None,
            });
            *current_2 = Some(new_node);
            current_2 = match current_2 {
                Some(node) => &mut node.next,
                None => unreachable!(),
            };
        }

        let result = merge(head_1, head_2);
        write_list(result, &mut output_buffer);
    }

    fs::write(output_path, output_buffer).expect("Failed to write file");
}
