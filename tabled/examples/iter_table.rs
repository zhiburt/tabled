//! The example can be run by this command
//! `cargo run --example iter_table`

use std::io::BufRead;

use tabled::{settings::Style, tables::iter::IterTable};

fn main() {
    let path = file!();
    let file = std::fs::File::open(path).unwrap();
    let reader = std::io::BufReader::new(file);
    let iterator = reader.lines().enumerate().map(|(i, line)| match line {
        Ok(line) => [i.to_string(), String::from("ok"), line],
        Err(err) => [i.to_string(), String::from("error"), err.to_string()],
    });

    let table = IterTable::new(iterator).with(Style::ascii_rounded());

    table.build(std::io::stdout()).unwrap();
    println!()
}
