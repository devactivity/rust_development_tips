use rand_basics::RandomWord;

fn main() {
    let mut generator = RandomWord::new_generator();

    for _ in 0..5 {
        println!("{}", generator.get_word());
    }
}
