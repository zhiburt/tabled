use tabled::{
    settings::{
        style::{HorizontalLine, On, Style, StyleBuilder},
        Border,
    },
    Table,
};

const STYLE_1: StyleBuilder<On, On, On, On, On, On, 0, 0> =
    StyleBuilder::modern().frame(Border::inherit(StyleBuilder::rounded()));

const STYLE_2: StyleBuilder<On, On, On, On, On, On, 0, 0> = StyleBuilder::rounded()
    .line_horizontal(HorizontalLine::inherit(StyleBuilder::modern()))
    .remove_horizontals();

fn main() {
    assert_eq!(Style::from(STYLE_1), Style::from(STYLE_2));

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
