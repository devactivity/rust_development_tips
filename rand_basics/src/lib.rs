use rand::prelude::*;

pub struct RandomWord {
    rng: ThreadRng,
}

impl RandomWord {
    pub fn new_generator() -> Self {
        Self {
            rng: rand::thread_rng(),
        }
    }

    pub fn get_word(&mut self) -> String {
        let max_unique_letters = 10;
        let mut v = Vec::new();

        while v.len() < max_unique_letters {
            let ch = match self.rng.gen_range(0..=51) {
                i @ 0..=25 => (b'A' + i) as char,
                i @ 26..=51 => (b'a' + (i - 26)) as char,
                _ => unreachable!(),
            };

            v.push(ch);
        }

        (0..10)
            .map(|_| self.rng.gen_range(0..max_unique_letters))
            .map(|i| v[i])
            .collect()
    }
}

impl Iterator for RandomWord {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.get_word())
    }
}
