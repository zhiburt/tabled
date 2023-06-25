#![cfg(feature = "std")]

use tabled::settings::{
    formatting::{AlignmentStrategy, TabSize, TrimStrategy},
    object::Segment,
    Alignment, Modify, Span, Style,
};

use crate::matrix::{Matrix, MatrixList};
use testing_table::test_table;

#[cfg(feature = "color")]
use owo_colors::OwoColorize;

test_table!(
    alignment_per_line,
    Matrix::iter(multiline_data1())
        .with(Style::psql())
        .with(Modify::new(Segment::all()).with(Alignment::right()).with(AlignmentStrategy::PerLine)),
    "         N | column 0 | column 1 | column 2 "
    "-----------+----------+----------+----------"
    "         0 |      0-0 |      0-1 |      0-2 "
    "       asd |      1-0 |      1-1 |      1-2 "
    "  21213123 |          |          |          "
    "           |          |          |          "
    "    asdasd |          |          |          "
    "           |          |          |          "
    "           |          |          |          "
    "         2 |      2-0 | https:// |      2-2 "
    "           |          |      www |          "
    "           |          |        . |          "
    "           |          |   redhat |          "
    "           |          |     .com |          "
    "           |          |      /en |          "
);

test_table!(
    alignment_per_line_with_trim_0,
    Matrix::iter(multiline_data1())
        .with(Style::psql())
        .with(Alignment::right())
        .with(AlignmentStrategy::PerLine)
        .with(TrimStrategy::Horizontal),
    "         N | column 0 | column 1 | column 2 "
    "-----------+----------+----------+----------"
    "         0 |      0-0 |      0-1 |      0-2 "
    "       asd |      1-0 |      1-1 |      1-2 "
    "  21213123 |          |          |          "
    "           |          |          |          "
    "    asdasd |          |          |          "
    "           |          |          |          "
    "           |          |          |          "
    "         2 |      2-0 | https:// |      2-2 "
    "           |          |      www |          "
    "           |          |        . |          "
    "           |          |   redhat |          "
    "           |          |     .com |          "
    "           |          |      /en |          "
);

test_table!(
    alignment_per_line_with_trim_1,
    Matrix::iter(multiline_data2())
        .with(Style::psql())
        .with(Modify::new(Segment::all())
            .with(Alignment::center_vertical())
            .with(Alignment::left())
            .with(AlignmentStrategy::PerLine)
            .with(TrimStrategy::Both)),
    " N                 | column 0 | column 1 | column 2 "
    "-------------------+----------+----------+----------"
    " 0                 | 0-0      | 0-1      | 0-2      "
    "                   |          |          |          "
    "                   |          |          |          "
    "                   |          |          |          "
    " asd               | 1-0      | 1-1      | 1-2      "
    " 21213123   asdasd |          |          |          "
    "                   |          |          |          "
    "                   |          |          |          "
    "                   |          |          |          "
    "                   |          | https:// |          "
    "                   |          | www      |          "
    " 2                 | 2-0      | .        | 2-2      "
    "                   |          | redhat   |          "
    "                   |          | .com     |          "
    "                   |          | /en      |          "
);

test_table!(
    tab_isnot_handled_by_default_test,
    Matrix::iter(tab_data1()).with(Style::psql()),
    "      N       | column 0 | column 1 | column 2 "
    "--------------+----------+----------+----------"
    "      0       |   0-0    |   0-1    |   0-2    "
    " 123\t123\tasdasd |   1-0    |   1-1    |   1-2    "
    "      2       |   2-0    | htt\tps:// |   2-2    "
    "              |          | www      |          "
    "              |          | .        |          "
    "              |          | red\that   |          "
    "              |          | .c\tom     |          "
    "              |          | /en      |          "
);

test_table!(
    tab_size_test_0,
    Matrix::iter(tab_data1()).with(Style::psql()).with(TabSize::new(4)),
    "          N           | column 0 |   column 1   | column 2 "
    "----------------------+----------+--------------+----------"
    "          0           |   0-0    |     0-1      |   0-2    "
    " 123    123    asdasd |   1-0    |     1-1      |   1-2    "
    "          2           |   2-0    | htt    ps:// |   2-2    "
    "                      |          | www          |          "
    "                      |          | .            |          "
    "                      |          | red    hat   |          "
    "                      |          | .c    om     |          "
    "                      |          | /en          |          "
);

test_table!(
    tab_size_test_1,
    Matrix::iter(tab_data1()).with(Style::psql()).with(Modify::new(Segment::all()).with(Alignment::right())).with(TabSize::new(2)),
    "                N | column 0 |   column 1 | column 2 "
    "------------------+----------+------------+----------"
    "                0 |      0-0 |        0-1 |      0-2 "
    " 123  123  asdasd |      1-0 |        1-1 |      1-2 "
    "                2 |      2-0 | htt  ps:// |      2-2 "
    "                  |          | www        |          "
    "                  |          | .          |          "
    "                  |          | red  hat   |          "
    "                  |          | .c  om     |          "
    "                  |          | /en        |          "
);

test_table!(
    tab_size_test_2,
    Matrix::iter(tab_data1()).with(Style::psql()).with(Modify::new(Segment::all()).with(Alignment::right())).with(TabSize::new(0)),
    "            N | column 0 | column 1 | column 2 "
    "--------------+----------+----------+----------"
    "            0 |      0-0 |      0-1 |      0-2 "
    " 123123asdasd |      1-0 |      1-1 |      1-2 "
    "            2 |      2-0 | https:// |      2-2 "
    "              |          | www      |          "
    "              |          | .        |          "
    "              |          | redhat   |          "
    "              |          | .com     |          "
    "              |          | /en      |          "
);

test_table!(
    tab_size_span_test,
    Matrix::iter(tab_data2())
        .with(TabSize::new(4))
        .with(Style::psql())
        .with(Modify::new((0, 0)).with(Span::column(3)))
        .with(Modify::new((1, 0)).with(Span::column(2)))
        .with(Modify::new((2, 1)).with(Span::column(2))),
    "                     N                     | column 2 "
    "----------------------+-----+--------------+----------"
    "     H        ello    World |     0-1      |   0-2    "
    " 123    123    asdasd |        1-0         |   1-2    "
    "          2           | 2-0 | htt    ps:// |   2-2    "
    "                      |     | www          |          "
    "                      |     | .            |          "
    "                      |     | red    hat   |          "
    "                      |     | .c    om     |          "
    "                      |     | /en          |          "
);

test_table!(
    test_top_alignment_and_vertical_trim_1,
    Matrix::iter(vec!["   \n\n\n    Hello World"])
        .with(Style::modern())
        .with(Modify::new(Segment::all()).with(Alignment::top()).with(TrimStrategy::Vertical)),
    "┌─────────────────┐"
    "│      &str       │"
    "├─────────────────┤"
    "│     Hello World │"
    "│                 │"
    "│                 │"
    "│                 │"
    "└─────────────────┘"
);

#[cfg(feature = "color")]
test_table!(
    trim_colored_string_test_2,
    Matrix::iter(colored_data())
        .with(Style::psql())
        .with(Modify::new(Segment::all()).with(Alignment::right()).with(TrimStrategy::None)),
    "         N | column 0 | column 1 | column 2 "
    "-----------+----------+----------+----------"
    "         0 |      0-0 |      0-1 |      0-2 "
    " \u{1b}[31masd\u{1b}[39m       |      1-0 |      1-1 |      1-2 "
    " \u{1b}[31m21213123\u{1b}[39m  |          |          |          "
    "           |          |          |          "
    " \u{1b}[31m   asdasd\u{1b}[39m |          |          |          "
    "           |          |          |          "
    " \u{1b}[31m\u{1b}[39m          |          |          |          "
    "         2 |      2-0 | \u{1b}[44mhttps://\u{1b}[49m |      2-2 "
    "           |          | \u{1b}[44mwww\u{1b}[49m      |          "
    "           |          | \u{1b}[44m.\u{1b}[49m        |          "
    "           |          | \u{1b}[44mredhat\u{1b}[49m   |          "
    "           |          | \u{1b}[44m.com\u{1b}[49m     |          "
    "           |          | \u{1b}[44m/en\u{1b}[49m      |          "
);

#[cfg(feature = "color")]
test_table!(
    trim_colored_string_test_1,
    Matrix::iter(colored_data())
        .with(Style::psql())
        .with(
            Modify::new(Segment::all())
                .with(Alignment::right())
                .with(TrimStrategy::Horizontal)
                .with(AlignmentStrategy::PerLine),
        ),
    "         N | column 0 | column 1 | column 2 "
    "-----------+----------+----------+----------"
    "         0 |      0-0 |      0-1 |      0-2 "
    "       \u{1b}[31masd\u{1b}[39m |      1-0 |      1-1 |      1-2 "
    "  \u{1b}[31m21213123\u{1b}[39m |          |          |          "
    "           |          |          |          "
    "    \u{1b}[31masdasd\u{1b}[39m |          |          |          "
    "           |          |          |          "
    "          \u{1b}[31m\u{1b}[39m |          |          |          "
    "         2 |      2-0 | \u{1b}[44mhttps://\u{1b}[49m |      2-2 "
    "           |          |      \u{1b}[44mwww\u{1b}[49m |          "
    "           |          |        \u{1b}[44m.\u{1b}[49m |          "
    "           |          |   \u{1b}[44mredhat\u{1b}[49m |          "
    "           |          |     \u{1b}[44m.com\u{1b}[49m |          "
    "           |          |      \u{1b}[44m/en\u{1b}[49m |          "
);
#[cfg(feature = "color")]
test_table!(
    trim_colored_string_test_0,
    Matrix::iter(colored_data())
        .with(Style::psql())
        .with(Modify::new(Segment::all()).with(Alignment::right()).with(TrimStrategy::Horizontal)),
    "         N | column 0 | column 1 | column 2 "
    "-----------+----------+----------+----------"
    "         0 |      0-0 |      0-1 |      0-2 "
    " \u{1b}[31masd\u{1b}[39m       |      1-0 |      1-1 |      1-2 "
    " \u{1b}[31m21213123\u{1b}[39m  |          |          |          "
    "           |          |          |          "
    " \u{1b}[31masdasd\u{1b}[39m    |          |          |          "
    "           |          |          |          "
    " \u{1b}[31m\u{1b}[39m          |          |          |          "
    "         2 |      2-0 | \u{1b}[44mhttps://\u{1b}[49m |      2-2 "
    "           |          | \u{1b}[44mwww\u{1b}[49m      |          "
    "           |          | \u{1b}[44m.\u{1b}[49m        |          "
    "           |          | \u{1b}[44mredhat\u{1b}[49m   |          "
    "           |          | \u{1b}[44m.com\u{1b}[49m     |          "
    "           |          | \u{1b}[44m/en\u{1b}[49m      |          "
);

fn multiline_data1() -> Vec<MatrixList<3, true>> {
    let mut data = Matrix::list::<3, 3>();
    data[1][0] = String::from("asd\n21213123\n\n   asdasd\n\n");
    data[2][2] = String::from("https://\nwww\n.\nredhat\n.com\n/en");
    data
}

fn multiline_data2() -> Vec<MatrixList<3, true>> {
    let mut data = Matrix::list::<3, 3>();
    data[1][0] = String::from("\n\n\nasd\n21213123   asdasd\n\n\n");
    data[2][2] = String::from("https://\nwww\n.\nredhat\n.com\n/en");
    data
}

fn tab_data1() -> Vec<MatrixList<3, true>> {
    let mut data = Matrix::list::<3, 3>();
    data[1][0] = String::from("123\t123\tasdasd");
    data[2][2] = String::from("htt\tps://\nwww\n.\nred\that\n.c\tom\n/en");
    data
}

fn tab_data2() -> Vec<MatrixList<3, true>> {
    let mut data = Matrix::list::<3, 3>();
    data[0][0] = String::from("\tH\t\tello\tWorld");
    data[1][0] = String::from("123\t123\tasdasd");
    data[2][2] = String::from("htt\tps://\nwww\n.\nred\that\n.c\tom\n/en");
    data
}

#[cfg(feature = "color")]
fn colored_data() -> Vec<MatrixList<3, true>> {
    let mut data = Matrix::list::<3, 3>();
    data[1][0] = "asd\n21213123\n\n   asdasd\n\n".red().to_string();
    data[2][2] = "https://\nwww\n.\nredhat\n.com\n/en".on_blue().to_string();
    data
}
