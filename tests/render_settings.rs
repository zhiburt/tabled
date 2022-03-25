use crate::util::create_vector;
use tabled::{
    object::Full,
    render_settings::{AlignmentStrategy, RenderSettings, TrimStrategy},
    Alignment, Modify, Style, Table,
};

mod util;

#[test]
fn alignment_per_line() {
    let mut data = create_vector::<3, 3>();
    data[1][0] = String::from("asd\n21213123\n\n   asdasd\n\n");
    data[2][2] = String::from("https://\nwww\n.\nredhat\n.com\n/en");

    let table = Table::new(&data)
        .with(Style::psql())
        .with(
            Modify::new(Full)
                .with(Alignment::right())
                .with(RenderSettings::default().alignement(AlignmentStrategy::PerLine)),
        )
        .to_string();

    let expected = concat!(
        "         N | column 0 | column 1 | column 2 \n",
        "-----------+----------+----------+----------\n",
        "         0 |      0-0 |      0-1 |      0-2 \n",
        "       asd |      1-0 |      1-1 |      1-2 \n",
        "  21213123 |          |          |          \n",
        "           |          |          |          \n",
        "    asdasd |          |          |          \n",
        "           |          |          |          \n",
        "         2 |      2-0 | https:// |      2-2 \n",
        "           |          |      www |          \n",
        "           |          |        . |          \n",
        "           |          |   redhat |          \n",
        "           |          |     .com |          \n",
        "           |          |      /en |          \n",
    );

    assert_eq!(table, expected);
}

#[test]
fn alignment_per_line_with_trim() {
    let mut data = create_vector::<3, 3>();
    data[1][0] = String::from("asd\n21213123\n\n   asdasd\n\n");
    data[2][2] = String::from("https://\nwww\n.\nredhat\n.com\n/en");

    let table = Table::new(&data)
        .with(Style::psql())
        .with(
            Modify::new(Full).with(Alignment::right()).with(
                RenderSettings::default()
                    .alignement(AlignmentStrategy::PerLine)
                    .trim(TrimStrategy::Horizontal),
            ),
        )
        .to_string();

    let expected = concat!(
        "         N | column 0 | column 1 | column 2 \n",
        "-----------+----------+----------+----------\n",
        "         0 |      0-0 |      0-1 |      0-2 \n",
        "       asd |      1-0 |      1-1 |      1-2 \n",
        "  21213123 |          |          |          \n",
        "           |          |          |          \n",
        "    asdasd |          |          |          \n",
        "           |          |          |          \n",
        "         2 |      2-0 | https:// |      2-2 \n",
        "           |          |      www |          \n",
        "           |          |        . |          \n",
        "           |          |   redhat |          \n",
        "           |          |     .com |          \n",
        "           |          |      /en |          \n",
    );

    assert_eq!(table, expected);

    let mut data = create_vector::<3, 3>();
    data[1][0] = String::from("\n\n\nasd\n21213123   asdasd\n\n\n");
    data[2][2] = String::from("https://\nwww\n.\nredhat\n.com\n/en");

    let table = Table::new(&data)
        .with(Style::psql())
        .with(
            Modify::new(Full)
                .with(Alignment::center_vertical())
                .with(Alignment::left())
                .with(
                    RenderSettings::default()
                        .alignement(AlignmentStrategy::PerLine)
                        .trim(TrimStrategy::Both),
                ),
        )
        .to_string();

    let expected = concat!(
        " N                 | column 0 | column 1 | column 2 \n",
        "-------------------+----------+----------+----------\n",
        " 0                 | 0-0      | 0-1      | 0-2      \n",
        "                   |          |          |          \n",
        "                   |          |          |          \n",
        " asd               |          |          |          \n",
        " 21213123   asdasd | 1-0      | 1-1      | 1-2      \n",
        "                   |          |          |          \n",
        "                   |          |          |          \n",
        "                   |          |          |          \n",
        "                   |          | https:// |          \n",
        "                   |          | www      |          \n",
        " 2                 | 2-0      | .        | 2-2      \n",
        "                   |          | redhat   |          \n",
        "                   |          | .com     |          \n",
        "                   |          | /en      |          \n",
    );

    assert_eq!(table, expected);
}
