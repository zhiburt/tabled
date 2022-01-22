use crate::util::create_vector;
use tabled::{Column, Full, MaxWidth, Modify, Object, Row, Style, Table};

mod util;

#[test]
fn max_width() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::GITHUB_MARKDOWN)
        .with(Modify::new(Column(1..).not(Row(..1))).with(MaxWidth::truncating(2, "...")))
        .to_string();

    let expected = concat!(
        "| N | column 0 | column 1 | column 2 |\n",
        "|---+----------+----------+----------|\n",
        "| 0 |  0-...   |  0-...   |  0-...   |\n",
        "| 1 |  1-...   |  1-...   |  1-...   |\n",
        "| 2 |  2-...   |  2-...   |  2-...   |\n",
    );

    assert_eq!(table, expected);
}

#[test]
fn max_width_wrapped() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::GITHUB_MARKDOWN)
        .with(Modify::new(Column(1..).not(Row(..1))).with(MaxWidth::wrapping(2)))
        .to_string();

    let expected = concat!(
        "| N | column 0 | column 1 | column 2 |\n",
        "|---+----------+----------+----------|\n",
        "| 0 |    0-    |    0-    |    0-    |\n",
        "|   |    0     |    1     |    2     |\n",
        "| 1 |    1-    |    1-    |    1-    |\n",
        "|   |    0     |    1     |    2     |\n",
        "| 2 |    2-    |    2-    |    2-    |\n",
        "|   |    0     |    1     |    2     |\n",
    );

    assert_eq!(table, expected);
}

#[cfg(feature = "color")]
#[test]
fn max_width_wrapped_collored() {
    use owo_colors::OwoColorize;

    let data = &[
        "asd".red().to_string(),
        "zxc2".blue().to_string(),
        "asdasd".on_black().green().to_string(),
    ];

    let expected = concat!(
        "| St |\n",
        "| ri |\n",
        "| ng |\n",
        "|----|\n",
        "| \u{1b}[31mas\u{1b}[39m |\n",
        "| \u{1b}[31md\u{1b}[0m  |\n",
        "| \u{1b}[34mzx\u{1b}[39m |\n",
        "| \u{1b}[34mc2\u{1b}[0m |\n",
        "| \u{1b}[32m\u{1b}[40mas\u{1b}[39m\u{1b}[49m |\n",
        "| \u{1b}[32m\u{1b}[40mda\u{1b}[39m\u{1b}[49m |\n",
        "| \u{1b}[32m\u{1b}[40msd\u{1b}[0m\u{1b}[0m |\n",
    );

    let table = Table::new(data)
        .with(Style::GITHUB_MARKDOWN)
        .with(Modify::new(Full).with(MaxWidth::wrapping(2)))
        .to_string();

    println!("{}", table);

    assert_eq!(expected, table);
}

#[test]
fn dont_change_content_if_width_is_less_then_max_width() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::GITHUB_MARKDOWN)
        .with(Modify::new(Full).with(MaxWidth::truncating(1000, "...")))
        .to_string();

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
fn max_width_with_emoji() {
    let data = &["🤠", "😳🥵🥶😱😨", "🚴🏻‍♀️🚴🏻🚴🏻‍♂️🚵🏻‍♀️🚵🏻🚵🏻‍♂️"];

    let _expected = concat!(
        "|  &st...   |\n",
        "|-----------|\n",
        "|    🤠     |\n",
        "| 😳🥵🥶... |\n",
        "|  🚴🏻\u{200d}...  |\n",
    );

    let table = Table::new(data)
        .with(Style::GITHUB_MARKDOWN)
        .with(Modify::new(Full).with(MaxWidth::truncating(3, "...")))
        .to_string();

    assert_eq!(table, _expected);
}

#[cfg(feature = "color")]
#[test]
fn color_chars_are_stripped() {
    use owo_colors::OwoColorize;

    let data = &[
        "asd".red().to_string(),
        "zxc".blue().to_string(),
        "asdasd".on_black().green().to_string(),
    ];

    let expected = concat!(
        "| Str... |\n",
        "|--------|\n",
        "|  \u{1b}[31masd\u{1b}[0m   |\n",
        "|  \u{1b}[34mzxc\u{1b}[0m   |\n",
        "| \u{1b}[32m\u{1b}[40masd\u{1b}[39m\u{1b}[49m... |\n",
    );

    let table = Table::new(data)
        .with(Style::GITHUB_MARKDOWN)
        .with(Modify::new(Full).with(MaxWidth::truncating(3, "...")))
        .to_string();

    println!("{}", table);

    assert_eq!(expected, table);
}
