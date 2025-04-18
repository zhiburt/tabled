use std::{
    fs::File,
    io::{stdout, BufRead, BufReader},
};

use tabled::{settings::Style, tables::IterTable};

fn main() {
    let path = file!();
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let iterator = reader.lines().enumerate().map(|(i, line)| match line {
        Ok(line) => [i.to_string(), "ok".into(), line],
        Err(err) => [i.to_string(), "error".into(), err.to_string()],
    });

    let table = IterTable::new(iterator).with(Style::ascii_rounded());

    table.build(stdout()).unwrap();

    println!()
}
