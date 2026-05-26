// fn matching_string(stringList: &[String], queries: &[String]) -> Vec<i32> {
//     let mut result: Vec<i32> = vec![0; queries.len()];
//     for i in 0..queries.len() {
//         stringList.iter().for_each(|s| {
//             if s == &queries[i] {
//                 result[i] += 1;
//             }
//         });
//     }
//     result
// }

fn matching_string(stringList: &[String], queries: &[String]) -> Vec<i32> {
    queries
        .iter()
        .map(|q| stringList.iter().filter(|&s| s == q).count() as i32)
        .collect()
}
fn main() {}
