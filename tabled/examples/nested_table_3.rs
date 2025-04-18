use tabled::{
    settings::{
        object::Rows, style::Style, themes::BorderCorrection, Alignment, Extract, Highlight,
        Padding, Panel,
    },
    Table, Tabled,
};

#[derive(Tabled)]
struct Contribution<'a> {
    author: &'a str,
    profile: &'a str,
}

impl<'a> Contribution<'a> {
    fn new(author: &'a str, profile: &'a str) -> Self {
        Self { author, profile }
    }
}

fn main() {
    let committers = [
        Contribution::new("kozmod", "https:/github.com/kozmod"),
        Contribution::new("IsaacCloos", "https:/github.com/IsaacCloos"),
    ];

    let issuers = [Contribution::new(
        "aharpervc",
        "https:/github.com/aharpervc",
    )];

    let committers_table = Table::new(committers)
        .with(Panel::header("Contributors"))
        .with(Alignment::center())
        .with(BorderCorrection::span())
        .to_string();

    let issues_table = Table::new(issuers)
        .with(Panel::header("Issuers"))
        .with(Alignment::center())
        .with(BorderCorrection::span())
        .to_string();

    let mut welcome_table = Table::new([(committers_table, issues_table)]);
    welcome_table
        .with(Extract::rows(1..))
        .with(Panel::header("Thank You"))
        .with(Style::ascii().remove_horizontal())
        .modify(Rows::new(1..), Padding::new(1, 1, 1, 0))
        .with(Alignment::center())
        .with(Highlight::outline(Rows::first(), '*'));

    println!("{welcome_table}");
}
