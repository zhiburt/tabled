use clap::{arg, command};

pub struct Config {
    pub debug: bool,
}

pub fn parse() -> Config {
    let matches = command!()
        .arg(arg!(
            -d --debug ... "Turn debugging information on"
        ))
        .get_matches();

    let debug = matches.is_present("debug");

    Config { debug }
}
