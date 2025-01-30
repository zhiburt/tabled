//! This example demonstrates using a [`HashMap`] of colors to simplify styling
//! sections of a [`Grid`] without embedding ANSI escape characters into cell values.
//!
//! * 🚩 This example requires the `ansi` feature.

use std::{
    collections::HashMap,
    fmt::{self},
    io::{self, Write},
};

use papergrid::{
    ansi::ANSIFmt,
    config::{pos, spanned::SpannedConfig, Borders, Position},
    dimension::{spanned::SpannedGridDimension, Estimate},
    grid::iterable::Grid,
    records::IterRecords,
};

fn main() {
    let records = vec![vec!["Hello", "World"], vec!["Hi", "World"]];
    let records = IterRecords::new(&records, 2, None);

    let cfg = generate_table_config();

    let mut dims = SpannedGridDimension::default();
    dims.estimate(records, &cfg);

    let colors = generate_colors();

    let grid = Grid::new(records, &dims, &cfg, &colors);
    grid.build(StdoutWriter::new()).unwrap();
    println!();
}

fn generate_colors() -> HashMap<Position, Style> {
    let mut m = HashMap::default();
    m.insert(pos(0, 0), Style::Blue);
    m.insert(pos(1, 1), Style::Black);
    m
}

fn generate_table_config() -> SpannedConfig {
    let mut cfg = SpannedConfig::default();
    cfg.set_borders(Borders {
        top: Some('-'),
        bottom: Some('-'),
        left: Some('|'),
        right: Some('|'),
        vertical: Some('|'),
        horizontal: Some('-'),
        ..Default::default()
    });
    cfg.set_borders_missing('+');
    cfg
}

#[derive(Debug, Clone, Copy)]
enum Style {
    Black,
    Blue,
}

impl ANSIFmt for Style {
    fn fmt_ansi_prefix<W: fmt::Write>(&self, f: &mut W) -> fmt::Result {
        let c = match self {
            Style::Black => "\u{1b}[34m",
            Style::Blue => "\u{1b}[31m",
        };

        f.write_str(c)
    }
}

struct StdoutWriter(io::Stdout);

impl StdoutWriter {
    fn new() -> Self {
        Self(io::stdout())
    }
}

impl fmt::Write for StdoutWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let buf = s.as_bytes();
        let mut offset = 0;

        loop {
            let n = self.0.write(&buf[offset..]).map_err(|_| fmt::Error)?;
            if n == buf.len() {
                break;
            }

            offset += n;
        }

        self.0.flush().map_err(|_| fmt::Error)?;

        Ok(())
    }
}
