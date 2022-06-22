use tabled::{
    formatting_settings::{AlignmentStrategy, TabSize, TrimStrategy},
    object::{Cell, Segment},
    Alignment, Modify, Span, Style, Table,
};

use crate::util::{create_vector, static_table};

mod util;

#[test]
fn alignment_per_line() {
    let mut data = create_vector::<3, 3>();
    data[1][0] = String::from("asd\n21213123\n\n   asdasd\n\n");
    data[2][2] = String::from("https://\nwww\n.\nredhat\n.com\n/en");

    let table = Table::new(&data)
        .with(Style::psql())
        .with(
            Modify::new(Segment::all())
                .with(Alignment::right())
                .with(AlignmentStrategy::PerLine),
        )
        .to_string();

    assert_eq!(
        table,
        static_table!(
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
        )
    );
}

#[test]
fn alignment_per_line_with_trim() {
    let mut data = create_vector::<3, 3>();
    data[1][0] = String::from("asd\n21213123\n\n   asdasd\n\n");
    data[2][2] = String::from("https://\nwww\n.\nredhat\n.com\n/en");

    let table = Table::new(&data)
        .with(Style::psql())
        .with(
            Modify::new(Segment::all())
                .with(Alignment::right())
                .with(AlignmentStrategy::PerLine)
                .with(TrimStrategy::Horizontal),
        )
        .to_string();

    assert_eq!(
        table,
        static_table!(
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
        )
    );

    let mut data = create_vector::<3, 3>();
    data[1][0] = String::from("\n\n\nasd\n21213123   asdasd\n\n\n");
    data[2][2] = String::from("https://\nwww\n.\nredhat\n.com\n/en");

    let table = Table::new(&data)
        .with(Style::psql())
        .with(
            Modify::new(Segment::all())
                .with(Alignment::center_vertical())
                .with(Alignment::left())
                .with(AlignmentStrategy::PerLine)
                .with(TrimStrategy::Both),
        )
        .to_string();

    assert_eq!(
        table,
        static_table!(
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
        )
    );
}

#[test]
fn tab_size_test() {
    let mut data = create_vector::<3, 3>();
    data[1][0] = String::from("123\t123\tasdasd");
    data[2][2] = String::from("htt\tps://\nwww\n.\nred\that\n.c\tom\n/en");

    let mut table = Table::new(&data).with(Style::psql());

    assert_eq!(
        table.to_string(),
        static_table!(
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
        )
    );

    table = table
        .with(Modify::new(Segment::all()).with(Alignment::right()))
        .with(TabSize(2));

    assert_eq!(
        table.to_string(),
        static_table!(
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
        )
    );

    table = table
        .with(Modify::new(Segment::all()).with(Alignment::right()))
        .with(TabSize(0));

    assert_eq!(
        table.to_string(),
        static_table!(
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
        )
    );
}

#[test]
fn tab_size_span_test() {
    let mut data = create_vector::<3, 3>();
    data[0][0] = String::from("\tH\t\tello\tWorld");
    data[1][0] = String::from("123\t123\tasdasd");
    data[2][2] = String::from("htt\tps://\nwww\n.\nred\that\n.c\tom\n/en");

    let table = Table::new(&data)
        .with(Style::psql())
        .with(Modify::new(Cell(0, 0)).with(Span::column(3)))
        .with(Modify::new(Cell(1, 0)).with(Span::column(2)))
        .with(Modify::new(Cell(2, 1)).with(Span::column(2)));

    assert_eq!(
        table.to_string(),
        static_table!(
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
        )
    );
}

#[cfg(feature = "color")]
#[test]
fn trim_colored_string_test() {
    use owo_colors::OwoColorize;

    let mut data = create_vector::<3, 3>();
    data[1][0] = "asd\n21213123\n\n   asdasd\n\n".red().to_string();
    data[2][2] = "https://\nwww\n.\nredhat\n.com\n/en".on_blue().to_string();

    let table = Table::new(&data)
        .with(Style::psql())
        .with(
            Modify::new(Segment::all())
                .with(Alignment::right())
                .with(TrimStrategy::None),
        )
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "         N | column 0 | column 1 | column 2 "
            "-----------+----------+----------+----------"
            "         0 |      0-0 |      0-1 |      0-2 "
            " \u{1b}[31masd\u{1b}[39m       |      1-0 |      1-1 |      1-2 "
            " \u{1b}[31m21213123\u{1b}[39m  |          |          |          "
            " \u{1b}[31m\u{1b}[39m          |          |          |          "
            " \u{1b}[31m   asdasd\u{1b}[39m |          |          |          "
            " \u{1b}[31m\u{1b}[39m          |          |          |          "
            " \u{1b}[31m\u{1b}[39m          |          |          |          "
            "         2 |      2-0 | \u{1b}[44mhttps://\u{1b}[49m |      2-2 "
            "           |          | \u{1b}[44mwww\u{1b}[49m      |          "
            "           |          | \u{1b}[44m.\u{1b}[49m        |          "
            "           |          | \u{1b}[44mredhat\u{1b}[49m   |          "
            "           |          | \u{1b}[44m.com\u{1b}[49m     |          "
            "           |          | \u{1b}[44m/en\u{1b}[49m      |          "
        )
    );

    let table = Table::new(&data)
        .with(Style::psql())
        .with(
            Modify::new(Segment::all())
                .with(Alignment::right())
                .with(TrimStrategy::Horizontal)
                .with(AlignmentStrategy::PerLine),
        )
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "         N | column 0 | column 1 | column 2 "
            "-----------+----------+----------+----------"
            "         0 |      0-0 |      0-1 |      0-2 "
            "       \u{1b}[31masd\u{1b}[39m |      1-0 |      1-1 |      1-2 "
            "  \u{1b}[31m21213123\u{1b}[39m |          |          |          "
            "          \u{1b}[31m\u{1b}[39m |          |          |          "
            "    \u{1b}[31masdasd\u{1b}[39m |          |          |          "
            "          \u{1b}[31m\u{1b}[39m |          |          |          "
            "          \u{1b}[31m\u{1b}[39m |          |          |          "
            "         2 |      2-0 | \u{1b}[44mhttps://\u{1b}[49m |      2-2 "
            "           |          |      \u{1b}[44mwww\u{1b}[49m |          "
            "           |          |        \u{1b}[44m.\u{1b}[49m |          "
            "           |          |   \u{1b}[44mredhat\u{1b}[49m |          "
            "           |          |     \u{1b}[44m.com\u{1b}[49m |          "
            "           |          |      \u{1b}[44m/en\u{1b}[49m |          "
        )
    );

    let table = Table::new(&data)
        .with(Style::psql())
        .with(
            Modify::new(Segment::all())
                .with(Alignment::right())
                .with(TrimStrategy::Horizontal),
        )
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "         N | column 0 | column 1 | column 2 "
            "-----------+----------+----------+----------"
            "         0 |      0-0 |      0-1 |      0-2 "
            " \u{1b}[31masd\u{1b}[39m       |      1-0 |      1-1 |      1-2 "
            " \u{1b}[31m21213123\u{1b}[39m  |          |          |          "
            " \u{1b}[31m\u{1b}[39m          |          |          |          "
            " \u{1b}[31masdasd\u{1b}[39m    |          |          |          "
            " \u{1b}[31m\u{1b}[39m          |          |          |          "
            " \u{1b}[31m\u{1b}[39m          |          |          |          "
            "         2 |      2-0 | \u{1b}[44mhttps://\u{1b}[49m |      2-2 "
            "           |          | \u{1b}[44mwww\u{1b}[49m      |          "
            "           |          | \u{1b}[44m.\u{1b}[49m        |          "
            "           |          | \u{1b}[44mredhat\u{1b}[49m   |          "
            "           |          | \u{1b}[44m.com\u{1b}[49m     |          "
            "           |          | \u{1b}[44m/en\u{1b}[49m      |          "
        )
    );
}

#[test]
fn test_top_alignment_and_vertical_trim_1() {
    let table = Table::new(&["   \n\n\n    Hello World"])
        .with(Style::modern())
        .with(
            Modify::new(Segment::all())
                .with(Alignment::top())
                .with(TrimStrategy::Vertical),
        );

    assert_eq!(
        table.to_string(),
        static_table!(
            "┌─────────────────┐"
            "│      &str       │"
            "├─────────────────┤"
            "│     Hello World │"
            "│                 │"
            "│                 │"
            "│                 │"
            "└─────────────────┘"
        )
    );
}
