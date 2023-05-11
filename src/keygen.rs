use rand::Rng;

use crate::config::Config;
use crate::tokens::{CharSet, Tokens};

pub struct Password {}

impl Password {
    pub fn generate(config: &Config) -> String {
        let mut token_sets = Password::get_token_sets(config);

        let size = token_sets.len();

        let password = (0..config.length)
            .map(|_| rand::thread_rng().gen_range(0..size))
            .map(|x| token_sets[x].get_char())
            .collect::<String>();

        if Password::validate(&password, config).is_err() {
            panic!("Invalid token was generated");
        }

        password
    }

    fn get_token_sets(config: &Config) -> Vec<Tokens> {
        let mut token_sets: Vec<Tokens> = Vec::new();
        if config.include_lowercase_chars {
            token_sets.push(Tokens::new(CharSet::LowerCase));
        }
        if config.include_uppercase_chars {
            token_sets.push(Tokens::new(CharSet::UpperCase));
        }
        if config.include_numeric_chars {
            token_sets.push(Tokens::new(CharSet::Number));
        }
        if config.include_special_chars {
            token_sets.push(Tokens::new(CharSet::SpecialChar));
        }

        token_sets
    }

    fn validate(token: &str, config: &Config) -> Result<(), &'static str> {
        validate_special_chars(token, config.include_special_chars)?;
        validate_lowercase(token, config.include_lowercase_chars)?;
        validate_uppercase(token, config.include_uppercase_chars)?;
        validate_numbers(token, config.include_numeric_chars)?;

        Ok(())
    }
}

fn validate_special_chars(token: &str, include_special_char: bool) -> Result<(), &'static str> {
    let all_alphanumeric = token.chars().all(char::is_alphanumeric);

    if all_alphanumeric && include_special_char {
        return Err("No special characters present");
    } else if !all_alphanumeric && !include_special_char {
        return Err("Token is not allowed to contain special characters");
    }
    Ok(())
}

fn validate_lowercase(token: &str, include_lowercase: bool) -> Result<(), &'static str> {
    if !token.chars().any(char::is_lowercase) && include_lowercase {
        return Err("No lowercase characters present");
    }
    Ok(())
}

fn validate_uppercase(token: &str, include_uppercase: bool) -> Result<(), &'static str> {
    if !token.chars().any(char::is_uppercase) && include_uppercase {
        return Err("No uppercase characters present");
    }
    Ok(())
}

fn validate_numbers(token: &str, include_digit: bool) -> Result<(), &'static str> {
    if !token.chars().any(char::is_numeric) && include_digit {
        return Err("No numeric characters present");
    }
    Ok(())
}
