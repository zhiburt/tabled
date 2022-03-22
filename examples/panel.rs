//! The example can be run by this command
//! `cargo run --example panel`

use tabled::{
    object::Full, Alignment, AlignmentHorizontal, Footer, Header, Modify, Style, Table, Tabled,
};

#[derive(Tabled)]
struct Release {
    version: &'static str,
    published_date: &'static str,
    is_active: bool,
    major_feature: &'static str,
}

const DATA: [Release; 3] = [
    Release {
        version: "0.2.1",
        published_date: "2021-06-23",
        is_active: true,
        major_feature: "#[header(inline)] attribute",
    },
    Release {
        version: "0.2.0",
        published_date: "2021-06-19",
        is_active: false,
        major_feature: "API changes",
    },
    Release {
        version: "0.1.4",
        published_date: "2021-06-07",
        is_active: false,
        major_feature: "display_with attribute",
    },
];

fn main() {
    let table = Table::new(DATA)
        .with(Header("Tabled Releases"))
        .with(Footer(format!("N - {}", DATA.len())))
        .with(Modify::new(Full).with(Alignment::Horizontal(AlignmentHorizontal::Center)))
        .with(
            Style::modern()
                .top_intersection('─')
                .bottom_intersection('─')
                .header_intersection('┬'),
        );

    println!("{}", table);
}
