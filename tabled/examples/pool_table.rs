use tabled::{
    settings::{style::Style, Alignment},
    tables::{PoolTable, TableValue},
};

fn main() {
    let data = TableValue::Column(vec![
        TableValue::Cell(String::from("independent cell")),
        TableValue::Row(vec![
            TableValue::Cell(String::from("row 1")),
            TableValue::Cell(String::from("row 2")),
            TableValue::Column(vec![
                TableValue::Cell(String::from("row 3 cell 1")),
                TableValue::Cell(String::from("row 3 cell 2")),
            ]),
        ]),
    ]);

    let mut table = PoolTable::from(data);
    table.with(Style::modern()).with(Alignment::center());

    println!("{table}");
}
