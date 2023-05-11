use clap::ArgMatches;

const DEFAULT_PASS_LEN: usize = 20;
const MAX_RETRIES: u32 = 15;

pub struct Config {
    pub length: usize,
    pub include_special_chars: bool,
    pub include_lowercase_chars: bool,
    pub include_uppercase_chars: bool,
    pub include_numeric_chars: bool,
    pub max_retries: u32,
}

impl Config {
    pub fn build(args: ArgMatches) -> Result<Config, &'static str> {
        // can we make this block easier???
        let length: usize = args
            .get_one::<String>("length")
            .map(|s| s.as_str())
            .unwrap_or(&DEFAULT_PASS_LEN.to_string())
            .parse()
            .unwrap();

        Ok(Config {
            length,
            include_special_chars: args.get_flag("include_special_chars"),
            // hard coded for now, but we can easily turn these into CLI args:
            include_lowercase_chars: true,
            include_uppercase_chars: true,
            include_numeric_chars: true,
            max_retries: MAX_RETRIES,
        })
    }
}
