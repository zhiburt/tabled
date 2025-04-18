use std::iter::FromIterator;

use tabled::{
    settings::{object::Rows, style::Style, themes::Colorization, Color, Concat},
    Table, Tabled,
};

#[derive(Tabled)]
#[tabled(rename_all = "Upper Title Case")]
struct Employee {
    id: usize,
    first_name: String,
    last_name: String,
    #[tabled(rename = "$")]
    salary: usize,
    comment: String,
}

fn main() {
    #[rustfmt::skip]
    let data = vec![
        Employee { id: 1,   first_name: String::from("Arya"),   last_name: String::from("Stark"),     salary: 3000, comment: String::from("") },
        Employee { id: 20,  first_name: String::from("Jon"),    last_name: String::from("Snow"),      salary: 2000, comment: String::from("You know nothing, Jon Snow!") },
        Employee { id: 300, first_name: String::from("Tyrion"), last_name: String::from("Lannister"), salary: 5000, comment: String::from("") },
        Employee { id: 300, first_name: String::from("Jaime"),  last_name: String::from("Lannister"), salary: 5000, comment: String::from("If there are gods, why is the world so full of pain and injustice?") },
    ];

    let total = data.iter().map(|e| e.salary).sum::<usize>();
    let total_row = Table::from_iter([[
        String::new(),
        String::new(),
        String::from("TOTAL"),
        total.to_string(),
    ]]);

    let clr_primary = Color::BG_WHITE | Color::FG_BLACK;
    let clr_secondary = Color::BG_BRIGHT_BLACK | Color::FG_WHITE;
    let clr_head = Color::BG_CYAN | Color::FG_BLACK | Color::BOLD;
    let clr_footer = Color::BG_BLUE | Color::FG_BLACK | Color::BOLD;

    let mut table = Table::new(data);
    table
        .with(Concat::vertical(total_row))
        .with(Style::empty())
        .with(Colorization::rows([clr_primary, clr_secondary]))
        .with(Colorization::exact([clr_head], Rows::first()))
        .with(Colorization::exact([clr_footer], Rows::last()));

    println!("{table}");
}
