use crate::util::create_vector;
use tabled::{
    Alignment, Cell, Column, Full, MaxWidth, MinWidth, Modify, Object, Panel, Row, Style, Table,
    TotalWidth,
};

mod util;

#[test]
fn max_width() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Modify::new(Column(1..).not(Row(..1))).with(MaxWidth::truncating(1)))
        .to_string();

    let expected = concat!(
        "| N | column 0 | column 1 | column 2 |\n",
        "|---+----------+----------+----------|\n",
        "| 0 |    0     |    0     |    0     |\n",
        "| 1 |    1     |    1     |    1     |\n",
        "| 2 |    2     |    2     |    2     |\n",
    );

    assert_eq!(table, expected);
}

#[test]
fn max_width_with_suffix() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Modify::new(Column(1..).not(Row(..1))).with(MaxWidth::truncating(2).suffix("...")))
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
fn max_width_doesnt_icrease_width_if_it_is_smaller() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Modify::new(Column(1..).not(Row(..1))).with(MaxWidth::truncating(50)))
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
fn max_width_wrapped() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::github_markdown())
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

#[test]
fn max_width_wrapped_does_nothing_if_str_is_smaller() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Modify::new(Column(1..).not(Row(..1))).with(MaxWidth::wrapping(100)))
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
fn max_width_wrapped_keep_words() {
    let data = vec!["this is a long sentence"];
    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Modify::new(Full).with(Alignment::left()))
        .with(Modify::new(Full).with(MaxWidth::wrapping(17).keep_words()))
        .to_string();

    let expected = concat!(
        "| &str            |\n",
        "|-----------------|\n",
        "| this is a long  |\n",
        "| sentence        |\n",
    );

    assert_eq!(table, expected);

    let data = vec!["this is a long  sentence"];
    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Modify::new(Full).with(Alignment::left()))
        .with(Modify::new(Full).with(MaxWidth::wrapping(17).keep_words()))
        .to_string();

    let expected = concat!(
        "| &str             |\n",
        "|------------------|\n",
        "| this is a long   |\n",
        "| sentence         |\n",
    );

    assert_eq!(table, expected);

    let data = vec!["this is a long   sentence"];
    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Modify::new(Full).with(Alignment::left()))
        .with(Modify::new(Full).with(MaxWidth::wrapping(17).keep_words()))
        .to_string();

    let expected = concat!(
        "| &str              |\n",
        "|-------------------|\n",
        "| this is a long    |\n",
        "| sentence          |\n",
    );

    assert_eq!(table, expected);

    let data = vec!["this is a long    sentence"];
    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Modify::new(Full).with(Alignment::left()))
        .with(Modify::new(Full).with(MaxWidth::wrapping(17).keep_words()))
        .to_string();

    // 'sentence' doesnt have a space ' sentence' because we use left alignment
    let expected = concat!(
        "| &str              |\n",
        "|-------------------|\n",
        "| this is a long    |\n",
        "| sentence          |\n",
    );

    assert_eq!(table, expected);

    let data = vec!["this"];
    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Modify::new(Full).with(MaxWidth::wrapping(10).keep_words()))
        .to_string();

    let expected = concat!("| &str |\n", "|------|\n", "| this |\n",);

    assert_eq!(table, expected);
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
        "| String          |\n",
        "|-----------------|\n",
        "| \u{1b}[32m\u{1b}[40mthis is a long \u{1b}[39m\u{1b}[49m |\n",
        "| \u{1b}[32m\u{1b}[40mse\u{1b}[39m\u{1b}[49m\u{1b}[32m\u{1b}[40mntence\u{1b}[39m\u{1b}[49m        |\n",
    );

    assert_eq!(table, expected);

    let data = vec!["this is a long  sentence".on_black().green().to_string()];
    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Modify::new(Full).with(Alignment::left()))
        .with(Modify::new(Full).with(MaxWidth::wrapping(17).keep_words()))
        .to_string();

    let expected = concat!(
        "| String           |\n",
        "|------------------|\n",
        "| \u{1b}[32m\u{1b}[40mthis is a long  \u{1b}[39m\u{1b}[49m |\n",
        "| \u{1b}[32m\u{1b}[40ms\u{1b}[39m\u{1b}[49m\u{1b}[32m\u{1b}[40mentence\u{1b}[39m\u{1b}[49m         |\n",
    );

    println!("{}", table);

    assert_eq!(table, expected);

    let data = vec!["this is a long   sentence".on_black().green().to_string()];
    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Modify::new(Full).with(Alignment::left()))
        .with(Modify::new(Full).with(MaxWidth::wrapping(17).keep_words()))
        .to_string();

    let expected = concat!(
        "| String            |\n",
        "|-------------------|\n",
        "| \u{1b}[32m\u{1b}[40mthis is a long   \u{1b}[39m\u{1b}[49m |\n",
        "| \u{1b}[32m\u{1b}[40msentence\u{1b}[39m\u{1b}[49m          |\n",
    );

    assert_eq!(table, expected);

    let data = vec!["this is a long    sentence".on_black().green().to_string()];
    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Modify::new(Full).with(Alignment::left()))
        .with(Modify::new(Full).with(MaxWidth::wrapping(17).keep_words()))
        .to_string();

    let expected = concat!(
        "| String            |\n",
        "|-------------------|\n",
        "| \u{1b}[32m\u{1b}[40mthis is a long   \u{1b}[39m\u{1b}[49m |\n",
        "| \u{1b}[32m\u{1b}[40m sentence\u{1b}[39m\u{1b}[49m         |\n",
    );

    assert_eq!(table, expected);

    let data = vec!["this".on_black().green().to_string()];
    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Modify::new(Full).with(MaxWidth::wrapping(10).keep_words()))
        .to_string();

    let expected = concat!(
        "| String |\n",
        "|--------|\n",
        "|  \u{1b}[32m\u{1b}[40mthis\u{1b}[39m\u{1b}[49m  |\n",
    );

    assert_eq!(table, expected);
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

    println!("{}", table);

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

    println!("{}", table);

    assert_eq!(expected, table);
}

#[test]
fn min_width() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Modify::new(Row(..1)).with(MinWidth::new(12)))
        .to_string();

    let expected = concat!(
        "|      N       |   column 0   |   column 1   |   column 2   |\n",
        "|--------------+--------------+--------------+--------------|\n",
        "|      0       |     0-0      |     0-1      |     0-2      |\n",
        "|      1       |     1-0      |     1-1      |     1-2      |\n",
        "|      2       |     2-0      |     2-1      |     2-2      |\n",
    );

    assert_eq!(table, expected);
}

#[test]
fn min_width_with_filler() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Modify::new(Row(..1)).with(MinWidth::new(12).fill_with('.')))
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
        .with(Modify::new(Cell(0, 0)).with(MinWidth::new(5)))
        .to_string();

    let expected = concat!(
        "|   N   | column 0 | column 1 | column 2 |\n",
        "|-------+----------+----------+----------|\n",
        "|   0   |   0-0    |   0-1    |   0-2    |\n",
        "|   1   |   1-0    |   1-1    |   1-2    |\n",
        "|   2   |   2-0    |   2-1    |   2-2    |\n",
    );

    assert_eq!(table, expected);
}

#[test]
fn min_width_on_smaller_content() {
    let data = create_vector::<3, 3>();

    assert_eq!(
        Table::new(&data)
            .with(Style::github_markdown())
            .with(Modify::new(Row(..1)).with(MinWidth::new(1)))
            .to_string(),
        Table::new(&data).with(Style::github_markdown()).to_string()
    );
}

#[test]
fn min_with_max_width() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Modify::new(Row(..1)).with(MinWidth::new(3)))
        .with(Modify::new(Row(..1)).with(MaxWidth::truncating(3)))
        .to_string();

    let expected = concat!(
        "|  N  | col | col | col |\n",
        "|-----+-----+-----+-----|\n",
        "|  0  | 0-0 | 0-1 | 0-2 |\n",
        "|  1  | 1-0 | 1-1 | 1-2 |\n",
        "|  2  | 2-0 | 2-1 | 2-2 |\n",
    );

    assert_eq!(table, expected);
}

#[test]
fn min_with_max_width_truncate_suffix() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Modify::new(Row(..1)).with(MinWidth::new(3)))
        .with(Modify::new(Row(..1)).with(MaxWidth::truncating(3).suffix("...")))
        .to_string();

    let expected = concat!(
        "|  N  | col... | col... | col... |\n",
        "|-----+--------+--------+--------|\n",
        "|  0  |  0-0   |  0-1   |  0-2   |\n",
        "|  1  |  1-0   |  1-1   |  1-2   |\n",
        "|  2  |  2-0   |  2-1   |  2-2   |\n",
    );

    assert_eq!(table, expected);
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

    let expected = concat!(
        "|   String   |\n",
        "|------------|\n",
        "|    \u{1b}[31masd\u{1b}[0m     |\n",
        "|    \u{1b}[34mzxc\u{1b}[0m     |\n",
        "|   \u{1b}[32m\u{1b}[40masdasd\u{1b}[0m\u{1b}[0m   |\n",
    );

    let table = Table::new(data)
        .with(Style::github_markdown())
        .with(Modify::new(Full).with(MinWidth::new(10)))
        .to_string();

    println!("{}", table);

    assert_eq!(expected, table);
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
        .with(TotalWidth::new(80))
        .to_string();

    let expected = concat!(
        "|      N       |      column 0       |      column 1      |      column 2      |\n",
        "|--------------+---------------------+--------------------+--------------------|\n",
        "|      0       |         0-0         |        0-1         |        0-2         |\n",
        "|      1       |         1-0         |        1-1         |        1-2         |\n",
        "|      2       |         2-0         |        2-1         |        2-2         |\n",
    );

    assert_eq!(table, expected);
}

#[test]
fn total_width_big_with_panel() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Panel("Hello World", 0))
        .with(Modify::new(Full).with(Alignment::center_horizontal()))
        .with(Style::github_markdown())
        .with(TotalWidth::new(80))
        .to_string();

    let expected = concat!(
        "|                                Hello World                                |\n",
        "|-------------+--------------------+--------------------+-------------------|\n",
        "|      N      |      column 0      |      column 1      |     column 2      |\n",
        "|      0      |        0-0         |        0-1         |        0-2        |\n",
        "|      1      |        1-0         |        1-1         |        1-2        |\n",
        "|      2      |        2-0         |        2-1         |        2-2        |\n",
    );

    assert_eq!(table, expected);
}

#[test]
fn total_width_small() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(TotalWidth::new(14))
        .to_string();

    let expected = concat!(
        "| N |  |  |  |\n",
        "|---+--+--+--|\n",
        "| 0 |  |  |  |\n",
        "| 1 |  |  |  |\n",
        "| 2 |  |  |  |\n",
    );

    assert_eq!(table, expected);
}

#[test]
fn total_width_smaller_then_content() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(TotalWidth::new(8))
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
        .with(Panel("Hello World", 0))
        .with(Modify::new(Full).with(Alignment::center_horizontal()))
        .with(Style::github_markdown())
        .with(TotalWidth::new(20))
        .to_string();

    let expected = concat!(
        "|   Hello World    |\n",
        "|---+----+----+----|\n",
        "| N | co | co | co |\n",
        "| 0 | 0- | 0- | 0- |\n",
        "| 1 | 1- | 1- | 1- |\n",
        "| 2 | 2- | 2- | 2- |\n",
    );

    assert_eq!(table, expected);
}
