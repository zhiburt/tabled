//! This example demonstrates using the [`Color`] [setting](tabled::settings) to
//! stylize text, backgrounds, and borders.
//!
//! * 🚩 This example requires the `color` feature.
//!
//! * Note how [`Format::content()`] is used to break out [`CellOption`]
//! specifications. This is helpful for organizing extensive [`Table`] configurations.

use tabled::{
    builder::Builder,
    settings::{object::Rows, style::Style, themes::Colorization, Color, Concat},
    Table, Tabled,
};

#[derive(Tabled)]
#[tabled(rename_all = "UPPERCASE")]
struct Employee {
    id: usize,
    #[tabled(rename = "FIRST NAME")]
    first_name: String,
    #[tabled(rename = "LAST NAME")]
    last_name: String,
    salary: usize,
    comment: String,
}

impl Employee {
    fn new(id: usize, first_name: &str, last_name: &str, salary: usize, comment: &str) -> Self {
        Self {
            id,
            salary,
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
            comment: comment.to_string(),
        }
    }
}

fn main() {
    let data = vec![
        Employee::new(1, "Arya", "Stark", 3000, ""),
        Employee::new(20, "Jon", "Snow", 2000, "You know nothing, Jon Snow!"),
        Employee::new(300, "Tyrion", "Lannister", 5000, ""),
    ];

    let total = data.iter().map(|e| e.salary).sum::<usize>();
    let total_row = Builder::from(vec![vec![
        String::from(""),
        String::from(""),
        String::from("TOTAL"),
        total.to_string(),
    ]])
    .build();

    let color_data_primary = Color::BG_WHITE | Color::FG_BLACK;
    let color_data_second = Color::BG_BRIGHT_WHITE | Color::FG_BLACK;
    let color_head = Color::BOLD | Color::BG_CYAN | Color::FG_BLACK;
    let color_footer = Color::BOLD | Color::BG_BLUE | Color::FG_BLACK;

    let mut table = Table::new(data);
    table
        .with(Concat::vertical(total_row))
        .with(Style::empty())
        .with(Colorization::rows([color_data_primary, color_data_second]))
        .with(Colorization::exact([color_head], Rows::first()))
        .with(Colorization::exact([color_footer], Rows::last()));

    println!("{table}");
}
