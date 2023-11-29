use tabled::{
    grid::config::Borders,
    settings::{
        style::{HorizontalLine, On, Style},
        Border,
    },
    Table,
};

const STYLE_1: Style<On, On, On, On, On, On, 0, 0> =
    Style::modern().frame(Border::inherit(Style::rounded()));

const STYLE_2: Style<On, On, On, On, On, On, 0, 0> = Style::rounded()
    .line_horizontal(HorizontalLine::inherit(Style::modern()))
    .remove_horizontals();

fn main() {
    assert_eq!(Borders::from(STYLE_1), Borders::from(STYLE_2));

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
