use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
    io::Write,
};

use owo_colors::{
    colors::{Black, Blue, Red},
    Style as OStyle,
};

use papergrid::{
    color::Color,
    config::{Borders, GridConfig, Position},
    dimension::{Dimension, ExactDimension},
    records::IterRecords,
    Grid,
};

fn main() {
    let records = vec![vec!["Hello", "World"], vec!["Hi", "World"]];
    let records = IterRecords::new(&records, 2, None);

    let cfg = generate_table_config();

    let mut dimension = ExactDimension::default();
    dimension.estimate(records, &cfg);

    let colors = generate_colors();

    let grid = Grid::new(records, &cfg, &dimension).with_colors(colors);

    grid.build(UTF8Stdout(std::io::stdout())).unwrap();
    println!();
}

fn generate_colors() -> HashMap<Position, Style> {
    HashMap::from([
        ((0, 0), Style(OStyle::default().bg::<Red>().fg::<Black>())),
        ((1, 1), Style(OStyle::default().bg::<Blue>())),
    ])
}

fn generate_table_config() -> GridConfig {
    let mut cfg = GridConfig::default();
    cfg.set_borders(Borders {
        top: Some('-'),
        bottom: Some('-'),
        vertical_left: Some('|'),
        vertical_right: Some('|'),
        vertical: Some('|'),
        horizontal: Some('-'),
        ..Default::default()
    });
    cfg.set_borders_missing('+');

    cfg
}

#[derive(Debug, Clone, Default)]
struct Style(OStyle);

impl Color for Style {
    fn fmt_prefix<W: std::fmt::Write>(&self, f: &mut W) -> std::fmt::Result {
        let buf = OStylePrefix(&self.0).to_string();
        f.write_str(&buf)
    }
}

struct OStylePrefix<'a>(&'a OStyle);

impl Display for OStylePrefix<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt_prefix(f)
    }
}

struct UTF8Stdout(std::io::Stdout);

impl std::fmt::Write for UTF8Stdout {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        let mut buf = s.as_bytes();
        loop {
            let n = self.0.write(buf).map_err(|_| std::fmt::Error::default())?;
            if n == buf.len() {
                break;
            }

            buf = &buf[n..];
        }

        self.0.flush().map_err(|_| std::fmt::Error::default())?;

        Ok(())
    }
}
