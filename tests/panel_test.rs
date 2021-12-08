use crate::util::create_vector;
use tabled::{
    Alignment, Border, Footer, Full, Header, Highlight, Modify, Object, Panel, Row, Style, Table,
};

mod util;

#[test]
fn panel_has_no_style_by_default() {
    let table = Table::new(create_vector::<3, 3>())
        .with(Style::PSQL)
        .with(Panel("Linux Distributions", 0))
        .to_string();

    // todo: it would be better if vertical split was not set in panel line
    // it is because it has right border printed
    let expected = concat!(
        "Linux Distributions                  \n",
        " N | column 0 | column 1 | column 2 \n",
        "---+----------+----------+----------\n",
        " 0 |   0-0    |   0-1    |   0-2    \n",
        " 1 |   1-0    |   1-1    |   1-2    \n",
        " 2 |   2-0    |   2-1    |   2-2    \n",
    );

    assert_eq!(table, expected);
}

#[test]
fn highligt_panel() {
    let table = Table::new(create_vector::<3, 3>())
        .with(Panel("Linux Distributions", 0))
        .with(Style::PSQL)
        .with(Highlight::cell(
            0,
            0,
            Border::full('#', '#', '#', '#', '#', '#', '#', '#'),
        ))
        .to_string();

    // todo: it would be better if vertical split was not set in panel line
    // it is because it has right border printed
    let expected = concat!(
        "######################################\n",
        "#Linux Distributions                 #\n",
        "######################################\n",
        "  N | column 0 | column 1 | column 2 \n",
        "  0 |   0-0    |   0-1    |   0-2    \n",
        "  1 |   1-0    |   1-1    |   1-2    \n",
        "  2 |   2-0    |   2-1    |   2-2    \n",
    );

    assert_eq!(table, expected);
}

#[test]
fn top_panel() {
    let table = Table::new(create_vector::<3, 3>())
        .with(Panel("Linux Distributions", 0))
        .with(Modify::new(Full).with(Alignment::center_horizontal()))
        .with(Style::PSQL)
        .to_string();

    let expected = concat!(
        "        Linux Distributions         |\n",
        "------------------------------------+\n",
        " N | column 0 | column 1 | column 2 \n",
        " 0 |   0-0    |   0-1    |   0-2    \n",
        " 1 |   1-0    |   1-1    |   1-2    \n",
        " 2 |   2-0    |   2-1    |   2-2    \n",
    );

    assert_eq!(table, expected);
}

#[test]
fn bottom_panel() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Panel("Linux Distributions", data.len() + 1))
        .with(Modify::new(Row(data.len() + 1..)).with(Alignment::center_horizontal()))
        .with(Style::PSQL)
        .to_string();

    let expected = concat!(
        " N | column 0 | column 1 | column 2 \n",
        "---+----------+----------+----------\n",
        " 0 |   0-0    |   0-1    |   0-2    \n",
        " 1 |   1-0    |   1-1    |   1-2    \n",
        " 2 |   2-0    |   2-1    |   2-2    \n",
        "        Linux Distributions         |\n",
    );

    assert_eq!(table, expected);
}

#[test]
fn inner_panel() {
    let table = Table::new(create_vector::<3, 3>())
        .with(Panel("Linux Distributions", 2))
        .with(Modify::new(Row(2..)).with(Alignment::center_horizontal()))
        .with(Style::PSQL)
        .to_string();

    let expected = concat!(
        " N | column 0 | column 1 | column 2 \n",
        "---+----------+----------+----------\n",
        " 0 |   0-0    |   0-1    |   0-2    \n",
        "        Linux Distributions         |\n",
        " 1 |   1-0    |   1-1    |   1-2    \n",
        " 2 |   2-0    |   2-1    |   2-2    \n",
    );

    assert_eq!(table, expected);
}

#[test]
fn header() {
    let table = Table::new(create_vector::<3, 3>())
        .with(Header("Linux Distributions"))
        .with(Style::PSQL)
        .with(Modify::new(Row(0..1)).with(Alignment::center_horizontal()))
        .to_string();

    let expected = concat!(
        "        Linux Distributions         |\n",
        "------------------------------------+\n",
        " N | column 0 | column 1 | column 2 \n",
        " 0 |   0-0    |   0-1    |   0-2    \n",
        " 1 |   1-0    |   1-1    |   1-2    \n",
        " 2 |   2-0    |   2-1    |   2-2    \n",
    );

    assert_eq!(table, expected);
}

#[test]
fn footer() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Header("Linux Distributions"))
        .with(Footer("The end"))
        .with(Style::PSQL)
        .with(Modify::new(Row(0..1).and(Row(data.len()..))).with(Alignment::center_horizontal()))
        .to_string();

    let expected = concat!(
        "        Linux Distributions         |\n",
        "------------------------------------+\n",
        " N | column 0 | column 1 | column 2 \n",
        " 0 |   0-0    |   0-1    |   0-2    \n",
        " 1 |   1-0    |   1-1    |   1-2    \n",
        " 2 |   2-0    |   2-1    |   2-2    \n",
        "              The end               |\n",
    );

    assert_eq!(table, expected);
}
