use tabled::{
    settings::{
        style::{HorizontalLine, Line, On, RawStyle},
        Border, Style,
    },
    Table,
};

type FullStyle = Style<On, On, On, On, On, On>;

const STYLE_1: FullStyle = Style::modern().frame(Style::rounded().get_frame());

const STYLE_2: FullStyle = Style::rounded()
    .remove_horizontals()
    .horizontal_line(Style::modern().get_horizontal_line());

fn main() {
    assert_eq!(RawStyle::from(STYLE_1), RawStyle::from(STYLE_2));

    let data = vec![("Hello", "world", "!"); 5];

    let mut table = Table::new(&data);
    table.with(STYLE_2);

    let output = table.to_string();

    println!("{output}");

    let mut table = Table::new(&data);
    table.with(STYLE_1);

    let output = table.to_string();

    println!("{output}");
}
