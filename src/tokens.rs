use rand::distributions::Uniform;
use rand::rngs::StdRng;
use rand::{Rng, RngCore, SeedableRng};

pub struct Tokens {
    pub chars: Vec<char>,
    pub range: Uniform<usize>,
    pub rng: StdRng,
}

pub enum CharSet {
    LowerCase,
    UpperCase,
    Number,
    SpecialChar,
}

impl Tokens {
    pub fn new(char_set: CharSet) -> Tokens {
        let chars = get_chars(char_set);
        let alphabet_length = chars.len();

        Tokens {
            chars,
            range: Uniform::new(0, alphabet_length),
            rng: StdRng::from_seed(generate_seed()),
        }
    }

    pub fn get_char(&mut self) -> char {
        self.chars[self.rng.sample(self.range)]
    }
}

fn generate_seed() -> [u8; 32] {
    let mut seed = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut seed);
    seed
}

fn get_chars(char_set: CharSet) -> Vec<char> {
    match char_set {
        CharSet::LowerCase => ('a'..='z').into_iter().collect::<Vec<char>>(),
        CharSet::UpperCase => ('A'..='Z').into_iter().collect::<Vec<char>>(),
        CharSet::Number => ('0'..='9').into_iter().collect::<Vec<char>>(),
        CharSet::SpecialChar => vec![
            '!', '#', '$', '%', '&', '*', ']', '[', '(', ')', '{', '}', '+', '-', ',', '=', '/',
            '?',
        ],
    }
}
