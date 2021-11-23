use crate::util::create_vector;
use tabled::style::Line;
use tabled::{Style, Table};

mod util;

#[test]
fn default_style() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data).with(Style::ASCII).to_string();

    let expected = concat!(
        "+---+----------+----------+----------+\n",
        "| N | column 0 | column 1 | column 2 |\n",
        "+---+----------+----------+----------+\n",
        "| 0 |   0-0    |   0-1    |   0-2    |\n",
        "+---+----------+----------+----------+\n",
        "| 1 |   1-0    |   1-1    |   1-2    |\n",
        "+---+----------+----------+----------+\n",
        "| 2 |   2-0    |   2-1    |   2-2    |\n",
        "+---+----------+----------+----------+\n",
    );

    assert_eq!(table, expected);
}

#[test]
fn psql_style() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data).with(Style::PSQL).to_string();

    let expected = concat!(
        " N | column 0 | column 1 | column 2 \n",
        "---+----------+----------+----------\n",
        " 0 |   0-0    |   0-1    |   0-2    \n",
        " 1 |   1-0    |   1-1    |   1-2    \n",
        " 2 |   2-0    |   2-1    |   2-2    \n",
    );

    assert_eq!(table, expected);
}

#[test]
fn github_markdown_style() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data).with(Style::GITHUB_MARKDOWN).to_string();

    let expected = concat!(
        "| N | column 0 | column 1 | column 2 |\n",
        "|---+----------+----------+----------|\n",
        "| 0 |   0-0    |   0-1    |   0-2    |\n",
        "| 1 |   1-0    |   1-1    |   1-2    |\n",
        "| 2 |   2-0    |   2-1    |   2-2    |\n",
    );

    assert_eq!(table, expected);
}

#[test]
fn pseudo_style() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data).with(Style::PSEUDO).to_string();

    let expected = concat!(
        "┌───┬──────────┬──────────┬──────────┐\n",
        "│ N │ column 0 │ column 1 │ column 2 │\n",
        "├───┼──────────┼──────────┼──────────┤\n",
        "│ 0 │   0-0    │   0-1    │   0-2    │\n",
        "├───┼──────────┼──────────┼──────────┤\n",
        "│ 1 │   1-0    │   1-1    │   1-2    │\n",
        "├───┼──────────┼──────────┼──────────┤\n",
        "│ 2 │   2-0    │   2-1    │   2-2    │\n",
        "└───┴──────────┴──────────┴──────────┘\n",
    );

    assert_eq!(table, expected);
}

#[test]
fn pseudo_clean_style() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data).with(Style::PSEUDO_CLEAN).to_string();

    let expected = concat!(
        "┌───┬──────────┬──────────┬──────────┐\n",
        "│ N │ column 0 │ column 1 │ column 2 │\n",
        "├───┼──────────┼──────────┼──────────┤\n",
        "│ 0 │   0-0    │   0-1    │   0-2    │\n",
        "│ 1 │   1-0    │   1-1    │   1-2    │\n",
        "│ 2 │   2-0    │   2-1    │   2-2    │\n",
        "└───┴──────────┴──────────┴──────────┘\n",
    );

    assert_eq!(table, expected);
}

#[test]
fn noborder_style() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data).with(Style::NO_BORDER).to_string();

    let expected = concat!(
        " N   column 0   column 1   column 2 \n",
        " 0     0-0        0-1        0-2    \n",
        " 1     1-0        1-1        1-2    \n",
        " 2     2-0        2-1        2-2    \n",
    );

    assert_eq!(table, expected);
}

#[test]
fn style_head_changes() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::PSEUDO_CLEAN.header(None))
        .to_string();

    let expected = concat!(
        "┌───┬──────────┬──────────┬──────────┐\n",
        "│ N │ column 0 │ column 1 │ column 2 │\n",
        "│ 0 │   0-0    │   0-1    │   0-2    │\n",
        "│ 1 │   1-0    │   1-1    │   1-2    │\n",
        "│ 2 │   2-0    │   2-1    │   2-2    │\n",
        "└───┴──────────┴──────────┴──────────┘\n",
    );

    assert_eq!(table, expected);
}

#[test]
fn style_frame_changes() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::PSEUDO_CLEAN.frame_bottom(None).frame_top(None))
        .to_string();

    let expected = concat!(
        "│ N │ column 0 │ column 1 │ column 2 │\n",
        "├───┼──────────┼──────────┼──────────┤\n",
        "│ 0 │   0-0    │   0-1    │   0-2    │\n",
        "│ 1 │   1-0    │   1-1    │   1-2    │\n",
        "│ 2 │   2-0    │   2-1    │   2-2    │\n",
    );

    assert_eq!(table, expected);
}

#[test]
fn custom_style() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(
            Style::NO_BORDER
                .frame_bottom(Some(Line::short('*', '\'')))
                .split(Some(Line::short('`', '\'')))
                .inner('\''),
        )
        .to_string();

    let expected = concat!(
        " N ' column 0 ' column 1 ' column 2 \n",
        "```'``````````'``````````'``````````\n",
        " 0 '   0-0    '   0-1    '   0-2    \n",
        "```'``````````'``````````'``````````\n",
        " 1 '   1-0    '   1-1    '   1-2    \n",
        "```'``````````'``````````'``````````\n",
        " 2 '   2-0    '   2-1    '   2-2    \n",
        "***'**********'**********'**********\n",
    );

    assert_eq!(table, expected);
}
