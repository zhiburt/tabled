use tabled::{
    settings::{
        object::{Columns, Object, Rows},
        style::Style,
        Highlight,
    },
    Table,
};

fn main() {
    let data = vec![["A", "B", "C"], ["D", "E", "F"], ["G", "H", "I"]];

    let target = Columns::first()
        .not(Rows::last())
        .and(Rows::last() - 1)
        .and(Rows::last().intersect(Columns::last()));

    let mut table = Table::new(data);
    table.with(Style::modern());
    table.with(Highlight::outline(target, '*'));

    println!("{table}");
}
