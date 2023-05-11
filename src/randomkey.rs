use rand::Rng;

use crate::config::Config;
use crate::tokens::{TokenSetKind, Tokens};

pub struct RandomKey {}

impl RandomKey {
    pub fn generate(config: &Config) -> String {
        for _i in 0..config.max_retries {
            let token_vec = RandomKey::get_token_vec(config);

            let random_key = RandomKey::get_random_key(token_vec, config.length);

            if RandomKey::validate(&random_key, config).is_ok() {
                return random_key;
            }
        }

        eprintln!("Failed to generate valid token");
        String::new()
    }

    fn get_random_key(mut token_vec: Vec<Tokens>, length: usize) -> String {
        let size = token_vec.len();

        (0..length)
            .map(|_| rand::thread_rng().gen_range(0..size))
            .map(|x| token_vec[x].get_token())
            .collect::<String>()
    }

    fn get_token_vec(config: &Config) -> Vec<Tokens> {
        let mut token_vec: Vec<Tokens> = Vec::new();
        if config.include_lowercase_chars {
            token_vec.push(Tokens::new(TokenSetKind::LowerCase));
        }
        if config.include_uppercase_chars {
            token_vec.push(Tokens::new(TokenSetKind::UpperCase));
        }
        if config.include_numeric_chars {
            token_vec.push(Tokens::new(TokenSetKind::Number));
        }
        if config.include_special_chars {
            token_vec.push(Tokens::new(TokenSetKind::SpecialChar));
        }

        token_vec
    }

    fn validate(random_key: &str, config: &Config) -> Result<(), &'static str> {
        validate_special_chars(random_key, config.include_special_chars)?;
        validate_lowercase(random_key, config.include_lowercase_chars)?;
        validate_uppercase(random_key, config.include_uppercase_chars)?;
        validate_numbers(random_key, config.include_numeric_chars)?;

        Ok(())
    }
}

fn validate_special_chars(
    random_key: &str,
    include_special_char: bool,
) -> Result<(), &'static str> {
    let all_alphanumeric = random_key.chars().all(char::is_alphanumeric);

    if all_alphanumeric && include_special_char {
        return Err("No special characters present");
    } else if !all_alphanumeric && !include_special_char {
        return Err("Token is not allowed to contain special characters");
    }
    Ok(())
}

fn validate_lowercase(random_key: &str, include_lowercase: bool) -> Result<(), &'static str> {
    if !random_key.chars().any(char::is_lowercase) && include_lowercase {
        return Err("No lowercase characters present");
    }
    Ok(())
}

fn validate_uppercase(random_key: &str, include_uppercase: bool) -> Result<(), &'static str> {
    if !random_key.chars().any(char::is_uppercase) && include_uppercase {
        return Err("No uppercase characters present");
    }
    Ok(())
}

fn validate_numbers(random_key: &str, include_digit: bool) -> Result<(), &'static str> {
    if !random_key.chars().any(char::is_numeric) && include_digit {
        return Err("No numeric characters present");
    }
    Ok(())
}
