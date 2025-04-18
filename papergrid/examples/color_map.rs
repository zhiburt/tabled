use std::{
    collections::HashMap,
    fmt::{self},
    io::{self, Write},
};

use papergrid::{
    ansi::ANSIFmt,
    config::{spanned::SpannedConfig, Borders, Position},
    dimension::{iterable::IterGridDimension, Estimate},
    grid::iterable::IterGrid,
    records::IterRecords,
};

fn main() {
    let records = vec![vec!["Hello", "World"], vec!["Hi", "World"]];
    let records = IterRecords::new(&records, 2, None);

    let mut cfg = SpannedConfig::default();
    cfg.set_borders_missing('+');
    cfg.set_borders(Borders {
        top: Some('-'),
        bottom: Some('-'),
        left: Some('|'),
        right: Some('|'),
        vertical: Some('|'),
        horizontal: Some('-'),
        ..Default::default()
    });

    let mut dims = IterGridDimension::default();
    dims.estimate(records, &cfg);

    let mut colors = HashMap::default();
    colors.insert(Position::new(0, 0), Style::Blue);
    colors.insert(Position::new(1, 1), Style::Black);

    let grid = IterGrid::new(records, &dims, &cfg, &colors);
    grid.build(StdoutWriter::new()).unwrap();
    println!();
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
