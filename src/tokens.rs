use rand::distributions::Uniform;
use rand::rngs::StdRng;
use rand::{Rng, RngCore, SeedableRng};

pub struct Tokens {
    pub token_set: Vec<char>,
    pub range: Uniform<usize>,
    pub rng: StdRng,
}

pub enum TokenSetKind {
    LowerCase,
    UpperCase,
    Number,
    SpecialChar,
}

impl Tokens {
    pub fn new(token_set_kind: TokenSetKind) -> Tokens {
        let token_set = get_token_set(token_set_kind);
        let alphabet_length = token_set.len();

        Tokens {
            token_set,
            range: Uniform::new(0, alphabet_length),
            rng: StdRng::from_seed(generate_seed()),
        }
    }

    pub fn get_token(&mut self) -> char {
        self.token_set[self.rng.sample(self.range)]
    }
}

fn generate_seed() -> [u8; 32] {
    let mut seed = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut seed);
    seed
}

fn get_token_set(token_set_kind: TokenSetKind) -> Vec<char> {
    match token_set_kind {
        TokenSetKind::LowerCase => ('a'..='z').collect::<Vec<char>>(),
        TokenSetKind::UpperCase => ('A'..='Z').collect::<Vec<char>>(),
        TokenSetKind::Number => ('0'..='9').collect::<Vec<char>>(),
        TokenSetKind::SpecialChar => vec![
            '!', '#', '$', '%', '&', '*', ']', '[', '(', ')', '{', '}', '+', '-', ',', '=', '/',
            '?',
        ],
    }
}
