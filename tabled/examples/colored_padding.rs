//! This example demonstrates using the [`Padding::colorize()`] function in several ways
//! to give a [`Table`] display a vibrant asthetic.
//!
//! * 🚩 This example requires the `color` feature.
//!
//! * Note how the [`Color`] [setting](tabled::settings) is used to simplify creating
//! reusable themes for text, backgrounds, padded whitespace, and borders.
//!
//! * Note how a unique color can be set for each direction.

use std::convert::TryFrom;

use owo_colors::OwoColorize;

use tabled::{
    grid::{
        config::{ColoredConfig, Entity},
        dimension::SpannedGridDimension,
        records::{
            vec_records::{Cell, VecRecords},
            ExactRecords, PeekableRecords, Records,
        },
        util::string::string_width_multiline,
    },
    settings::{
        object::{Columns, Object, Rows, Segment},
        Alignment, CellOption, Color, Format, Margin, Modify, Padding, Style,
    },
    Table, Tabled,
};

#[derive(Tabled)]
#[tabled(rename_all = "PascalCase")]
struct Fundamental {
    quantity: &'static str,
    symbol: &'static str,
    value: &'static str,
    unit: &'static str,
}

impl Fundamental {
    fn new(
        quantity: &'static str,
        symbol: &'static str,
        value: &'static str,
        unit: &'static str,
    ) -> Self {
        Self {
            quantity,
            symbol,
            value,
            unit,
        }
    }
}

fn main() {
    // data source: https://www.britannica.com/science/physical-constant
    let data = [
        Fundamental::new(
            "constant of gravitation",
            "G",
            "6.67384 × 10⁻¹¹",
            "cubic metre per second squared per kilogram",
        ),
        Fundamental::new(
            "speed of light (in a vacuum)",
            "c",
            "2.99792458 × 10⁻⁸",
            "metres per second",
        ),
        Fundamental::new(
            "Planck's constant",
            "h",
            "6.626070040 × 10⁻³⁴",
            "joule second",
        ),
        Fundamental::new(
            "Boltzmann constant",
            "k",
            "1.38064852 × 10⁻²³",
            "joule per kelvin",
        ),
        Fundamental::new(
            "Faraday constant",
            "F",
            "9.648533289 × 10⁴",
            "coulombs per mole",
        ),
    ];

    let pane_color = Color::try_from(' '.bg_rgb::<220, 220, 220>().to_string()).unwrap();
    let border_color = Color::try_from(' '.bg_rgb::<200, 200, 220>().bold().to_string()).unwrap();
    let data_color = Color::try_from(' '.bg_rgb::<200, 200, 220>().to_string()).unwrap();

    let header_settings = Modify::new(Rows::first())
        .with(Padding::new(1, 1, 2, 2).colorize(
            Color::BG_GREEN,
            Color::BG_YELLOW,
            Color::BG_MAGENTA,
            Color::BG_CYAN,
        ))
        .with(MakeMaxPadding)
        .with(Format::content(|s| s.on_black().white().to_string()));

    let data_settings = Modify::new(Rows::first().inverse())
        .with(Alignment::left())
        .with(MakeMaxPadding)
        .with(Padding::new(1, 1, 0, 0).colorize(
            Color::default(),
            Color::default(),
            data_color.clone(),
            data_color.clone(),
        ));

    let symbol_settings = Modify::new(Columns::single(1).not(Rows::first()))
        .with(Format::content(|s| s.bold().to_string()));

    let unit_settings = Modify::new(Columns::single(3).not(Rows::first()))
        .with(Format::content(|s| s.italic().to_string()));

    let table = Table::new(data)
        .with(Style::rounded())
        .with(Margin::new(1, 2, 1, 1).colorize(
            pane_color.clone(),
            pane_color.clone(),
            pane_color.clone(),
            pane_color,
        ))
        .with(border_color)
        .with(Modify::new(Segment::all()).with(data_color))
        .with(header_settings)
        .with(data_settings)
        .with(symbol_settings)
        .with(unit_settings)
        .to_string();

    println!("\n\n{table}\n\n");
}

#[derive(Debug, Clone)]
struct MakeMaxPadding;

impl<T> CellOption<VecRecords<T>, ColoredConfig> for MakeMaxPadding
where
    T: Cell + AsRef<str>,
{
    fn change(self, records: &mut VecRecords<T>, cfg: &mut ColoredConfig, entity: Entity) {
        let widths = SpannedGridDimension::width(&*records, cfg);

        let count_rows = records.count_rows();
        let count_cols = records.count_columns();

        for (row, col) in entity.iter(count_rows, count_cols) {
            let column_width = widths[col];
            let text = records.get_text((row, col));
            let width = string_width_multiline(text);

            if width < column_width {
                let available_width = column_width - width;
                let left = available_width / 2;
                let right = available_width - left;

                let pos = (row, col).into();
                let mut pad = cfg.get_padding(pos);
                pad.left.size = left;
                pad.right.size = right;

                cfg.set_padding(pos, pad);
            }
        }
    }
}
