use test_module::tournament;

fn main() {
    // vec![
    //     vec![(1, 3), (2, 4)],  // first round:  1 vs 3, 2 vs 4
    //     vec![(1, 4), (3, 2)],  // second round: 1 vs 4, 3 vs 2
    //     vec![(1, 2), (4, 3)]   // third round:  1 vs 2, 4 vs 3
    // ]
    println!("{:?}", tournament(4));
}
