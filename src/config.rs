use clap::ArgMatches;

const DEFAULT_PASS_LEN: usize = 20;

pub struct Config {
    pub length: usize,
    pub include_special_chars: bool,
    pub include_lowercase_chars: bool,
    pub include_uppercase_chars: bool,
    pub include_numeric_chars: bool,
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
            include_lowercase_chars: true,
            include_uppercase_chars: true,
            include_numeric_chars: true,
        })
    }
}
