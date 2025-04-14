/// data source: https://www.britannica.com/science/physical-constant
use tabled::{
    settings::{
        object::{Columns, Object, Rows, Segment},
        style::BorderColor,
        Alignment, Color, Format, Margin, MarginColor, Padding, PaddingColor, Settings, Style,
    },
    Table, Tabled,
};

#[derive(Tabled)]
#[tabled(rename_all = "PascalCase")]
struct Fundamental {
    quantity: &'static str,
    value: &'static str,
    unit: &'static str,
    symbol: char,
}

fn main() {
    #[rustfmt::skip]
    let data = [
        Fundamental { quantity: "Constant of gravitation",      symbol: 'G', value: "6.67384 × 10⁻¹¹",      unit: "cubic metre per second squared per kilogram" },
        Fundamental { quantity: "Speed of light (in a vacuum)", symbol: 'c', value: "2.99792458 × 10⁻⁸",    unit: "metres per second" },
        Fundamental { quantity: "Planck's constant",            symbol: 'h', value: "6.626070040 × 10⁻³⁴",  unit: "joule second" },
        Fundamental { quantity: "Boltzmann constant",           symbol: 'k', value: "1.38064852 × 10⁻²³",   unit: "joule per kelvin" },
        Fundamental { quantity: "Faraday constant",             symbol: 'F', value: "9.648533289 × 10⁴",    unit: "coulombs per mole" },
    ];

    let mut table = Table::new(data);

    let pane_color = Color::rgb_bg(220, 220, 220);
    let border_color = Color::rgb_bg(200, 200, 220);
    let data_color = Color::rgb_bg(200, 200, 220);

    let header_settings = Settings::empty()
        .with(Padding::new(1, 1, 2, 2))
        .with(PaddingColor::new(
            Color::BG_GREEN,
            Color::BG_YELLOW,
            Color::BG_MAGENTA,
            Color::BG_CYAN,
        ))
        .with(Alignment::center())
        .with(Padding::expand(true))
        .with(Color::BG_BRIGHT_RED | Color::FG_BRIGHT_WHITE);

    let data_settings = Settings::empty()
        .with(Alignment::right())
        .with(Padding::expand(true))
        .with(PaddingColor::filled(data_color.clone()))
        .with(Color::FG_BLACK | data_color);

    let value_settings = Settings::empty().with(Format::content(|s| Color::BOLD.colorize(s)));

    let unit_settings = Settings::empty().with(Format::content(|s| Color::UNDERLINE.colorize(s)));

    table
        .with(Style::blank())
        .with(Margin::new(10, 1, 1, 1))
        .with(MarginColor::filled(pane_color))
        .modify(Segment::all(), BorderColor::filled(border_color))
        .modify(Rows::first(), header_settings)
        .modify(Rows::first().inverse(), data_settings)
        .modify(Columns::single(1).not(Rows::first()), value_settings)
        .modify(Columns::single(3).not(Rows::first()), unit_settings);

    println!("{table}");
}
