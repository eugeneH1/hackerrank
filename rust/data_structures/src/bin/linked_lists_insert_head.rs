use std::io::{self, BufRead};
use std::{env, fs};

struct SinglyLinkedList {
    head: Option<Box<SinglyLinkedListNode>>,
}

impl SinglyLinkedList {
    fn new() -> Self {
        Self { head: None }
    }
}

struct SinglyLinkedListNode {
    data: i32,
    next: Option<Box<SinglyLinkedListNode>>,
}

impl SinglyLinkedListNode {
    fn new(data: i32) -> Self {
        Self { data, next: None }
    }
}

fn insert_head(n: i32, list: &mut SinglyLinkedList) -> &SinglyLinkedList {
    let mut new_node = SinglyLinkedListNode::new(n);
    new_node.next = list.head.take();
    list.head = Some(Box::new(new_node));
    list
}

fn write_linked_list(list: &SinglyLinkedList, out_buff: &mut String) {
    let mut current = list.head.as_ref();
    while let Some(node) = current {
        out_buff.push_str(&format!("{}\n", node.data));
        current = node.next.as_ref();
    }
}

fn main() {
    let output_path = env::var("OUTPUT_PATH").expect("Couldn't find output path");

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let t: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();

    let mut output_buffer = String::new();

    let mut head = SinglyLinkedList::new();

    for _ in 0..t {
        let next_val: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();
        insert_head(next_val, &mut head);
    }

    write_linked_list(&head, &mut output_buffer);

    fs::write(output_path, output_buffer).expect("Failed to write file");
}
