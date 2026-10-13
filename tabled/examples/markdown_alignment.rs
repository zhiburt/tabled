use tabled::{
    grid::config::Offset,
    settings::{object::Columns, style::LineChar, Alignment, Style},
    Table, Tabled,
};

#[derive(Tabled)]
struct Package {
    name: &'static str,
    version: &'static str,
    downloads: usize,
}

fn main() {
    let packages = [
        Package {
            name: "tabled",
            version: "0.22",
            downloads: 1250,
        },
        Package {
            name: "papergrid",
            version: "0.18",
            downloads: 860,
        },
    ];

    let mut table = Table::new(packages);
    table.with(Style::markdown());

    // Markdown reads alignment from colons in the separator row.
    table.modify(
        Columns::one(0),
        (
            Alignment::left(),
            LineChar::horizontal(':', Offset::Start(0)),
        ),
    );
    table.modify(
        Columns::one(1),
        (
            Alignment::center(),
            LineChar::horizontal(':', Offset::Start(0)),
            LineChar::horizontal(':', Offset::End(0)),
        ),
    );
    table.modify(
        Columns::one(2),
        (
            Alignment::right(),
            LineChar::horizontal(':', Offset::End(0)),
        ),
    );

    println!("{table}");
}
