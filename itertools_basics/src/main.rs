// fn permutation<T: Clone>(data: &[T], prefix: Vec<T>, results: &mut Vec<Vec<T>>, max_len: usize) {
//     if prefix.len() == max_len {
//         results.push(prefix);
//     } else {
//         for i in 0..data.len() {
//             let mut new_prefix = prefix.clone();
//
//             new_prefix.push(data[i].clone());
//
//             let new_data: Vec<T> = data
//                 .into_iter()
//                 .enumerate()
//                 .filter_map(|(j, x)| if i != j { Some(x.clone()) } else { None })
//                 .collect();
//
//             permutation(&new_data, new_prefix, results, max_len);
//         }
//     }
// }
//
// fn main() {
//     let data = vec![1, 2, 3, 4, 5];
//     let mut results: Vec<Vec<i32>> = Vec::new();
//
//     permutation(&data, vec![], &mut results, 4);
//
//     for result in results {
//         println!("{:?}", result);
//     }
// }

use itertools::Itertools;

fn main() {
    let data = vec![1, 2, 3, 4, 5];
    let results = data.into_iter().permutations(4);

    for result in results {
        println!("{:?}", result);
    }
}
