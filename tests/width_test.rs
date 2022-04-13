use crate::util::{create_vector, is_lines_equal};
use tabled::{
    formatting_settings::TrimStrategy,
    object::{Cell, Columns, Full, Object, Rows},
    Alignment, MaxWidth, MinWidth, Modify, Panel, Span, Style, Table, Tabled,
};

mod util;

#[test]
fn max_width() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Modify::new(Columns::new(1..).not(Rows::single(0))).with(MaxWidth::truncating(1)))
        .to_string();

    assert_eq!(
        table,
        concat!(
            "| N | column 0 | column 1 | column 2 |\n",
            "|---+----------+----------+----------|\n",
            "| 0 |    0     |    0     |    0     |\n",
            "| 1 |    1     |    1     |    1     |\n",
            "| 2 |    2     |    2     |    2     |\n",
        )
    );
}

#[test]
fn max_width_with_suffix() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(
            Modify::new(Columns::new(1..).not(Rows::single(0)))
                .with(MaxWidth::truncating(2).suffix("...")),
        )
        .to_string();

    assert_eq!(
        table,
        concat!(
            "| N | column 0 | column 1 | column 2 |\n",
            "|---+----------+----------+----------|\n",
            "| 0 |  0-...   |  0-...   |  0-...   |\n",
            "| 1 |  1-...   |  1-...   |  1-...   |\n",
            "| 2 |  2-...   |  2-...   |  2-...   |\n",
        )
    );
}

#[test]
fn max_width_doesnt_icrease_width_if_it_is_smaller() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Modify::new(Columns::new(1..).not(Rows::single(0))).with(MaxWidth::truncating(50)))
        .to_string();

    assert_eq!(
        table,
        concat!(
            "| N | column 0 | column 1 | column 2 |\n",
            "|---+----------+----------+----------|\n",
            "| 0 |   0-0    |   0-1    |   0-2    |\n",
            "| 1 |   1-0    |   1-1    |   1-2    |\n",
            "| 2 |   2-0    |   2-1    |   2-2    |\n",
        )
    );
}

#[test]
fn max_width_wrapped() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Modify::new(Columns::new(1..).not(Rows::single(0))).with(MaxWidth::wrapping(2)))
        .to_string();

    assert_eq!(
        table,
        concat!(
            "| N | column 0 | column 1 | column 2 |\n",
            "|---+----------+----------+----------|\n",
            "| 0 |    0-    |    0-    |    0-    |\n",
            "|   |    0     |    1     |    2     |\n",
            "| 1 |    1-    |    1-    |    1-    |\n",
            "|   |    0     |    1     |    2     |\n",
            "| 2 |    2-    |    2-    |    2-    |\n",
            "|   |    0     |    1     |    2     |\n",
        )
    );
}

#[test]
fn max_width_wrapped_does_nothing_if_str_is_smaller() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Modify::new(Columns::new(1..).not(Rows::single(0))).with(MaxWidth::wrapping(100)))
        .to_string();

    assert_eq!(
        table,
        concat!(
            "| N | column 0 | column 1 | column 2 |\n",
            "|---+----------+----------+----------|\n",
            "| 0 |   0-0    |   0-1    |   0-2    |\n",
            "| 1 |   1-0    |   1-1    |   1-2    |\n",
            "| 2 |   2-0    |   2-1    |   2-2    |\n",
        )
    );
}

#[test]
fn max_width_wrapped_keep_words() {
    let data = vec!["this is a long sentence"];
    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Modify::new(Full).with(Alignment::left()))
        .with(Modify::new(Full).with(MaxWidth::wrapping(17).keep_words()))
        .to_string();

    assert_eq!(
        table,
        concat!(
            "| &str              |\n",
            "|-------------------|\n",
            "| this is a long    |\n",
            "| sentence          |\n",
        )
    );
    assert!(is_lines_equal(&table, 17 + 2 + 2));

    let data = vec!["this is a long  sentence"];
    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Modify::new(Full).with(Alignment::left()))
        .with(Modify::new(Full).with(MaxWidth::wrapping(17).keep_words()))
        .to_string();

    assert_eq!(
        table,
        concat!(
            "| &str              |\n",
            "|-------------------|\n",
            "| this is a long    |\n",
            "| sentence          |\n",
        )
    );
    assert!(is_lines_equal(&table, 17 + 2 + 2));

    let data = vec!["this is a long   sentence"];
    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Modify::new(Full).with(Alignment::left()))
        .with(Modify::new(Full).with(MaxWidth::wrapping(17).keep_words()))
        .to_string();

    assert_eq!(
        table,
        concat!(
            "| &str              |\n",
            "|-------------------|\n",
            "| this is a long    |\n",
            "| sentence          |\n",
        )
    );

    let data = vec!["this is a long    sentence"];
    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Modify::new(Full).with(Alignment::left()))
        .with(Modify::new(Full).with(MaxWidth::wrapping(17).keep_words()))
        .to_string();

    // 'sentence' doesnt have a space ' sentence' because we use left alignment
    assert_eq!(
        table,
        concat!(
            "| &str              |\n",
            "|-------------------|\n",
            "| this is a long    |\n",
            "|  sentence         |\n",
        )
    );
    assert!(is_lines_equal(&table, 17 + 2 + 2));

    let data = vec!["this"];
    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Modify::new(Full).with(MaxWidth::wrapping(10).keep_words()))
        .to_string();

    let expected = concat!("| &str |\n", "|------|\n", "| this |\n",);

    assert_eq!(table, expected);
    assert!(is_lines_equal(&table, 8));
}

#[cfg(feature = "color")]
#[test]
fn max_width_wrapped_keep_words_color() {
    use owo_colors::OwoColorize;

    let data = vec!["this is a long sentence".on_black().green().to_string()];
    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Modify::new(Full).with(Alignment::left()))
        .with(Modify::new(Full).with(MaxWidth::wrapping(17).keep_words()))
        .to_string();

    let expected = concat!(
        "| String            |\n",
        "|-------------------|\n",
        "| \u{1b}[32m\u{1b}[40mthis is a long \u{1b}[39m\u{1b}[49m   |\n",
        "| \u{1b}[32m\u{1b}[40mse\u{1b}[39m\u{1b}[49m\u{1b}[32m\u{1b}[40mntence\u{1b}[39m\u{1b}[49m          |\n",
    );

    assert_eq!(table, expected);

    let data = vec!["this is a long  sentence".on_black().green().to_string()];
    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Modify::new(Full).with(Alignment::left()))
        .with(Modify::new(Full).with(MaxWidth::wrapping(17).keep_words()))
        .to_string();

    let expected = concat!(
        "| String            |\n",
        "|-------------------|\n",
        "| \u{1b}[32m\u{1b}[40mthis is a long  \u{1b}[39m\u{1b}[49m  |\n",
        "| \u{1b}[32m\u{1b}[40ms\u{1b}[39m\u{1b}[49m\u{1b}[32m\u{1b}[40mentence\u{1b}[39m\u{1b}[49m          |\n",
    );

    assert_eq!(table, expected);

    let data = vec!["this is a long   sentence".on_black().green().to_string()];
    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Modify::new(Full).with(Alignment::left()))
        .with(Modify::new(Full).with(MaxWidth::wrapping(17).keep_words()))
        .to_string();

    assert_eq!(
        table,
        concat!(
            "| String            |\n",
            "|-------------------|\n",
            "| \u{1b}[32m\u{1b}[40mthis is a long   \u{1b}[39m\u{1b}[49m |\n",
            "| \u{1b}[32m\u{1b}[40msentence\u{1b}[39m\u{1b}[49m          |\n",
        )
    );

    let data = vec!["this is a long    sentence".on_black().green().to_string()];
    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Modify::new(Full).with(Alignment::left()))
        .with(Modify::new(Full).with(MaxWidth::wrapping(17).keep_words()))
        .to_string();

    assert_eq!(
        table,
        concat!(
            "| String            |\n",
            "|-------------------|\n",
            "| \u{1b}[32m\u{1b}[40mthis is a long   \u{1b}[39m\u{1b}[49m |\n",
            "| \u{1b}[32m\u{1b}[40m sentence\u{1b}[39m\u{1b}[49m         |\n",
        )
    );

    let data = vec!["this".on_black().green().to_string()];
    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Modify::new(Full).with(MaxWidth::wrapping(10).keep_words()))
        .to_string();

    assert_eq!(
        table,
        concat!(
            "| String |\n",
            "|--------|\n",
            "|  \u{1b}[32m\u{1b}[40mthis\u{1b}[39m\u{1b}[49m  |\n",
        )
    );
}

#[test]
fn max_width_wrapped_keep_words_long_word() {
    let data = vec!["this is a long sentencesentencesentence"];
    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Modify::new(Full).with(Alignment::left()))
        .with(Modify::new(Full).with(MaxWidth::wrapping(17).keep_words()))
        .to_string();

    let expected = concat!(
        "| &str              |\n",
        "|-------------------|\n",
        "| this is a long    |\n",
        "| sentencesentences |\n",
        "| entence           |\n",
    );

    assert_eq!(table, expected);
}

#[cfg(feature = "color")]
#[test]
fn max_width_wrapped_keep_words_long_word_color() {
    use owo_colors::OwoColorize;

    let data = vec!["this is a long sentencesentencesentence"
        .on_black()
        .green()
        .to_string()];
    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Modify::new(Full).with(Alignment::left()))
        .with(Modify::new(Full).with(MaxWidth::wrapping(17).keep_words()))
        .to_string();

    let expected = concat!(
        "| String            |\n",
        "|-------------------|\n",
        "| \u{1b}[32m\u{1b}[40mthis is a long \u{1b}[39m\u{1b}[49m   |\n",
        "| \u{1b}[32m\u{1b}[40mse\u{1b}[39m\u{1b}[49m\u{1b}[32m\u{1b}[40mntencesentences\u{1b}[39m\u{1b}[49m |\n",
        "| \u{1b}[32m\u{1b}[40mentence\u{1b}[39m\u{1b}[49m           |\n",
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
        "| \u{1b}[31md\u{1b}[39m  |\n",
        "| \u{1b}[34mzx\u{1b}[39m |\n",
        "| \u{1b}[34mc2\u{1b}[39m |\n",
        "| \u{1b}[32m\u{1b}[40mas\u{1b}[39m\u{1b}[49m |\n",
        "| \u{1b}[32m\u{1b}[40mda\u{1b}[39m\u{1b}[49m |\n",
        "| \u{1b}[32m\u{1b}[40msd\u{1b}[39m\u{1b}[49m |\n",
    );

    let table = Table::new(data)
        .with(Style::github_markdown())
        .with(Modify::new(Full).with(MaxWidth::wrapping(2)))
        .to_string();

    assert_eq!(expected, table);
}

#[test]
fn dont_change_content_if_width_is_less_then_max_width() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Modify::new(Full).with(MaxWidth::truncating(1000).suffix("...")))
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
        .with(Style::github_markdown())
        .with(Modify::new(Full).with(MaxWidth::truncating(3).suffix("...")))
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
        .with(Style::github_markdown())
        .with(Modify::new(Full).with(MaxWidth::truncating(3).suffix("...")))
        .to_string();

    assert_eq!(expected, table);
}

#[test]
fn min_width() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Modify::new(Rows::single(0)).with(MinWidth::new(12)));

    assert_eq!(
        table.to_string(),
        concat!(
            "|      N       |   column 0   |   column 1   |   column 2   |\n",
            "|--------------+--------------+--------------+--------------|\n",
            "|      0       |     0-0      |     0-1      |     0-2      |\n",
            "|      1       |     1-0      |     1-1      |     1-2      |\n",
            "|      2       |     2-0      |     2-1      |     2-2      |\n",
        ),
    );

    let table = table.with(Modify::new(Full).with(TrimStrategy::None));

    assert_eq!(
        table.to_string(),
        concat!(
            "| N            | column 0     | column 1     | column 2     |\n",
            "|--------------+--------------+--------------+--------------|\n",
            "|      0       |     0-0      |     0-1      |     0-2      |\n",
            "|      1       |     1-0      |     1-1      |     1-2      |\n",
            "|      2       |     2-0      |     2-1      |     2-2      |\n",
        ),
    );
}

#[test]
fn min_width_with_filler() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Modify::new(Rows::single(0)).with(MinWidth::new(12).fill_with('.')))
        .to_string();

    let expected = concat!(
        "| N........... | column 0.... | column 1.... | column 2.... |\n",
        "|--------------+--------------+--------------+--------------|\n",
        "|      0       |     0-0      |     0-1      |     0-2      |\n",
        "|      1       |     1-0      |     1-1      |     1-2      |\n",
        "|      2       |     2-0      |     2-1      |     2-2      |\n",
    );

    assert_eq!(table, expected);
}

#[test]
fn min_width_one_column() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Modify::new(Cell(0, 0)).with(MinWidth::new(5)));

    assert_eq!(
        table.to_string(),
        concat!(
            "|   N   | column 0 | column 1 | column 2 |\n",
            "|-------+----------+----------+----------|\n",
            "|   0   |   0-0    |   0-1    |   0-2    |\n",
            "|   1   |   1-0    |   1-1    |   1-2    |\n",
            "|   2   |   2-0    |   2-1    |   2-2    |\n",
        )
    );

    let table = table.with(Modify::new(Full).with(TrimStrategy::None));

    assert_eq!(
        table.to_string(),
        concat!(
            "| N     | column 0 | column 1 | column 2 |\n",
            "|-------+----------+----------+----------|\n",
            "|   0   |   0-0    |   0-1    |   0-2    |\n",
            "|   1   |   1-0    |   1-1    |   1-2    |\n",
            "|   2   |   2-0    |   2-1    |   2-2    |\n",
        )
    );
}

#[test]
fn min_width_on_smaller_content() {
    let data = create_vector::<3, 3>();

    assert_eq!(
        Table::new(&data)
            .with(Style::github_markdown())
            .with(Modify::new(Rows::single(0)).with(MinWidth::new(1)))
            .to_string(),
        Table::new(&data).with(Style::github_markdown()).to_string()
    );
}

#[test]
fn min_with_max_width() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Modify::new(Rows::single(0)).with(MinWidth::new(3)))
        .with(Modify::new(Rows::single(0)).with(MaxWidth::truncating(3)));

    assert_eq!(
        table.to_string(),
        concat!(
            "|  N  | col | col | col |\n",
            "|-----+-----+-----+-----|\n",
            "|  0  | 0-0 | 0-1 | 0-2 |\n",
            "|  1  | 1-0 | 1-1 | 1-2 |\n",
            "|  2  | 2-0 | 2-1 | 2-2 |\n",
        )
    );

    let table = table.with(Modify::new(Full).with(TrimStrategy::None));

    assert_eq!(
        table.to_string(),
        concat!(
            "| N   | col | col | col |\n",
            "|-----+-----+-----+-----|\n",
            "|  0  | 0-0 | 0-1 | 0-2 |\n",
            "|  1  | 1-0 | 1-1 | 1-2 |\n",
            "|  2  | 2-0 | 2-1 | 2-2 |\n",
        )
    );
}

#[test]
fn min_with_max_width_truncate_suffix() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Modify::new(Rows::single(0)).with(MinWidth::new(3)))
        .with(Modify::new(Rows::single(0)).with(MaxWidth::truncating(3).suffix("...")));

    assert_eq!(
        table.to_string(),
        concat!(
            "|  N  | col... | col... | col... |\n",
            "|-----+--------+--------+--------|\n",
            "|  0  |  0-0   |  0-1   |  0-2   |\n",
            "|  1  |  1-0   |  1-1   |  1-2   |\n",
            "|  2  |  2-0   |  2-1   |  2-2   |\n",
        )
    );

    let table = table.with(Modify::new(Full).with(TrimStrategy::None));

    assert_eq!(
        table.to_string(),
        concat!(
            "| N   | col... | col... | col... |\n",
            "|-----+--------+--------+--------|\n",
            "|  0  |  0-0   |  0-1   |  0-2   |\n",
            "|  1  |  1-0   |  1-1   |  1-2   |\n",
            "|  2  |  2-0   |  2-1   |  2-2   |\n",
        )
    );
}

#[cfg(feature = "color")]
#[test]
fn min_width_color() {
    use owo_colors::OwoColorize;

    let data = &[
        "asd".red().to_string(),
        "zxc".blue().to_string(),
        "asdasd".on_black().green().to_string(),
    ];

    let table = Table::new(data)
        .with(Style::github_markdown())
        .with(Modify::new(Full).with(MinWidth::new(10)))
        .to_string();

    assert_eq!(
        table,
        concat!(
            "|   String   |\n",
            "|------------|\n",
            "|    \u{1b}[31masd\u{1b}[0m     |\n",
            "|    \u{1b}[34mzxc\u{1b}[0m     |\n",
            "|   \u{1b}[32m\u{1b}[40masdasd\u{1b}[0m\u{1b}[0m   |\n",
        )
    );
}

#[cfg(feature = "color")]
#[test]
fn min_width_color_with_smaller_then_width() {
    use owo_colors::OwoColorize;

    let data = &[
        "asd".red().to_string(),
        "zxc".blue().to_string(),
        "asdasd".on_black().green().to_string(),
    ];

    assert_eq!(
        Table::new(data)
            .with(Modify::new(Full).with(MinWidth::new(1)))
            .to_string(),
        Table::new(data).to_string()
    );
}

#[test]
fn total_width_big() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(MaxWidth::truncating(80))
        .with(MinWidth::new(80))
        .to_string();

    assert_eq!(
        table,
        concat!(
            "|      N       |      column 0       |      column 1      |      column 2      |\n",
            "|--------------+---------------------+--------------------+--------------------|\n",
            "|      0       |         0-0         |        0-1         |        0-2         |\n",
            "|      1       |         1-0         |        1-1         |        1-2         |\n",
            "|      2       |         2-0         |        2-1         |        2-2         |\n",
        )
    );
    assert!(is_lines_equal(&table, 80));

    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(MaxWidth::truncating(80))
        .with(MinWidth::new(80))
        .with(Modify::new(Full).with(TrimStrategy::None))
        .to_string();

    assert_eq!(
        table,
        concat!(
            "| N            | column 0            | column 1           | column 2           |\n",
            "|--------------+---------------------+--------------------+--------------------|\n",
            "| 0            |   0-0               |   0-1              |   0-2              |\n",
            "| 1            |   1-0               |   1-1              |   1-2              |\n",
            "| 2            |   2-0               |   2-1              |   2-2              |\n",
        )
    );
    assert!(is_lines_equal(&table, 80));
}

#[test]
fn total_width_big_with_panel() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Panel("Hello World", 0))
        .with(Modify::new(Full).with(Alignment::center()))
        .with(Style::github_markdown())
        .with(MaxWidth::truncating(80))
        .with(MinWidth::new(80))
        .to_string();

    let expected = concat!(
        "|                            Hello World                                       |\n",
        "|--------------+---------------------+--------------------+--------------------|\n",
        "| N            | column 0            | column 1           | column 2           |\n",
        "| 0            |   0-0               |   0-1              |   0-2              |\n",
        "| 1            |   1-0               |   1-1              |   1-2              |\n",
        "| 2            |   2-0               |   2-1              |   2-2              |\n",
    );

    assert_eq!(table, expected);
}

#[test]
fn total_width_big_with_panel_with_wrapping_doesnt_affect_increase() {
    let data = create_vector::<3, 3>();
    let table1 = Table::new(&data)
        .with(Panel("Hello World", 0))
        .with(Modify::new(Full).with(Alignment::center()))
        .with(Style::github_markdown())
        .with(MaxWidth::wrapping(80))
        .with(MinWidth::new(80))
        .to_string();

    let table2 = Table::new(&data)
        .with(Panel("Hello World", 0))
        .with(Modify::new(Full).with(Alignment::center()))
        .with(Style::github_markdown())
        .with(MaxWidth::truncating(80))
        .with(MinWidth::new(80))
        .to_string();

    assert_eq!(table1, table2);
}

#[test]
fn total_width_small() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(MaxWidth::truncating(14))
        .with(MinWidth::new(14))
        .to_string();

    let expected = concat!(
        "|  |  |  | c |\n",
        "|--+--+--+---|\n",
        "|  |  |  | 0 |\n",
        "|  |  |  | 1 |\n",
        "|  |  |  | 2 |\n",
    );

    assert_eq!(table, expected);
    assert!(is_lines_equal(&table, 14));
}

#[test]
fn total_width_smaller_then_content() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(MaxWidth::truncating(8))
        .with(MinWidth::new(8))
        .to_string();

    let expected = concat!(
        "|  |  |  |  |\n",
        "|--+--+--+--|\n",
        "|  |  |  |  |\n",
        "|  |  |  |  |\n",
        "|  |  |  |  |\n",
    );

    assert_eq!(table, expected);
}

#[test]
fn total_width_small_with_panel() {
    let data = create_vector::<3, 3>();

    let table = Table::new(&data)
        .with(Modify::new(Full).with(Alignment::center()))
        .with(Style::github_markdown())
        .with(MaxWidth::truncating(20))
        .with(MinWidth::new(20))
        .to_string();

    let expected = concat!(
        "|  | co | co | col |\n",
        "|--+----+----+-----|\n",
        "|  | 0- | 0- | 0-2 |\n",
        "|  | 1- | 1- | 1-2 |\n",
        "|  | 2- | 2- | 2-2 |\n",
    );

    assert_eq!(table, expected);
    assert!(is_lines_equal(&table, 20));

    let table = Table::new(Vec::<usize>::new())
        .with(Panel("Hello World", 0))
        .with(MaxWidth::truncating(5))
        .with(MinWidth::new(5))
        .to_string();

    let expected = concat!(" Hel \n", "+---+\n", "| u |\n", "+---+\n",);

    assert_eq!(table, expected);
    assert!(is_lines_equal(&table, 5));

    let table = Table::new(&create_vector::<1, 2>())
        .with(Panel("Hello World", 0))
        .with(Modify::new(Full).with(Alignment::center()))
        .with(Style::github_markdown())
        .with(MaxWidth::truncating(20))
        .with(MinWidth::new(20))
        .to_string();

    let expected = concat!(
        "|   Hello World    |\n",
        "|--+-------+-------|\n",
        "|  | colum | colum |\n",
        "|  |  0-0  |  0-1  |\n",
    );

    assert_eq!(table, expected);
    assert!(is_lines_equal(&table, 20));

    let table = Table::new(&data)
        .with(Panel("Hello World", 0))
        .with(Modify::new(Full).with(Alignment::center()))
        .with(Style::github_markdown())
        .with(MaxWidth::truncating(20))
        .with(MinWidth::new(20))
        .to_string();

    let expected = concat!(
        "|   Hello World    |\n",
        "|--+----+----+-----|\n",
        "|  | co | co | col |\n",
        "|  | 0- | 0- | 0-2 |\n",
        "|  | 1- | 1- | 1-2 |\n",
        "|  | 2- | 2- | 2-2 |\n",
    );

    assert_eq!(table, expected);
    assert!(is_lines_equal(&table, 20));

    let table = Table::new(&data)
        .with(Panel("Hello World", 0))
        .with(Modify::new(Full).with(Alignment::center()))
        .with(Style::github_markdown())
        .with(MaxWidth::truncating(6))
        .with(MinWidth::new(6))
        .to_string();

    let expected = concat!(
        "|Hello World|\n",
        "|--+--+--+--|\n",
        "|  |  |  |  |\n",
        "|  |  |  |  |\n",
        "|  |  |  |  |\n",
        "|  |  |  |  |\n",
    );

    assert_eq!(table, expected);
    assert!(is_lines_equal(&table, 13));

    let table = Table::new(&data)
        .with(Panel("Hello World", 0))
        .with(Modify::new(Full).with(Alignment::center()))
        .with(Style::github_markdown())
        .with(MaxWidth::truncating(14))
        .with(MinWidth::new(14))
        .to_string();

    let expected = concat!(
        "|Hello World |\n",
        "|--+--+--+---|\n",
        "|  |  |  | c |\n",
        "|  |  |  | 0 |\n",
        "|  |  |  | 1 |\n",
        "|  |  |  | 2 |\n",
    );

    assert_eq!(table, expected);
    assert!(is_lines_equal(&table, 14));

    let table = Table::new(&data)
        .with(Panel("Hello World 123", 0))
        .with(Modify::new(Full).with(Alignment::center()))
        .with(Style::github_markdown())
        .with(MaxWidth::truncating(14))
        .with(MinWidth::new(14))
        .to_string();

    let expected = concat!(
        "|Hello World |\n",
        "|--+--+--+---|\n",
        "|  |  |  | c |\n",
        "|  |  |  | 0 |\n",
        "|  |  |  | 1 |\n",
        "|  |  |  | 2 |\n",
    );

    assert_eq!(table, expected);
    assert!(is_lines_equal(&table, 14));
}

#[test]
fn total_width_wrapping() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Modify::new(Full).with(Alignment::center()))
        .with(Style::github_markdown())
        .with(MaxWidth::wrapping(20))
        .with(MinWidth::new(20))
        .to_string();

    let expected = concat!(
        "|  | co | co | col |\n",
        "|  | lu | lu | umn |\n",
        "|  | mn | mn |  2  |\n",
        "|  |  0 |  1 |     |\n",
        "|--+----+----+-----|\n",
        "|  | 0- | 0- | 0-2 |\n",
        "|  | 0  | 1  |     |\n",
        "|  | 1- | 1- | 1-2 |\n",
        "|  | 0  | 1  |     |\n",
        "|  | 2- | 2- | 2-2 |\n",
        "|  | 0  | 1  |     |\n",
    );

    assert_eq!(table, expected);
    assert!(is_lines_equal(&table, 20));

    let mut data = create_vector::<3, 3>();
    data[2][2] = "some loong string".to_owned();
    let table = Table::new(&data)
        .with(Modify::new(Full).with(Alignment::center()))
        .with(Style::github_markdown())
        .with(MaxWidth::wrapping(20).keep_words())
        .with(MinWidth::new(20))
        .to_string();

    let expected = concat!(
        "|  |  | column  |  |\n",
        "|  |  | 1       |  |\n",
        "|--+--+---------+--|\n",
        "|  |  |   0-1   |  |\n",
        "|  |  |   1-1   |  |\n",
        "|  |  | some    |  |\n",
        "|  |  | loong   |  |\n",
        "|  |  | string  |  |\n",
    );

    assert_eq!(table, expected);
    assert!(is_lines_equal(&table, 20));
}

#[test]
fn total_width_small_with_panel_using_wrapping() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Panel("Hello World", 0))
        .with(Modify::new(Full).with(Alignment::center()))
        .with(Style::github_markdown())
        .with(MaxWidth::wrapping(20))
        .with(MinWidth::new(20))
        .to_string();

    let expected = concat!(
        "|   Hello World    |\n",
        "|--+----+----+-----|\n",
        "|  | co | co | col |\n",
        "|  | lu | lu | umn |\n",
        "|  | mn | mn |  2  |\n",
        "|  |  0 |  1 |     |\n",
        "|  | 0- | 0- | 0-2 |\n",
        "|  | 0  | 1  |     |\n",
        "|  | 1- | 1- | 1-2 |\n",
        "|  | 0  | 1  |     |\n",
        "|  | 2- | 2- | 2-2 |\n",
        "|  | 0  | 1  |     |\n",
    );

    assert_eq!(table, expected);
    assert!(is_lines_equal(&table, 20));

    let table = Table::new(&data)
        .with(Panel("Hello World", 0))
        .with(Modify::new(Full).with(Alignment::center()))
        .with(Style::github_markdown())
        .with(MaxWidth::wrapping(14))
        .with(MinWidth::new(14))
        .to_string();

    assert_eq!(
        table,
        concat!(
            "|Hello World |\n",
            "|--+--+--+---|\n",
            "|  |  |  | c |\n",
            "|  |  |  | o |\n",
            "|  |  |  | l |\n",
            "|  |  |  | u |\n",
            "|  |  |  | m |\n",
            "|  |  |  | n |\n",
            "|  |  |  |   |\n",
            "|  |  |  | 2 |\n",
            "|  |  |  | 0 |\n",
            "|  |  |  | - |\n",
            "|  |  |  | 2 |\n",
            "|  |  |  | 1 |\n",
            "|  |  |  | - |\n",
            "|  |  |  | 2 |\n",
            "|  |  |  | 2 |\n",
            "|  |  |  | - |\n",
            "|  |  |  | 2 |\n",
        )
    );
    assert!(is_lines_equal(&table, 14));

    let table = Table::new(&data)
        .with(Panel("Hello World 123", 0))
        .with(Modify::new(Full).with(Alignment::center()))
        .with(Style::github_markdown())
        .with(MaxWidth::wrapping(14))
        .with(MinWidth::new(14))
        .to_string();

    assert_eq!(
        table,
        concat!(
            "|Hello World |\n",
            "| 123        |\n",
            "|--+--+--+---|\n",
            "|  |  |  | c |\n",
            "|  |  |  | o |\n",
            "|  |  |  | l |\n",
            "|  |  |  | u |\n",
            "|  |  |  | m |\n",
            "|  |  |  | n |\n",
            "|  |  |  |   |\n",
            "|  |  |  | 2 |\n",
            "|  |  |  | 0 |\n",
            "|  |  |  | - |\n",
            "|  |  |  | 2 |\n",
            "|  |  |  | 1 |\n",
            "|  |  |  | - |\n",
            "|  |  |  | 2 |\n",
            "|  |  |  | 2 |\n",
            "|  |  |  | - |\n",
            "|  |  |  | 2 |\n",
        )
    );
    assert!(is_lines_equal(&table, 14));
}

#[test]
fn max_width_with_span() {
    let mut data = create_vector::<3, 3>();
    data[0][1] = "a long string".to_string();

    let table = Table::new(&data)
        .with(Style::psql())
        .with(Modify::new(Cell(1, 1)).with(Span::column(2)))
        .with(Modify::new(Cell(2, 2)).with(Span::column(2)));

    let table = table.with(MaxWidth::truncating(40));

    println!("{}", table);

    assert_eq!(
        table.to_string(),
        concat!(
            " N | column 0 | column 1 | column 2 \n",
            "---+----------+----------+----------\n",
            " 0 |    a long string    |   0-2    \n",
            " 1 |   1-0    |         1-1         \n",
            " 2 |   2-0    |   2-1    |   2-2    \n",
        )
    );
    assert!(is_lines_equal(&table.to_string(), 36));

    let table = table.with(MaxWidth::truncating(20));

    println!("{}", table);

    assert_eq!(
        table.to_string(),
        concat!(
            "  | col | col | col \n",
            "--+-----+-----+-----\n",
            "  | a long st | 0-2 \n",
            "  | 1-0 |    1-1    \n",
            "  | 2-0 | 2-1 | 2-2 \n",
        )
    );
    assert!(is_lines_equal(&table.to_string(), 20));

    let table = table.with(MaxWidth::truncating(10));

    println!("{}", table);

    assert_eq!(
        table.to_string(),
        concat!(
            "  |  |  |  \n",
            "--+--+--+--\n",
            "  | a l |  \n",
            "  |  | 1-1 \n",
            "  |  |  |  \n",
        )
    );
    assert!(is_lines_equal(&table.to_string(), 11));
}

#[test]
fn wrapping_as_total_multiline() {
    #[derive(Tabled)]
    struct D<'a>(
        #[tabled(rename = "version")] &'a str,
        #[tabled(rename = "published_date")] &'a str,
        #[tabled(rename = "is_active")] &'a str,
        #[tabled(rename = "major_feature")] &'a str,
    );

    let data = vec![
        D("0.2.1", "2021-06-23", "true", "#[header(inline)] attribute"),
        D("0.2.0", "2021-06-19", "false", "API changes"),
        D("0.1.4", "2021-06-07", "false", "display_with attribute"),
    ];

    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Modify::new(Full).with(Alignment::left()))
        .with(MaxWidth::wrapping(57))
        .to_string();

    assert_eq!(
        table,
        concat!(
            "| ver | published_d | is_act | major_feature            |\n",
            "| sio | ate         | ive    |                          |\n",
            "| n   |             |        |                          |\n",
            "|-----+-------------+--------+--------------------------|\n",
            "| 0.2 | 2021-06-23  | true   | #[header(inline)] attrib |\n",
            "| .1  |             |        | ute                      |\n",
            "| 0.2 | 2021-06-19  | false  | API changes              |\n",
            "| .0  |             |        |                          |\n",
            "| 0.1 | 2021-06-07  | false  | display_with attribute   |\n",
            "| .4  |             |        |                          |\n",
        )
    );
    assert!(is_lines_equal(&table, 57));

    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Modify::new(Full).with(Alignment::left()))
        .with(MaxWidth::wrapping(57).keep_words())
        .to_string();

    assert_eq!(
        table,
        concat!(
            "| ver | published_d | is_act | major_feature            |\n",
            "| sio | ate         | ive    |                          |\n",
            "| n   |             |        |                          |\n",
            "|-----+-------------+--------+--------------------------|\n",
            "| 0.2 | 2021-06-23  | true   | #[header(inline)]        |\n",
            "| .1  |             |        | attribute                |\n",
            "| 0.2 | 2021-06-19  | false  | API changes              |\n",
            "| .0  |             |        |                          |\n",
            "| 0.1 | 2021-06-07  | false  | display_with attribute   |\n",
            "| .4  |             |        |                          |\n",
        )
    );
    assert!(is_lines_equal(&table, 57));
}

#[cfg(feature = "color")]
#[test]
fn wrapping_as_total_multiline_color() {
    use owo_colors::{AnsiColors, OwoColorize};

    #[derive(Tabled)]
    struct D(
        #[tabled(rename = "version")] String,
        #[tabled(rename = "published_date")] String,
        #[tabled(rename = "is_active")] String,
        #[tabled(rename = "major_feature")] String,
    );

    let data = vec![
        D(
            "0.2.1".red().to_string(),
            "2021-06-23".red().on_truecolor(8, 10, 30).to_string(),
            "true".to_string(),
            "#[header(inline)] attribute"
                .blue()
                .on_color(AnsiColors::Green)
                .to_string(),
        ),
        D(
            "0.2.0".red().to_string(),
            "2021-06-19".green().on_truecolor(8, 100, 30).to_string(),
            "false".to_string(),
            "API changes".yellow().to_string(),
        ),
        D(
            "0.1.4".white().to_string(),
            "2021-06-07".red().on_truecolor(8, 10, 30).to_string(),
            "false".to_string(),
            "display_with attribute"
                .red()
                .on_color(AnsiColors::Black)
                .to_string(),
        ),
    ];

    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Modify::new(Full).with(Alignment::left()))
        .with(MaxWidth::wrapping(57))
        .to_string();

    println!("{}", table);

    assert_eq!(
        table,
        concat!(
            "| ver | published_d | is_act | major_feature            |\n",
            "| sio | ate         | ive    |                          |\n",
            "| n   |             |        |                          |\n",
            "|-----+-------------+--------+--------------------------|\n",
            "| \u{1b}[31m0.2\u{1b}[39m | \u{1b}[48;2;8;10;30m\u{1b}[31m2021-06-23\u{1b}[0m\u{1b}[0m  | true   | \u{1b}[34m\u{1b}[42m#[header(inline)] attrib\u{1b}[39m\u{1b}[49m |\n",
            "| \u{1b}[31m.1\u{1b}[39m  |             |        | \u{1b}[34m\u{1b}[42mute\u{1b}[39m\u{1b}[49m                      |\n",
            "| \u{1b}[31m0.2\u{1b}[39m | \u{1b}[48;2;8;100;30m\u{1b}[32m2021-06-19\u{1b}[0m\u{1b}[0m  | false  | \u{1b}[33mAPI changes\u{1b}[0m              |\n",
            "| \u{1b}[31m.0\u{1b}[39m  |             |        |                          |\n",
            "| \u{1b}[37m0.1\u{1b}[39m | \u{1b}[48;2;8;10;30m\u{1b}[31m2021-06-07\u{1b}[0m\u{1b}[0m  | false  | \u{1b}[40m\u{1b}[31mdisplay_with attribute\u{1b}[0m\u{1b}[0m   |\n",
            "| \u{1b}[37m.4\u{1b}[39m  |             |        |                          |\n",
        )
    );
    assert!(is_lines_equal(&table, 57));

    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Modify::new(Full).with(Alignment::left()))
        .with(MaxWidth::wrapping(57).keep_words())
        .to_string();

    println!("{}", table);

    assert_eq!(
        table,
        concat!(
            "| ver | published_d | is_act | major_feature            |\n",
            "| sio | ate         | ive    |                          |\n",
            "| n   |             |        |                          |\n",
            "|-----+-------------+--------+--------------------------|\n",
            "| \u{1b}[31m0.2\u{1b}[39m | \u{1b}[48;2;8;10;30m\u{1b}[31m2021-06-23\u{1b}[0m\u{1b}[0m  | true   | \u{1b}[34m\u{1b}[42m#[header(inline)] \u{1b}[39m\u{1b}[49m       |\n",
            "| \u{1b}[31m.1\u{1b}[39m  |             |        | \u{1b}[34m\u{1b}[42mattrib\u{1b}[39m\u{1b}[49m\u{1b}[34m\u{1b}[42mute\u{1b}[39m\u{1b}[49m                |\n",
            "| \u{1b}[31m0.2\u{1b}[39m | \u{1b}[48;2;8;100;30m\u{1b}[32m2021-06-19\u{1b}[0m\u{1b}[0m  | false  | \u{1b}[33mAPI changes\u{1b}[0m              |\n",
            "| \u{1b}[31m.0\u{1b}[39m  |             |        |                          |\n",
            "| \u{1b}[37m0.1\u{1b}[39m | \u{1b}[48;2;8;10;30m\u{1b}[31m2021-06-07\u{1b}[0m\u{1b}[0m  | false  | \u{1b}[40m\u{1b}[31mdisplay_with attribute\u{1b}[0m\u{1b}[0m   |\n",
            "| \u{1b}[37m.4\u{1b}[39m  |             |        |                          |\n",
        )
    );
    assert!(is_lines_equal(&table, 57));
}

#[cfg(feature = "color")]
#[test]
fn truncating_as_total_multiline_color() {
    use owo_colors::{AnsiColors, OwoColorize};

    #[derive(Tabled)]
    struct D(
        #[tabled(rename = "version")] String,
        #[tabled(rename = "published_date")] String,
        #[tabled(rename = "is_active")] String,
        #[tabled(rename = "major_feature")] String,
    );

    let data = vec![
        D(
            "0.2.1".red().to_string(),
            "2021-06-23".red().on_truecolor(8, 10, 30).to_string(),
            "true".to_string(),
            "#[header(inline)] attribute"
                .blue()
                .on_color(AnsiColors::Green)
                .to_string(),
        ),
        D(
            "0.2.0".red().to_string(),
            "2021-06-19".green().on_truecolor(8, 100, 30).to_string(),
            "false".to_string(),
            "API changes".yellow().to_string(),
        ),
        D(
            "0.1.4".white().to_string(),
            "2021-06-07".red().on_truecolor(8, 10, 30).to_string(),
            "false".to_string(),
            "display_with attribute"
                .red()
                .on_color(AnsiColors::Black)
                .to_string(),
        ),
    ];

    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Modify::new(Full).with(Alignment::left()))
        .with(MaxWidth::truncating(57))
        .to_string();

    println!("{}", table);

    assert_eq!(
        table,
        concat!(
            "| ver | published_d | is_act | major_feature            |\n",
            "|-----+-------------+--------+--------------------------|\n",
            "| \u{1b}[31m0.2\u{1b}[39m | \u{1b}[48;2;8;10;30m\u{1b}[31m2021-06-23\u{1b}[0m\u{1b}[0m  | true   | \u{1b}[42m\u{1b}[34m#[header(inline)] attrib\u{1b}[39m\u{1b}[49m |\n",
            "| \u{1b}[31m0.2\u{1b}[39m | \u{1b}[48;2;8;100;30m\u{1b}[32m2021-06-19\u{1b}[0m\u{1b}[0m  | false  | \u{1b}[33mAPI changes\u{1b}[0m              |\n",
            "| \u{1b}[37m0.1\u{1b}[39m | \u{1b}[48;2;8;10;30m\u{1b}[31m2021-06-07\u{1b}[0m\u{1b}[0m  | false  | \u{1b}[40m\u{1b}[31mdisplay_with attribute\u{1b}[0m\u{1b}[0m   |\n",
        )
    );
    assert!(is_lines_equal(&table, 57));
}

#[test]
fn min_width_works_with_right_alignment() {
    let json = r#"
    {
        "some": "random",
        "json": [
            { "1": "2" },
            { "1": "2" },
            { "1": "2" }
        ]
    }
    "#;

    let table = Table::new([json])
        .with(Style::github_markdown())
        .with(MinWidth::new(50))
        .with(
            Modify::new(Full)
                .with(Alignment::right())
                .with(TrimStrategy::None),
        );

    assert_eq!(
        table.to_string(),
        concat!(
            "|                      &str                      |\n",
            "|------------------------------------------------|\n",
            "|                                                |\n",
            "|     {                                          |\n",
            "|         \"some\": \"random\",                      |\n",
            "|         \"json\": [                              |\n",
            "|             { \"1\": \"2\" },                      |\n",
            "|             { \"1\": \"2\" },                      |\n",
            "|             { \"1\": \"2\" }                       |\n",
            "|         ]                                      |\n",
            "|     }                                          |\n",
            "|                                                |\n",
        )
    );
    assert!(is_lines_equal(&table.to_string(), 50));

    let table = table.with(Modify::new(Full).with(TrimStrategy::Horizontal));

    assert_eq!(
        table.to_string(),
        concat!(
            "|                                           &str |\n",
            "|------------------------------------------------|\n",
            "|                                                |\n",
            "|                          {                     |\n",
            "|                              \"some\": \"random\", |\n",
            "|                              \"json\": [         |\n",
            "|                                  { \"1\": \"2\" }, |\n",
            "|                                  { \"1\": \"2\" }, |\n",
            "|                                  { \"1\": \"2\" }  |\n",
            "|                              ]                 |\n",
            "|                          }                     |\n",
            "|                                                |\n",
        )
    );
    assert!(is_lines_equal(&table.to_string(), 50));

    let table = table.with(Modify::new(Full).with(TrimStrategy::Both));

    println!("{}", table);

    assert_eq!(
        table.to_string(),
        concat!(
            "|                                           &str |\n",
            "|------------------------------------------------|\n",
            "|                          {                     |\n",
            "|                              \"some\": \"random\", |\n",
            "|                              \"json\": [         |\n",
            "|                                  { \"1\": \"2\" }, |\n",
            "|                                  { \"1\": \"2\" }, |\n",
            "|                                  { \"1\": \"2\" }  |\n",
            "|                              ]                 |\n",
            "|                          }                     |\n",
            "|                                                |\n",
            "|                                                |\n",
        )
    );
    assert!(is_lines_equal(&table.to_string(), 50));

    let table = Table::new([json])
        .with(Style::github_markdown())
        .with(MinWidth::new(50))
        .with(
            Modify::new(Full)
                .with(Alignment::center())
                .with(TrimStrategy::None),
        );

    println!("{}", table);

    assert_eq!(
        table.to_string(),
        concat!(
            "|           &str                                 |\n",
            "|------------------------------------------------|\n",
            "|                                                |\n",
            "|     {                                          |\n",
            "|         \"some\": \"random\",                      |\n",
            "|         \"json\": [                              |\n",
            "|             { \"1\": \"2\" },                      |\n",
            "|             { \"1\": \"2\" },                      |\n",
            "|             { \"1\": \"2\" }                       |\n",
            "|         ]                                      |\n",
            "|     }                                          |\n",
            "|                                                |\n",
        )
    );
    assert!(is_lines_equal(&table.to_string(), 50));

    let table = table.with(Modify::new(Full).with(TrimStrategy::Horizontal));

    assert_eq!(
        table.to_string(),
        concat!(
            "|                      &str                      |\n",
            "|------------------------------------------------|\n",
            "|                                                |\n",
            "|               {                                |\n",
            "|                   \"some\": \"random\",            |\n",
            "|                   \"json\": [                    |\n",
            "|                       { \"1\": \"2\" },            |\n",
            "|                       { \"1\": \"2\" },            |\n",
            "|                       { \"1\": \"2\" }             |\n",
            "|                   ]                            |\n",
            "|               }                                |\n",
            "|                                                |\n",
        )
    );
    assert!(is_lines_equal(&table.to_string(), 50));

    let table = table.with(Modify::new(Full).with(TrimStrategy::Both));

    println!("{}", table);

    assert_eq!(
        table.to_string(),
        concat!(
            "|                      &str                      |\n",
            "|------------------------------------------------|\n",
            "|               {                                |\n",
            "|                   \"some\": \"random\",            |\n",
            "|                   \"json\": [                    |\n",
            "|                       { \"1\": \"2\" },            |\n",
            "|                       { \"1\": \"2\" },            |\n",
            "|                       { \"1\": \"2\" }             |\n",
            "|                   ]                            |\n",
            "|               }                                |\n",
            "|                                                |\n",
            "|                                                |\n",
        )
    );
    assert!(is_lines_equal(&table.to_string(), 50));
}

#[test]
fn min_width_with_span_1() {
    let data = [
        ["0", "1"],
        ["a long string which will affect min width logic", ""],
        ["2", "3"],
    ];

    let table = Table::new(data)
        .with(Style::github_markdown())
        .with(Modify::new(Cell(1, 0)).with(Span::column(2)))
        .with(MinWidth::new(100))
        .to_string();

    assert_eq!(
        table,
        concat!(
            "|                                   0                                    |            1            |\n",
            "|------------------------------------------------------------------------+-------------------------|\n",
            "|                                                0                                                 |\n",
            "|            a long string which will affect min width logic             |                         |\n",
            "|                                   2                                    |            3            |\n",
        )
    );
    assert!(is_lines_equal(&table, 100));
}

#[test]
fn min_width_with_span_2() {
    let data = [
        ["0", "1"],
        ["a long string which will affect min width logic", ""],
        ["2", "3"],
    ];

    let table = Table::new(data)
        .with(Style::github_markdown())
        .with(Modify::new(Cell(2, 0)).with(Span::column(2)))
        .with(MinWidth::new(100))
        .to_string();

    assert_eq!(
        table,
        concat!(
            "|                        0                        |                       1                        |\n",
            "|-------------------------------------------------+------------------------------------------------|\n",
            "|                        0                        |                       1                        |\n",
            "|                         a long string which will affect min width logic                          |\n",
            "|                        2                        |                       3                        |\n",
        )
    );
    assert!(is_lines_equal(&table, 100));
}
