//! The example can be run by this command.
//!
//! `cargo run --features color --example padding_color`
//!
//! This example requires a `color` feature.

use std::convert::TryFrom;

use owo_colors::OwoColorize;

use tabled::{
    grid::{
        config::Entity,
        spanned::{ExactDimension, GridConfig},
        util::string::string_width_multiline,
    },
    records::{ExactRecords, Records, VecRecords},
    settings::{
        alignment::Alignment,
        color::Color,
        format::Format,
        margin::{Margin, MarginColor},
        object::{Columns, Object, Rows, Segment},
        padding::{Padding, PaddingColor},
        style::Style,
        CellOption, Modify,
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
        .with(Padding::new(1, 1, 2, 2))
        .with(MakeMaxPadding)
        .with(PaddingColor::new(
            Color::BG_GREEN,
            Color::BG_YELLOW,
            Color::BG_MAGENTA,
            Color::BG_CYAN,
        ))
        .with(Format::content(|s| s.on_black().white().to_string()));

    let data_settings = Modify::new(Rows::first().inverse())
        .with(Alignment::left())
        .with(MakeMaxPadding)
        .with(PaddingColor::new(
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
        .with(Margin::new(1, 2, 1, 1))
        .with(MarginColor::new(
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

struct MakeMaxPadding;

impl<T> CellOption<VecRecords<T>, GridConfig> for MakeMaxPadding
where
    T: AsRef<str>,
{
    fn change(&mut self, records: &mut VecRecords<T>, cfg: &mut GridConfig, entity: Entity) {
        let widths = ExactDimension::width(&*records, cfg);

        let count_rows = records.count_rows();
        let count_cols = records.count_columns();

        for (row, col) in entity.iter(count_rows, count_cols) {
            let column_width = widths[col];
            let text = records.get_cell((row, col)).as_ref();
            let width = string_width_multiline(text);

            if width < column_width {
                let available_width = column_width - width;
                let left = available_width / 2;
                let right = available_width - left;

                let pos = (row, col).into();
                let mut padding = *cfg.get_padding(pos);
                padding.left.size = left;
                padding.right.size = right;

                cfg.set_padding(pos, padding);
            }
        }
    }
}
