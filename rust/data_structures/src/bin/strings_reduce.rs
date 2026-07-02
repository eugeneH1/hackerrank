// use regex::Regex;

// fn superReducedString(s: &str) -> String {
//     let re = Regex::new(r"([a-z])\1").unwrap();
//     re.replace_all(s, "").to_string()
// }

// fn super_reduced_string(s: &str) -> String {
//     let mut res = String::new();
//     let mut end_of_string = "";
//     let mut i = 0;
//     while true {

//         end_of_string = s.chars().nth(i).unwrap();
//     }
//     res
// }

fn what_now(s: &str) -> String {
    let mut stack = Vec::with_capacity(s.len());

    for c in s.chars() {
        match stack.last() {
            Some(&top) if top == c => {
                stack.pop();
            },
            _ => stack.push(c),
        }
        // if stack.last() == Some(&) {
        //     stack.pop();
        // } else {
        //     stack.push(c);
        // }
    }
    match stack.into_iter().collect::<String>() {
        s if s.is_empty() => String::from("Emtpy string"),
        s => s,
    }
}
fn main() {
let s = String::from("abba");
let result = what_now(&s);
println!("{}", result);
}