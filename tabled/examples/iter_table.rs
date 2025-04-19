use std::{
    fs::File,
    io::{stdout, BufRead, BufReader},
};

use tabled::{settings::Style, tables::IterTable};

fn main() {
    let path = file!();
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let iter = reader.lines().enumerate().map(|(index, line)| match line {
        Ok(line) => [index.to_string(), "ok".into(), line],
        Err(err) => [index.to_string(), "error".into(), err.to_string()],
    });

    let mut table = IterTable::new(iter);
    table.with(Style::ascii_rounded());

    table.build(stdout()).unwrap();

    println!()
}
