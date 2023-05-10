use std::process;

use autoclap::autoclap;
use clap::{Arg, ArgAction, Command};

use random_keygen::config::Config;
use random_keygen::keygen::Password;

fn main() {
    let app = autoclap!();
    let args = app
        .arg(
            Arg::new("length")
                .long("length")
                .short('l')
                .help("Password length.")
                .required(false),
        )
        .arg(
            Arg::new("include_special_chars")
                .long("include-special-chars")
                .short('c')
                .action(ArgAction::SetTrue)
                .help("Include special characters (i.e.: !, $, #).")
                .required(false),
        )
        .get_matches();

    let config = Config::build(args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("{}", Password::generate(&config));
}
