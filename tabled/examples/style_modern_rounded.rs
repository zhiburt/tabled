use tabled::{
    settings::{
        style::{HorizontalLine, Line, On},
        Border, Style,
    },
    Table,
};

type FullStyle = Style<On, On, On, On, On, On>;

const ROUND_FRAME: Border = Style::rounded().get_frame();

const STYLE_BY_MODERN: FullStyle = Style::modern()
    .corner_top_left(unwrap(ROUND_FRAME.get_corner_top_left()))
    .corner_top_right(unwrap(ROUND_FRAME.get_corner_top_right()))
    .corner_bottom_left(unwrap(ROUND_FRAME.get_corner_bottom_left()))
    .corner_bottom_right(unwrap(ROUND_FRAME.get_corner_bottom_right()));

const MODERN_SPLIT_LINE: HorizontalLine = Style::modern().get_horizontal();

const STYLE_BY_ROUNDED: FullStyle = Style::rounded()
    .remove_horizontals()
    .horizontal(unwrap(Style::modern().get_frame().get_top()))
    .intersection_left(unwrap(Style::modern().get_horizontal().get_top()))
    .intersection_right(unwrap(Style::modern().get_frame().get_top()));

const fn unwrap(opt: Option<char>) -> char {
    match opt {
        Some(c) => c,
        None => unreachable!(),
    }
}

fn main() {
    let data = vec![("Hello", "world", "!"); 5];
    let mut table = Table::new(data);
    table.with(MODERN_ROUNDED_1);

    let output = table.to_string();
    let expected = concat!(
        "╭───────┬───────┬──────╮\n",
        "│ &str  │ &str  │ &str │\n",
        "├───────┼───────┼──────┤\n",
        "│ Hello │ world │ !    │\n",
        "├───────┼───────┼──────┤\n",
        "│ Hello │ world │ !    │\n",
        "├───────┼───────┼──────┤\n",
        "│ Hello │ world │ !    │\n",
        "├───────┼───────┼──────┤\n",
        "│ Hello │ world │ !    │\n",
        "├───────┼───────┼──────┤\n",
        "│ Hello │ world │ !    │\n",
        "╰───────┴───────┴──────╯",
    );

    assert_eq!(output, expected);
}
