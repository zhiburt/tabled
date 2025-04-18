use tabled::{
    settings::{
        style::{HorizontalLine, On, Style},
        Border,
    },
    Table,
};

const STYLE_1: Style<On, On, On, On, On, On, 0, 0> =
    Style::modern().frame(Border::inherit(Style::rounded()));

const STYLE_2: Style<On, On, On, On, (), On, 1, 0> = Style::rounded()
    .remove_horizontals()
    .horizontals([(1, HorizontalLine::inherit(Style::modern()))]);

fn main() {
    let data = vec![("Hello", "world", "!"); 3];

    let mut table1 = Table::new(&data);
    table1.with(STYLE_2);

    let mut table2 = Table::new(&data);
    table2.with(STYLE_1);

    let output1 = table1.to_string();
    let output2 = table2.to_string();
    let output = Table::new([(output1, output2)]);

    println!("{}", output);
}
