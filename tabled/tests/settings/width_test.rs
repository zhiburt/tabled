#![cfg(feature = "std")]

use tabled::{
    grid::util::string::string_width_multiline,
    settings::{
        formatting::{TabSize, TrimStrategy},
        object::{Columns, Object, Rows, Segment},
        peaker::{PriorityMax, PriorityMin},
        width::{Justify, MinWidth, SuffixLimit, Width},
        Alignment, Margin, Modify, Padding, Panel, Settings, Span, Style,
    },
};

use crate::matrix::Matrix;
use testing_table::{is_lines_equal, static_table, test_table};

#[cfg(feature = "color")]
use ::{ansi_str::AnsiStr, owo_colors::OwoColorize};

#[cfg(all(feature = "derive", feature = "color"))]
use ::owo_colors::AnsiColors;

test_table!(
    max_width,
    Matrix::new(3, 3)
        .with(Style::markdown())
        .with(Modify::new(Columns::new(1..).not(Rows::single(0))).with(Width::truncate(1))),
    "| N | column 0 | column 1 | column 2 |"
    "|---|----------|----------|----------|"
    "| 0 |    0     |    0     |    0     |"
    "| 1 |    1     |    1     |    1     |"
    "| 2 |    2     |    2     |    2     |"
);

test_table!(
    max_width_with_suffix,
    Matrix::new(3, 3)
        .with(Style::markdown())
        .with(
            Modify::new(Columns::new(1..).not(Rows::single(0)))
                .with(Width::truncate(2).suffix("...")),
        ),
    "| N | column 0 | column 1 | column 2 |"
    "|---|----------|----------|----------|"
    "| 0 |    ..    |    ..    |    ..    |"
    "| 1 |    ..    |    ..    |    ..    |"
    "| 2 |    ..    |    ..    |    ..    |"
);

test_table!(
    max_width_doesnt_icrease_width_if_it_is_smaller,
    Matrix::new(3, 3)
        .with(Style::markdown())
        .with(Modify::new(Columns::new(1..).not(Rows::single(0))).with(Width::truncate(50))),
    "| N | column 0 | column 1 | column 2 |"
    "|---|----------|----------|----------|"
    "| 0 |   0-0    |   0-1    |   0-2    |"
    "| 1 |   1-0    |   1-1    |   1-2    |"
    "| 2 |   2-0    |   2-1    |   2-2    |"
);

test_table!(
    max_width_wrapped,
    Matrix::new(3, 3)
        .with(Style::markdown())
        .with(Modify::new(Columns::new(1..).not(Rows::single(0))).with(Width::wrap(2))),
    "| N | column 0 | column 1 | column 2 |"
    "|---|----------|----------|----------|"
    "| 0 |    0-    |    0-    |    0-    |"
    "|   |    0     |    1     |    2     |"
    "| 1 |    1-    |    1-    |    1-    |"
    "|   |    0     |    1     |    2     |"
    "| 2 |    2-    |    2-    |    2-    |"
    "|   |    0     |    1     |    2     |"
);

test_table!(
    max_width_wrapped_does_nothing_if_str_is_smaller,
    Matrix::new(3, 3)
        .with(Style::markdown())
        .with(Modify::new(Columns::new(1..).not(Rows::single(0))).with(Width::wrap(100))),
    "| N | column 0 | column 1 | column 2 |"
    "|---|----------|----------|----------|"
    "| 0 |   0-0    |   0-1    |   0-2    |"
    "| 1 |   1-0    |   1-1    |   1-2    |"
    "| 2 |   2-0    |   2-1    |   2-2    |"
);

test_table!(
    max_width_wrapped_keep_words_0,
    {
        let table = Matrix::iter(vec!["this is a long sentence"])
            .with(Style::markdown())
            .with(Modify::new(Segment::all()).with(Alignment::left()))
            .with(Modify::new(Segment::all()).with(Width::wrap(17).keep_words()))
            .to_string();

        assert!(is_lines_equal(&table, 17 + 2 + 2));

        table
    },
    "| &str              |"
    "|-------------------|"
    "| this is a long    |"
    "| sentence          |"
);

test_table!(
    max_width_wrapped_keep_words_1,
    {
        let table = Matrix::iter(vec!["this is a long  sentence"])
            .with(Style::markdown())
            .with(Modify::new(Segment::all()).with(Alignment::left()))
            .with(Modify::new(Segment::all()).with(Width::wrap(17).keep_words()))
            .to_string();

        assert!(is_lines_equal(&table, 17 + 2 + 2));

        table
    },
    "| &str              |"
    "|-------------------|"
    "| this is a long    |"
    "| sentence          |"
);

test_table!(
    max_width_wrapped_keep_words_2,
    {
        let table = Matrix::iter(vec!["this is a long   sentence"])
            .with(Style::markdown())
            .with(Modify::new(Segment::all()).with(Alignment::left()))
            .with(Modify::new(Segment::all()).with(Width::wrap(17).keep_words()))
            .to_string();

        assert!(is_lines_equal(&table, 17 + 2 + 2));

        table
    },
    "| &str              |"
    "|-------------------|"
    "| this is a long    |"
    "| sentence          |"
);

#[cfg(feature = "color")]
test_table!(
    max_width_wrapped_keep_words_3,
    {
        let table = Matrix::iter(vec!["this is a long    sentence"])
            .with(Style::markdown())
            .with(Modify::new(Segment::all()).with(Alignment::left()))
            .with(Modify::new(Segment::all()).with(Width::wrap(17).keep_words()))
            .to_string();

        assert!(is_lines_equal(&table, 17 + 2 + 2));

        table
    },
    // 'sentence' doesn't have a space ' sentence' because we use left alignment
    "| &str              |"
    "|-------------------|"
    "| this is a long    |"
    "|  sentence         |"
);

#[cfg(not(feature = "color"))]
test_table!(
    max_width_wrapped_keep_words_3,
    {
        let table = Matrix::iter(vec!["this is a long    sentence"])
            .with(Style::markdown())
            .with(Modify::new(Segment::all()).with(Alignment::left()))
            .with(Modify::new(Segment::all()).with(Width::wrap(17).keep_words()))
            .to_string();

        assert!(is_lines_equal(&table, 17 + 2 + 2));

        table
    },
    // 'sentence' doesn't have a space ' sentence' because we use left alignment
    "| &str              |"
    "|-------------------|"
    "| this is a long    |"
    "| sentence          |"
);

test_table!(
    max_width_wrapped_keep_words_4,
    {
        let table = Matrix::iter(vec!["this"])
            .with(Style::markdown())
            .with(Modify::new(Segment::all()).with(Width::wrap(10).keep_words()))
            .to_string();

        assert!(is_lines_equal(&table, 8));

        table
    },
    "| &str |"
    "|------|"
    "| this |"
);

#[cfg(feature = "color")]
test_table!(
    max_width_wrapped_keep_words_color_0,
    {
        let table = Matrix::iter(vec!["this is a long sentence".on_black().green().to_string()])
            .with(Style::markdown())
            .with(Modify::new(Segment::all()).with(Alignment::left()))
            .with(Modify::new(Segment::all()).with(Width::wrap(17).keep_words()))
            .to_string();

        AnsiStr::ansi_strip(&table).to_string()
    },
    "| String            |"
    "|-------------------|"
    "| this is a long    |"
    "| sentence          |"
);

#[cfg(feature = "color")]
test_table!(
    max_width_wrapped_keep_words_color_0_1,
    Matrix::iter(vec!["this is a long sentence".on_black().green().to_string()])
        .with(Style::markdown())
        .with(Modify::new(Segment::all()).with(Alignment::left()))
        .with(Modify::new(Segment::all()).with(Width::wrap(17).keep_words())),
        "| String            |"
        "|-------------------|"
        "| \u{1b}[32m\u{1b}[40mthis is a long \u{1b}[39m\u{1b}[49m   |"
        "| \u{1b}[32m\u{1b}[40msentence\u{1b}[39m\u{1b}[49m          |"
);

#[cfg(feature = "color")]
test_table!(
    max_width_wrapped_keep_words_color_1,
    {
        let table = Matrix::iter(vec!["this is a long  sentence".on_black().green().to_string()])
            .with(Style::markdown())
            .with(Modify::new(Segment::all()).with(Alignment::left()))
            .with(Modify::new(Segment::all()).with(Width::wrap(17).keep_words()))
            .to_string();

        AnsiStr::ansi_strip(&table).to_string()
    },
    "| String            |"
    "|-------------------|"
    "| this is a long    |"
    "| sentence          |"
);

#[cfg(feature = "color")]
test_table!(
    max_width_wrapped_keep_words_color_1_1,
    Matrix::iter(vec!["this is a long  sentence".on_black().green().to_string()])
        .with(Style::markdown())
        .with(Modify::new(Segment::all()).with(Alignment::left()))
        .with(Modify::new(Segment::all()).with(Width::wrap(17).keep_words())),
    "| String            |"
    "|-------------------|"
    "| \u{1b}[32m\u{1b}[40mthis is a long  \u{1b}[39m\u{1b}[49m  |"
    "| \u{1b}[32m\u{1b}[40msentence\u{1b}[39m\u{1b}[49m          |"
);

#[cfg(feature = "color")]
test_table!(
    max_width_wrapped_keep_words_color_2,
    {
        let table = Matrix::iter(vec!["this is a long   sentence".on_black().green().to_string()])
            .with(Style::markdown())
            .with(Modify::new(Segment::all()).with(Alignment::left()))
            .with(Modify::new(Segment::all()).with(Width::wrap(17).keep_words()))
            .to_string();

        AnsiStr::ansi_strip(&table).to_string()
    },
    "| String            |"
    "|-------------------|"
    "| this is a long    |"
    "| sentence          |"
);

#[cfg(feature = "color")]
test_table!(
    max_width_wrapped_keep_words_color_2_1,
    Matrix::iter(vec!["this is a long   sentence".on_black().green().to_string()])
        .with(Style::markdown())
        .with(Modify::new(Segment::all()).with(Alignment::left()))
        .with(Modify::new(Segment::all()).with(Width::wrap(17).keep_words())),
    "| String            |"
    "|-------------------|"
    "| \u{1b}[32m\u{1b}[40mthis is a long   \u{1b}[39m\u{1b}[49m |"
    "| \u{1b}[32m\u{1b}[40msentence\u{1b}[39m\u{1b}[49m          |"
);

#[cfg(feature = "color")]
test_table!(
    max_width_wrapped_keep_words_color_3,
    {
        let table = Matrix::iter(vec!["this is a long    sentence".on_black().green().to_string()])
            .with(Style::markdown())
            .with(Modify::new(Segment::all()).with(Alignment::left()))
            .with(Modify::new(Segment::all()).with(Width::wrap(17).keep_words()))
            .to_string();

        AnsiStr::ansi_strip(&table).to_string()
    },
    "| String            |"
    "|-------------------|"
    "| this is a long    |"
    "|  sentence         |"
);

#[cfg(feature = "color")]
test_table!(
    max_width_wrapped_keep_words_color_3_1,
    Matrix::iter(vec!["this is a long    sentence".on_black().green().to_string()])
        .with(Style::markdown())
        .with(Modify::new(Segment::all()).with(Alignment::left()))
        .with(Modify::new(Segment::all()).with(Width::wrap(17).keep_words())),
    "| String            |"
    "|-------------------|"
    "| \u{1b}[32m\u{1b}[40mthis is a long   \u{1b}[39m\u{1b}[49m |"
    "| \u{1b}[32m\u{1b}[40m sentence\u{1b}[39m\u{1b}[49m         |"
);

#[cfg(feature = "color")]
test_table!(
    max_width_wrapped_keep_words_color_4,
    {
        let table = Matrix::iter(vec!["this".on_black().green().to_string()])
            .with(Style::markdown())
            .with(Modify::new(Segment::all()).with(Width::wrap(10).keep_words()))
            .to_string();

        AnsiStr::ansi_strip(&table).to_string()
    },
    "| String |"
    "|--------|"
    "|  this  |"
);

#[cfg(feature = "color")]
test_table!(
    max_width_wrapped_keep_words_color_4_1,
    Matrix::iter(vec!["this".on_black().green().to_string()])
        .with(Style::markdown())
        .with(Modify::new(Segment::all()).with(Width::wrap(10).keep_words())),
    "| String |"
    "|--------|"
    "|  \u{1b}[32;40mthis\u{1b}[0m  |"
);

test_table!(
    max_width_wrapped_keep_words_long_word,
    Matrix::iter(["this is a long sentencesentencesentence"])
        .with(Style::markdown())
        .with(Modify::new(Segment::all()).with(Alignment::left()))
        .with(Modify::new(Segment::all()).with(Width::wrap(17).keep_words())),
    "| &str              |"
    "|-------------------|"
    "| this is a long se |"
    "| ntencesentencesen |"
    "| tence             |"
);

#[cfg(feature = "color")]
#[test]
fn max_width_wrapped_keep_words_long_word_color() {
    let data = vec!["this is a long sentencesentencesentence"
        .on_black()
        .green()
        .to_string()];
    let table = Matrix::iter(data)
        .with(Style::markdown())
        .with(Modify::new(Segment::all()).with(Alignment::left()))
        .with(Modify::new(Segment::all()).with(Width::wrap(17).keep_words()))
        .to_string();

    assert_eq!(
        ansi_str::AnsiStr::ansi_strip(&table),
        static_table!(
            "| String            |"
            "|-------------------|"
            "| this is a long se |"
            "| ntencesentencesen |"
            "| tence             |"
        )
    );

    assert_eq!(
        table,
        static_table!(
            "| String            |"
            "|-------------------|"
            "| \u{1b}[32m\u{1b}[40mthis is a long se\u{1b}[39m\u{1b}[49m |"
            "| \u{1b}[32m\u{1b}[40mntencesentencesen\u{1b}[39m\u{1b}[49m |"
            "| \u{1b}[32m\u{1b}[40mtence\u{1b}[39m\u{1b}[49m             |"
        )
    );
}

#[cfg(feature = "color")]
#[test]
fn max_width_keep_words_1() {
    use tabled::settings::style::HorizontalLine;

    let table = Matrix::iter(["asdf"])
        .with(Width::wrap(7).keep_words())
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "+-----+"
            "| &st |"
            "| r   |"
            "+-----+"
            "| asd |"
            "| f   |"
            "+-----+"
        )
    );

    let table = Matrix::iter(["qweqw eqwe"])
        .with(Width::wrap(8).keep_words())
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "+------+"
            "| &str |"
            "+------+"
            "| qweq |"
            "| w    |"
            "| eqwe |"
            "+------+"
        )
    );

    let table = Matrix::iter([
        ["123 45678", "qweqw eqwe", "..."],
        ["0", "1", "..."],
        ["0", "1", "..."],
    ])
    .with(
        Style::modern()
            .remove_horizontal()
            .horizontals([HorizontalLine::new(1, Style::modern().get_horizontal())]),
    )
    .with(Width::wrap(21).keep_words().priority::<PriorityMax>())
    .with(Alignment::center())
    .to_string();

    assert_eq!(
        table,
        static_table!(
            "┌──────┬──────┬─────┐"
            "│  0   │  1   │  2  │"
            "├──────┼──────┼─────┤"
            "│ 123  │ qweq │ ... │"
            "│ 4567 │ w    │     │"
            "│ 8    │ eqwe │     │"
            "│  0   │  1   │ ... │"
            "│  0   │  1   │ ... │"
            "└──────┴──────┴─────┘"
        )
    );
}

#[cfg(feature = "color")]
#[test]
fn max_width_wrapped_collored() {
    let data = &[
        "asd".red().to_string(),
        "zxc2".blue().to_string(),
        "asdasd".on_black().green().to_string(),
    ];

    let table = Matrix::iter(data)
        .with(Style::markdown())
        .with(Modify::new(Segment::all()).with(Width::wrap(2)))
        .to_string();

    assert_eq!(
        table,
        "| St |\n| ri |\n| ng |\n|----|\n| \u{1b}[31mas\u{1b}[39m |\n| \u{1b}[31md\u{1b}[39m  |\n| \u{1b}[34mzx\u{1b}[39m |\n| \u{1b}[34mc2\u{1b}[39m |\n| \u{1b}[32m\u{1b}[40mas\u{1b}[39m\u{1b}[49m |\n| \u{1b}[32m\u{1b}[40mda\u{1b}[39m\u{1b}[49m |\n| \u{1b}[32m\u{1b}[40msd\u{1b}[39m\u{1b}[49m |"
    );
}

#[test]
fn dont_change_content_if_width_is_less_then_max_width() {
    let table = Matrix::new(3, 3)
        .with(Style::markdown())
        .with(Modify::new(Segment::all()).with(Width::truncate(1000).suffix("...")))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "| N | column 0 | column 1 | column 2 |"
            "|---|----------|----------|----------|"
            "| 0 |   0-0    |   0-1    |   0-2    |"
            "| 1 |   1-0    |   1-1    |   1-2    |"
            "| 2 |   2-0    |   2-1    |   2-2    |"
        )
    );
}

#[test]
fn max_width_with_emoji() {
    let data = &["🤠", "😳🥵🥶😱😨", "🚴🏻‍♀️🚴🏻🚴🏻‍♂️🚵🏻‍♀️🚵🏻🚵🏻‍♂️"];

    let table = Matrix::iter(data)
        .with(Style::markdown())
        .with(Modify::new(Segment::all()).with(Width::truncate(6).suffix("...")))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "|  &str  |"
            "|--------|"
            "|   🤠   |"
            "| 😳�... |"
            "| 🚴�... |"
        )
    );
}

#[cfg(feature = "color")]
#[test]
fn color_chars_are_stripped() {
    let data = &[
        "asd".red().to_string(),
        "zxc".blue().to_string(),
        "asdasd".on_black().green().to_string(),
    ];

    let table = Matrix::iter(data)
        .with(Style::markdown())
        .with(Modify::new(Segment::all()).with(Width::truncate(4).suffix("...")))
        .to_string();

    assert_eq!(
        ansi_str::AnsiStr::ansi_strip(&table),
        static_table!(
            "| S... |"
            "|------|"
            "| asd  |"
            "| zxc  |"
            "| a... |"
        )
    );

    assert_eq!(
        table,
        "| S... |\n|------|\n| \u{1b}[31masd\u{1b}[39m  |\n| \u{1b}[34mzxc\u{1b}[39m  |\n| \u{1b}[32;40ma\u{1b}[39m\u{1b}[49m... |",
    );
}

#[test]
fn min_width() {
    let mut table = Matrix::table(3, 3);
    table
        .with(Style::markdown())
        .with(Modify::new(Rows::single(0)).with(MinWidth::new(12)));

    assert_eq!(
        table.to_string(),
        static_table!(
            "| N            | column 0     | column 1     | column 2     |"
            "|--------------|--------------|--------------|--------------|"
            "|      0       |     0-0      |     0-1      |     0-2      |"
            "|      1       |     1-0      |     1-1      |     1-2      |"
            "|      2       |     2-0      |     2-1      |     2-2      |"
        ),
    );

    table.with(Modify::new(Segment::all()).with(TrimStrategy::None));

    assert_eq!(
        table.to_string(),
        static_table!(
            "| N            | column 0     | column 1     | column 2     |"
            "|--------------|--------------|--------------|--------------|"
            "|      0       |     0-0      |     0-1      |     0-2      |"
            "|      1       |     1-0      |     1-1      |     1-2      |"
            "|      2       |     2-0      |     2-1      |     2-2      |"
        ),
    );
}

#[test]
fn min_width_with_filler() {
    let table = Matrix::new(3, 3)
        .with(Style::markdown())
        .with(Modify::new(Rows::single(0)).with(MinWidth::new(12).fill_with('.')))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "| N........... | column 0.... | column 1.... | column 2.... |"
            "|--------------|--------------|--------------|--------------|"
            "|      0       |     0-0      |     0-1      |     0-2      |"
            "|      1       |     1-0      |     1-1      |     1-2      |"
            "|      2       |     2-0      |     2-1      |     2-2      |"
        )
    );
}

#[test]
fn min_width_one_column() {
    let mut table = Matrix::table(3, 3);
    table
        .with(Style::markdown())
        .with(Modify::new((0, 0)).with(MinWidth::new(5)));

    assert_eq!(
        table.to_string(),
        static_table!(
            "| N     | column 0 | column 1 | column 2 |"
            "|-------|----------|----------|----------|"
            "|   0   |   0-0    |   0-1    |   0-2    |"
            "|   1   |   1-0    |   1-1    |   1-2    |"
            "|   2   |   2-0    |   2-1    |   2-2    |"
        )
    );

    table.with(Modify::new(Segment::all()).with(TrimStrategy::None));

    assert_eq!(
        table.to_string(),
        static_table!(
            "| N     | column 0 | column 1 | column 2 |"
            "|-------|----------|----------|----------|"
            "|   0   |   0-0    |   0-1    |   0-2    |"
            "|   1   |   1-0    |   1-1    |   1-2    |"
            "|   2   |   2-0    |   2-1    |   2-2    |"
        )
    );
}

#[test]
fn min_width_on_smaller_content() {
    assert_eq!(
        Matrix::new(3, 3)
            .with(Style::markdown())
            .with(Modify::new(Rows::single(0)).with(MinWidth::new(1)))
            .to_string(),
        Matrix::new(3, 3).with(Style::markdown()).to_string()
    );
}

#[test]
fn min_with_max_width() {
    let mut table = Matrix::table(3, 3);
    table
        .with(Style::markdown())
        .with(Modify::new(Rows::single(0)).with(MinWidth::new(3)))
        .with(Modify::new(Rows::single(0)).with(Width::truncate(3)));

    assert_eq!(
        table.to_string(),
        static_table!(
            "| N   | col | col | col |"
            "|-----|-----|-----|-----|"
            "|  0  | 0-0 | 0-1 | 0-2 |"
            "|  1  | 1-0 | 1-1 | 1-2 |"
            "|  2  | 2-0 | 2-1 | 2-2 |"
        )
    );

    table.with(Modify::new(Segment::all()).with(TrimStrategy::None));

    assert_eq!(
        table.to_string(),
        static_table!(
            "| N   | col | col | col |"
            "|-----|-----|-----|-----|"
            "|  0  | 0-0 | 0-1 | 0-2 |"
            "|  1  | 1-0 | 1-1 | 1-2 |"
            "|  2  | 2-0 | 2-1 | 2-2 |"
        )
    );
}

#[test]
fn min_with_max_width_truncate_suffix() {
    let mut table = Matrix::table(3, 3);
    table
        .with(Style::markdown())
        .with(Modify::new(Rows::single(0)).with(MinWidth::new(3)))
        .with(Modify::new(Rows::single(0)).with(Width::truncate(3).suffix("...")));

    assert_eq!(
        table.to_string(),
        static_table!(
            "| N   | ... | ... | ... |"
            "|-----|-----|-----|-----|"
            "|  0  | 0-0 | 0-1 | 0-2 |"
            "|  1  | 1-0 | 1-1 | 1-2 |"
            "|  2  | 2-0 | 2-1 | 2-2 |"
        )
    );

    table.with(Modify::new(Segment::all()).with(TrimStrategy::None));

    assert_eq!(
        table.to_string(),
        static_table!(
            "| N   | ... | ... | ... |"
            "|-----|-----|-----|-----|"
            "|  0  | 0-0 | 0-1 | 0-2 |"
            "|  1  | 1-0 | 1-1 | 1-2 |"
            "|  2  | 2-0 | 2-1 | 2-2 |"
        )
    );
}

#[test]
fn min_with_max_width_truncate_suffix_limit_replace() {
    let table = Matrix::new(3, 3)
        .with(Style::markdown())
        .with(
            Modify::new(Rows::single(0)).with(
                Width::truncate(3)
                    .suffix("...")
                    .suffix_limit(SuffixLimit::Replace('x')),
            ),
        )
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "| N | xxx | xxx | xxx |"
            "|---|-----|-----|-----|"
            "| 0 | 0-0 | 0-1 | 0-2 |"
            "| 1 | 1-0 | 1-1 | 1-2 |"
            "| 2 | 2-0 | 2-1 | 2-2 |"
        )
    );
}

#[test]
fn min_with_max_width_truncate_suffix_limit_cut() {
    let table = Matrix::new(3, 3)
        .with(Style::markdown())
        .with(
            Modify::new(Rows::single(0)).with(
                Width::truncate(3)
                    .suffix("qwert")
                    .suffix_limit(SuffixLimit::Cut),
            ),
        )
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "| N | qwe | qwe | qwe |"
            "|---|-----|-----|-----|"
            "| 0 | 0-0 | 0-1 | 0-2 |"
            "| 1 | 1-0 | 1-1 | 1-2 |"
            "| 2 | 2-0 | 2-1 | 2-2 |"
        )
    );
}

#[test]
fn min_with_max_width_truncate_suffix_limit_ignore() {
    let table = Matrix::new(3, 3)
        .with(Style::markdown())
        .with(
            Modify::new(Rows::single(0)).with(
                Width::truncate(3)
                    .suffix("qwert")
                    .suffix_limit(SuffixLimit::Ignore),
            ),
        )
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "| N | col | col | col |"
            "|---|-----|-----|-----|"
            "| 0 | 0-0 | 0-1 | 0-2 |"
            "| 1 | 1-0 | 1-1 | 1-2 |"
            "| 2 | 2-0 | 2-1 | 2-2 |"
        )
    );
}

#[cfg(feature = "color")]
#[test]
fn min_with_max_width_truncate_suffix_try_color() {
    let data = &[
        "asd".red().to_string(),
        "zxc".blue().to_string(),
        "asdasd".on_black().green().to_string(),
    ];

    let table = Matrix::iter(data)
        .with(Style::markdown())
        .with(Width::truncate(7).suffix("..").suffix_try_color(true))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "| S.. |"
            "|-----|"
            "| \u{1b}[31masd\u{1b}[39m |"
            "| \u{1b}[34mzxc\u{1b}[39m |"
            "| \u{1b}[32;40ma\u{1b}[39m\u{1b}[49m\u{1b}[32m\u{1b}[40m..\u{1b}[39m\u{1b}[49m |"
        )
    );
}

#[cfg(feature = "color")]
#[test]
fn min_width_color() {
    let data = &[
        "asd".red().to_string(),
        "zxc".blue().to_string(),
        "asdasd".on_black().green().to_string(),
    ];

    let table = Matrix::iter(data)
        .with(Style::markdown())
        .with(Modify::new(Segment::all()).with(MinWidth::new(10)))
        .to_string();

    assert_eq!(
        ansi_str::AnsiStr::ansi_strip(&table),
        static_table!(
            "| String     |"
            "|------------|"
            "| asd        |"
            "| zxc        |"
            "| asdasd     |"
        )
    );

    assert_eq!(
        table,
        "| String     |\n|------------|\n| \u{1b}[31masd\u{1b}[39m        |\n| \u{1b}[34mzxc\u{1b}[39m        |\n| \u{1b}[32;40masdasd\u{1b}[0m     |",
    );
}

#[cfg(feature = "color")]
#[test]
fn min_width_color_with_smaller_then_width() {
    let data = &[
        "asd".red().to_string(),
        "zxc".blue().to_string(),
        "asdasd".on_black().green().to_string(),
    ];

    assert_eq!(
        Matrix::iter(data)
            .with(Modify::new(Segment::all()).with(MinWidth::new(1)))
            .to_string(),
        Matrix::iter(data).to_string()
    );
}

#[test]
fn total_width_big() {
    let table = Matrix::new(3, 3)
        .with(Style::markdown())
        .with(Width::truncate(80))
        .with(MinWidth::new(80))
        .to_string();

    assert_eq!(string_width_multiline(&table), 80);
    assert_eq!(
        table,
        static_table!(
            "|      N       |      column 0       |      column 1      |      column 2      |"
            "|--------------|---------------------|--------------------|--------------------|"
            "|      0       |         0-0         |        0-1         |        0-2         |"
            "|      1       |         1-0         |        1-1         |        1-2         |"
            "|      2       |         2-0         |        2-1         |        2-2         |"
        )
    );

    let table = Matrix::new(3, 3)
        .with(Style::markdown())
        .with(Modify::new(Segment::all()).with(TrimStrategy::None))
        .with(Settings::new(Width::truncate(80), Width::increase(80)))
        .to_string();

    assert_eq!(string_width_multiline(&table), 80);
    assert_eq!(
        table,
        static_table!(
            "|      N       |      column 0       |      column 1      |      column 2      |"
            "|--------------|---------------------|--------------------|--------------------|"
            "|      0       |         0-0         |        0-1         |        0-2         |"
            "|      1       |         1-0         |        1-1         |        1-2         |"
            "|      2       |         2-0         |        2-1         |        2-2         |"
        )
    );
}

#[test]
fn total_width_big_with_panel() {
    let table = Matrix::new(3, 3)
        .with(Panel::horizontal(0, "Hello World"))
        .with(
            Modify::new(Segment::all())
                .with(Alignment::center())
                .with(Padding::zero()),
        )
        .with(Style::markdown())
        .with(Width::truncate(80))
        .with(MinWidth::new(80))
        .to_string();

    assert!(is_lines_equal(&table, 80));
    assert_eq!(
        table,
        static_table!(
            "|                                 Hello World                                  |"
            "|--------------|---------------------|--------------------|--------------------|"
            "|      N       |      column 0       |      column 1      |      column 2      |"
            "|      0       |         0-0         |        0-1         |        0-2         |"
            "|      1       |         1-0         |        1-1         |        1-2         |"
            "|      2       |         2-0         |        2-1         |        2-2         |"
        )
    );
}

#[test]
fn total_width_big_with_panel_with_wrapping_doesnt_affect_increase() {
    let table1 = Matrix::new(3, 3)
        .with(Panel::horizontal(0, "Hello World"))
        .with(Modify::new(Segment::all()).with(Alignment::center()))
        .with(Style::markdown())
        .with(Width::wrap(80))
        .with(MinWidth::new(80))
        .to_string();

    let table2 = Matrix::new(3, 3)
        .with(Panel::horizontal(0, "Hello World"))
        .with(Modify::new(Segment::all()).with(Alignment::center()))
        .with(Style::markdown())
        .with(Width::truncate(80))
        .with(MinWidth::new(80))
        .to_string();

    assert_eq!(table1, table2);
}

#[test]
fn total_width_small() {
    let table = Matrix::new(3, 3)
        .with(Style::markdown())
        .with(Width::truncate(14))
        .with(MinWidth::new(14))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "|  |  |  | c |"
            "|--|--|--|---|"
            "|  |  |  | 0 |"
            "|  |  |  | 1 |"
            "|  |  |  | 2 |"
        )
    );
    assert!(is_lines_equal(&table, 14));
}

#[test]
fn total_width_smaller_then_content() {
    let table = Matrix::new(3, 3)
        .with(Style::markdown())
        .with(Width::truncate(8))
        .with(MinWidth::new(8))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "|  |  |  |  |"
            "|--|--|--|--|"
            "|  |  |  |  |"
            "|  |  |  |  |"
            "|  |  |  |  |"
        )
    );
}

#[test]
fn total_width_small_with_panel() {
    let table = Matrix::new(3, 3)
        .with(Modify::new(Segment::all()).with(Alignment::center()))
        .with(Style::markdown())
        .with(Width::truncate(20))
        .with(MinWidth::new(20))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "|  | co | co | col |"
            "|--|----|----|-----|"
            "|  | 0- | 0- | 0-2 |"
            "|  | 1- | 1- | 1-2 |"
            "|  | 2- | 2- | 2-2 |"
        )
    );
    assert!(is_lines_equal(&table, 20));

    let table = Matrix::iter(Vec::<usize>::new())
        .with(Panel::horizontal(0, "Hello World"))
        .with(
            Modify::new(Segment::all())
                .with(Alignment::center())
                .with(Padding::zero()),
        )
        .with(Width::truncate(5))
        .with(MinWidth::new(5))
        .to_string();

    assert_eq!(
        table,
        static_table!("+---+" "|Hel|" "+---+" "|usi|" "+---+")
    );
    assert!(is_lines_equal(&table, 5));

    let table = Matrix::table(1, 2)
        .with(Panel::horizontal(0, "Hello World"))
        .with(Modify::new(Segment::all()).with(Alignment::center()))
        .with(Style::markdown())
        .with(Width::truncate(20))
        .with(MinWidth::new(20))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "|   Hello World    |"
            "|--|-------|-------|"
            "|  | colum | colum |"
            "|  |  0-0  |  0-1  |"
        )
    );
    assert!(is_lines_equal(&table, 20));

    let table = Matrix::new(3, 3)
        .with(Panel::horizontal(0, "Hello World"))
        .with(Modify::new(Segment::all()).with(Alignment::center()))
        .with(Style::markdown())
        .with(Width::truncate(20))
        .with(MinWidth::new(20))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "|   Hello World    |"
            "|--|----|----|-----|"
            "|  | co | co | col |"
            "|  | 0- | 0- | 0-2 |"
            "|  | 1- | 1- | 1-2 |"
            "|  | 2- | 2- | 2-2 |"
        )
    );
    assert!(is_lines_equal(&table, 20));

    let table = Matrix::new(3, 3)
        .with(Panel::horizontal(0, "Hello World"))
        .with(Modify::new(Segment::all()).with(Alignment::center()))
        .with(Style::markdown())
        .with(Width::truncate(6))
        .with(MinWidth::new(6))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "| Hello Wor |"
            "|--|--|--|--|"
            "|  |  |  |  |"
            "|  |  |  |  |"
            "|  |  |  |  |"
            "|  |  |  |  |"
        )
    );
    assert!(is_lines_equal(&table, 13));

    let table = Matrix::new(3, 3)
        .with(Panel::horizontal(0, "Hello World"))
        .with(Modify::new(Segment::all()).with(Alignment::center()))
        .with(Style::markdown())
        .with(Width::truncate(14))
        .with(MinWidth::new(14))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "| Hello Worl |"
            "|--|--|--|---|"
            "|  |  |  | c |"
            "|  |  |  | 0 |"
            "|  |  |  | 1 |"
            "|  |  |  | 2 |"
        )
    );
    assert!(is_lines_equal(&table, 14));

    let table = Matrix::new(3, 3)
        .with(Panel::horizontal(0, "Hello World 123"))
        .with(Modify::new(Segment::all()).with(Alignment::center()))
        .with(Style::markdown())
        .with(Width::truncate(14))
        .with(MinWidth::new(14))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "| Hello Worl |"
            "|--|--|--|---|"
            "|  |  |  | c |"
            "|  |  |  | 0 |"
            "|  |  |  | 1 |"
            "|  |  |  | 2 |"
        )
    );
    assert!(is_lines_equal(&table, 14));
}

#[cfg(feature = "color")]
#[test]
fn total_width_wrapping() {
    let table = Matrix::new(3, 3)
        .with(Modify::new(Segment::all()).with(Alignment::center()))
        .with(Style::markdown())
        .with(Width::wrap(20))
        .with(MinWidth::new(20))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "|  | co | co | col |"
            "|  | lu | lu | umn |"
            "|  | mn | mn |  2  |"
            "|  |  0 |  1 |     |"
            "|--|----|----|-----|"
            "|  | 0- | 0- | 0-2 |"
            "|  | 0  | 1  |     |"
            "|  | 1- | 1- | 1-2 |"
            "|  | 0  | 1  |     |"
            "|  | 2- | 2- | 2-2 |"
            "|  | 0  | 1  |     |"
        )
    );
    assert!(is_lines_equal(&table, 20));

    let table = Matrix::new(3, 3)
        .insert((3, 2), "some loong string")
        .with(Modify::new(Segment::all()).with(Alignment::center()))
        .with(Style::markdown())
        .with(Width::wrap(20).keep_words())
        .with(MinWidth::new(20))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "|  |  | column  |  |"
            "|  |  | 1       |  |"
            "|--|--|---------|--|"
            "|  |  |   0-1   |  |"
            "|  |  |   1-1   |  |"
            "|  |  | some    |  |"
            "|  |  | loong   |  |"
            "|  |  | string  |  |"
        )
    );
    assert!(is_lines_equal(&table, 20));
}

#[test]
fn total_width_small_with_panel_using_wrapping() {
    let table = Matrix::new(3, 3)
        .with(Panel::horizontal(0, "Hello World"))
        .with(Modify::new(Segment::all()).with(Alignment::center()))
        .with(Style::markdown())
        .with(Width::wrap(20))
        .with(MinWidth::new(20))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "|   Hello World    |"
            "|--|----|----|-----|"
            "|  | co | co | col |"
            "|  | lu | lu | umn |"
            "|  | mn | mn |  2  |"
            "|  |  0 |  1 |     |"
            "|  | 0- | 0- | 0-2 |"
            "|  | 0  | 1  |     |"
            "|  | 1- | 1- | 1-2 |"
            "|  | 0  | 1  |     |"
            "|  | 2- | 2- | 2-2 |"
            "|  | 0  | 1  |     |"
        )
    );
    assert!(is_lines_equal(&table, 20));

    let table = Matrix::new(3, 3)
        .with(Panel::horizontal(0, "Hello World"))
        .with(Modify::new(Segment::all()).with(Alignment::center()))
        .with(Style::markdown())
        .with(Width::wrap(14))
        .with(MinWidth::new(14))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "| Hello Worl |"
            "| d          |"
            "|--|--|--|---|"
            "|  |  |  | c |"
            "|  |  |  | o |"
            "|  |  |  | l |"
            "|  |  |  | u |"
            "|  |  |  | m |"
            "|  |  |  | n |"
            "|  |  |  |   |"
            "|  |  |  | 2 |"
            "|  |  |  | 0 |"
            "|  |  |  | - |"
            "|  |  |  | 2 |"
            "|  |  |  | 1 |"
            "|  |  |  | - |"
            "|  |  |  | 2 |"
            "|  |  |  | 2 |"
            "|  |  |  | - |"
            "|  |  |  | 2 |"
        )
    );
    assert!(is_lines_equal(&table, 14));

    let table = Matrix::new(3, 3)
        .with(Panel::horizontal(0, "Hello World 123"))
        .with(Modify::new(Segment::all()).with(Alignment::center()))
        .with(Style::markdown())
        .with(Width::wrap(14))
        .with(MinWidth::new(14))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "| Hello Worl |"
            "| d 123      |"
            "|--|--|--|---|"
            "|  |  |  | c |"
            "|  |  |  | o |"
            "|  |  |  | l |"
            "|  |  |  | u |"
            "|  |  |  | m |"
            "|  |  |  | n |"
            "|  |  |  |   |"
            "|  |  |  | 2 |"
            "|  |  |  | 0 |"
            "|  |  |  | - |"
            "|  |  |  | 2 |"
            "|  |  |  | 1 |"
            "|  |  |  | - |"
            "|  |  |  | 2 |"
            "|  |  |  | 2 |"
            "|  |  |  | - |"
            "|  |  |  | 2 |"
        )
    );
    assert!(is_lines_equal(&table, 14));
}

#[test]
fn max_width_with_span() {
    let mut table = Matrix::new(3, 3).insert((1, 1), "a long string").to_table();
    table
        .with(Style::psql())
        .with(Modify::new((1, 1)).with(Span::column(2)))
        .with(Modify::new((2, 2)).with(Span::column(2)));

    table.with(Width::truncate(40));

    assert_eq!(
        table.to_string(),
        static_table!(
            " N | column 0 | column 1 | column 2 "
            "---+----------+----------+----------"
            " 0 |    a long string    |   0-2    "
            " 1 |   1-0    |         1-1         "
            " 2 |   2-0    |   2-1    |   2-2    "
        )
    );
    assert!(is_lines_equal(&table.to_string(), 36));

    table.with(Width::truncate(20));

    assert_eq!(
        table.to_string(),
        static_table!(
            "  | col | col | col "
            "--+-----+-----+-----"
            "  | a long st | 0-2 "
            "  | 1-0 |    1-1    "
            "  | 2-0 | 2-1 | 2-2 "
        )
    );
    assert!(is_lines_equal(&table.to_string(), 20));

    table.with(Width::truncate(10));

    assert_eq!(
        table.to_string(),
        static_table!(
            "  |  |  |  "
            "--+--+--+--"
            "  | a l |  "
            "  |  | 1-1 "
            "  |  |  |  "
        )
    );
    assert!(is_lines_equal(&table.to_string(), 11));
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

    let mut table = Matrix::iter([json]);
    table
        .with(Style::markdown())
        .with(
            Modify::new(Segment::all())
                .with(Alignment::right())
                .with(TrimStrategy::None),
        )
        .with(MinWidth::new(50));

    assert_eq!(string_width_multiline(&table.to_string()), 50);
    assert_eq!(
        table.to_string(),
        static_table!(
            "|                                           &str |"
            "|------------------------------------------------|"
            "|                                                |"
            "|                          {                     |"
            "|                              \"some\": \"random\", |"
            "|                              \"json\": [         |"
            "|                                  { \"1\": \"2\" }, |"
            "|                                  { \"1\": \"2\" }, |"
            "|                                  { \"1\": \"2\" }  |"
            "|                              ]                 |"
            "|                          }                     |"
            "|                                                |"
        )
    );

    table
        .with(Modify::new(Segment::all()).with(TrimStrategy::Horizontal))
        .with(MinWidth::new(50));

    assert_eq!(
        table.to_string(),
        static_table!(
            r#"|                                           &str |"#
            r#"|------------------------------------------------|"#
            r#"|                                                |"#
            r#"|                              {                 |"#
            r#"|                              "some": "random", |"#
            r#"|                              "json": [         |"#
            r#"|                              { "1": "2" },     |"#
            r#"|                              { "1": "2" },     |"#
            r#"|                              { "1": "2" }      |"#
            r#"|                              ]                 |"#
            r#"|                              }                 |"#
            r#"|                                                |"#
        )
    );
    assert!(is_lines_equal(&table.to_string(), 50));

    table
        .with(Modify::new(Segment::all()).with(TrimStrategy::Both))
        .with(MinWidth::new(50));

    assert_eq!(
        table.to_string(),
        static_table!(
            r#"|                                           &str |"#
            r#"|------------------------------------------------|"#
            r#"|                              {                 |"#
            r#"|                              "some": "random", |"#
            r#"|                              "json": [         |"#
            r#"|                              { "1": "2" },     |"#
            r#"|                              { "1": "2" },     |"#
            r#"|                              { "1": "2" }      |"#
            r#"|                              ]                 |"#
            r#"|                              }                 |"#
            r#"|                                                |"#
            r#"|                                                |"#
        )
    );
    assert!(is_lines_equal(&table.to_string(), 50));

    let mut table = Matrix::iter([json]);
    table
        .with(Style::markdown())
        .with(
            Modify::new(Segment::all())
                .with(Alignment::center())
                .with(TrimStrategy::None),
        )
        .with(MinWidth::new(50));

    assert_eq!(
        table.to_string(),
        static_table!(
            "|                      &str                      |"
            "|------------------------------------------------|"
            "|                                                |"
            "|               {                                |"
            "|                   \"some\": \"random\",            |"
            "|                   \"json\": [                    |"
            "|                       { \"1\": \"2\" },            |"
            "|                       { \"1\": \"2\" },            |"
            "|                       { \"1\": \"2\" }             |"
            "|                   ]                            |"
            "|               }                                |"
            "|                                                |"
        )
    );
    assert_eq!(string_width_multiline(&table.to_string()), 50);

    table
        .with(Modify::new(Segment::all()).with(TrimStrategy::Horizontal))
        .with(MinWidth::new(50));

    assert_eq!(
        table.to_string(),
        static_table!(
            r#"|                      &str                      |"#
            r#"|------------------------------------------------|"#
            r#"|                                                |"#
            r#"|               {                                |"#
            r#"|               "some": "random",                |"#
            r#"|               "json": [                        |"#
            r#"|               { "1": "2" },                    |"#
            r#"|               { "1": "2" },                    |"#
            r#"|               { "1": "2" }                     |"#
            r#"|               ]                                |"#
            r#"|               }                                |"#
            r#"|                                                |"#
        )
    );
    assert!(is_lines_equal(&table.to_string(), 50));

    table
        .with(Modify::new(Segment::all()).with(TrimStrategy::Both))
        .with(MinWidth::new(50));

    assert_eq!(
        table.to_string(),
        static_table!(
            r#"|                      &str                      |"#
            r#"|------------------------------------------------|"#
            r#"|               {                                |"#
            r#"|               "some": "random",                |"#
            r#"|               "json": [                        |"#
            r#"|               { "1": "2" },                    |"#
            r#"|               { "1": "2" },                    |"#
            r#"|               { "1": "2" }                     |"#
            r#"|               ]                                |"#
            r#"|               }                                |"#
            r#"|                                                |"#
            r#"|                                                |"#
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

    let table = Matrix::iter(data)
        .with(Style::markdown())
        .with(Modify::new((1, 0)).with(Span::column(2)))
        .with(MinWidth::new(100))
        .to_string();

    assert_eq!(string_width_multiline(&table), 100);
    assert_eq!(
        table,
        static_table!(
            "|                                   0                                    |            1            |"
            "|------------------------------------------------------------------------|-------------------------|"
            "|                                                0                                                 |"
            "|            a long string which will affect min width logic             |                         |"
            "|                                   2                                    |            3            |"
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

    let table = Matrix::iter(data)
        .with(Style::markdown())
        .with(Modify::new((2, 0)).with(Span::column(2)))
        .with(MinWidth::new(100))
        .to_string();

    assert_eq!(string_width_multiline(&table), 100);
    assert_eq!(
        table,
        static_table!(
            "|                        0                        |                       1                        |"
            "|-------------------------------------------------|------------------------------------------------|"
            "|                        0                        |                       1                        |"
            "|                         a long string which will affect min width logic                          |"
            "|                        2                        |                       3                        |"
        )
    );
}

#[test]
fn justify_width_constant_test() {
    let table = Matrix::new(3, 3)
        .with(Style::markdown())
        .with(Justify::new(3))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "| N   | col | col | col |"
            "|-----|-----|-----|-----|"
            "| 0   | 0-0 | 0-1 | 0-2 |"
            "| 1   | 1-0 | 1-1 | 1-2 |"
            "| 2   | 2-0 | 2-1 | 2-2 |"
        )
    );
}

#[test]
fn justify_width_constant_different_sizes_test() {
    let table = Matrix::new(3, 3)
        .insert((1, 1), "Hello World")
        .insert((3, 2), "multi\nline string\n")
        .with(Style::markdown())
        .with(Justify::new(3))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "| N   | col | col | col |"
            "|-----|-----|-----|-----|"
            "| 0   | Hel | 0-1 | 0-2 |"
            "| 1   | 1-0 | 1-1 | 1-2 |"
            "| 2   | 2-0 | mul | 2-2 |"
        )
    );
}

#[test]
fn justify_width_constant_0_test() {
    let table = Matrix::new(3, 3)
        .with(Style::markdown())
        .with(Justify::new(0))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "|  |  |  |  |"
            "|--|--|--|--|"
            "|  |  |  |  |"
            "|  |  |  |  |"
            "|  |  |  |  |"
        )
    );
}

#[test]
fn justify_width_min_test() {
    let table = Matrix::new(3, 3)
        .with(Style::markdown())
        .with(Justify::min())
        .to_string();

    println!("{table}");

    assert_eq!(
        table,
        static_table!(
            "| N | c | c | c |"
            "|---|---|---|---|"
            "| 0 | 0 | 0 | 0 |"
            "| 1 | 1 | 1 | 1 |"
            "| 2 | 2 | 2 | 2 |"
        )
    );
}

#[test]
fn justify_width_max_test() {
    let table = Matrix::new(3, 3)
        .with(Style::markdown())
        .with(Justify::max())
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "| N        | column 0 | column 1 | column 2 |"
            "|----------|----------|----------|----------|"
            "| 0        | 0-0      | 0-1      | 0-2      |"
            "| 1        | 1-0      | 1-1      | 1-2      |"
            "| 2        | 2-0      | 2-1      | 2-2      |"
        )
    );
}

#[test]
fn max_width_when_cell_has_tabs() {
    let table = Matrix::new(3, 3)
        .insert((2, 1), "\tHello\tWorld\t")
        .with(TabSize::new(4))
        .with(Style::markdown())
        .with(Modify::new(Columns::new(..)).with(Width::truncate(1)))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "| N | c | c | c |"
            "|---|---|---|---|"
            "| 0 | 0 | 0 | 0 |"
            "| 1 |   | 1 | 1 |"
            "| 2 | 2 | 2 | 2 |"
        )
    );
}

#[test]
fn max_width_table_when_cell_has_tabs() {
    let table = Matrix::new(3, 3)
        .insert((2, 1), "\tHello\tWorld\t")
        .with(TabSize::new(4))
        .with(Style::markdown())
        .with(Width::truncate(15))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "|  | co |  |  |"
            "|--|----|--|--|"
            "|  | 0- |  |  |"
            "|  |    |  |  |"
            "|  | 2- |  |  |"
        )
    );
}

// WE GOT [["", "column 0", "column 1 ", "column 2  "], ["", "0-0     ", "0-1      ", "0-2       "], ["", "Hello World With Big Line; Here w", "1-1", "1-2"], ["", "2-0     ", "Hello World With Big L", "2-2"]]
// [2, 10, 11, 12]
// 40 55 40

// BEFORE ADJ [2, 10, 11, 12]

// WE GOT [["", "column 0", "column 1", "column 2"], ["", "0-0", "0-1", "0-2"], ["", "Hello World With Big Line; Here w", "1-1", "1-2"], ["", "2-0", "Hello World With Big L", "2-2"]]
// [2, 11, 12, 11]
// 41 55 40

// adj [2, 10, 10, 10]

#[test]
fn max_width_truncate_with_big_span() {
    let table = Matrix::new(3, 3)
        .insert((2, 1), "Hello World With Big Line; Here we gooooooo")
        .with(Style::markdown())
        .with(Modify::new((2, 1)).with(Span::column(3)))
        .with(Width::truncate(40))
        .to_string();

    assert_eq!(string_width_multiline(&table), 40);
    assert_eq!(
        table,
        static_table!(
            "|  | column 0  | column 1  | column 2  |"
            "|--|-----------|-----------|-----------|"
            "|  |    0-0    |    0-1    |    0-2    |"
            "|  | Hello World With Big Line; Here w |"
            "|  |    2-0    |    2-1    |    2-2    |"
        )
    );

    let table = Matrix::new(3, 3)
        .insert((2, 1), "Hello World With Big Line; Here we gooooooo")
        .insert((3, 2), "Hello World With Big Line; Here")
        .with(Style::markdown())
        .with(Modify::new((2, 1)).with(Span::column(3)))
        .with(Modify::new((3, 2)).with(Span::column(2)))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "| N | column 0  |    column 1    |    column 2    |"
            "|---|-----------|----------------|----------------|"
            "| 0 |    0-0    |      0-1       |      0-2       |"
            "| 1 | Hello World With Big Line; Here we gooooooo |"
            "| 2 |    2-0    | Hello World With Big Line; Here |"
        )
    );

    let table = Matrix::new(3, 3)
        .insert((2, 1), "Hello World With Big Line; Here we gooooooo")
        .insert((3, 2), "Hello World With Big Line; Here")
        .with(Style::markdown())
        .with(Modify::new((2, 1)).with(Span::column(3)))
        .with(Modify::new((3, 2)).with(Span::column(2)))
        .with(Width::truncate(40))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "|  | colum |  column 1   |  column 2   |"
            "|--|-------|-------------|-------------|"
            "|  |  0-0  |     0-1     |     0-2     |"
            "|  | Hello World With Big Line; Here w |"
            "|  |  2-0  | Hello World With Big Line |"
        )
    );
    assert_eq!(string_width_multiline(&table), 40);

    let table = Matrix::new(3, 3)
        .insert((2, 1), "Hello World With Big Line; Here we gooooooo")
        .insert((3, 2), "Hello World With Big Line; Here")
        .with(Style::markdown())
        .with(Modify::new((2, 1)).with(Span::column(2)))
        .with(Modify::new((3, 2)).with(Span::column(2)))
        .with(Width::truncate(40))
        .to_string();

    assert_eq!(string_width_multiline(&table), 40);
    assert_eq!(
        table,
        static_table!(
            "|  |   column 0    |   column 1    | c |"
            "|--|---------------|---------------|---|"
            "|  |      0-0      |      0-1      | 0 |"
            "|  | Hello World With Big Line; He | 1 |"
            "|  |      2-0      | Hello World With  |"
        )
    );

    let table = Matrix::new(3, 3)
        .insert((2, 1), "Hello World With Big Line; Here w")
        .insert((3, 2), "Hello World With Big L")
        .with(Style::markdown())
        .with(Modify::new((2, 1)).with(Span::column(3)))
        .with(Modify::new((3, 2)).with(Span::column(2)))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "| N | column 0 |  column 1  | column 2  |"
            "|---|----------|------------|-----------|"
            "| 0 |   0-0    |    0-1     |    0-2    |"
            "| 1 | Hello World With Big Line; Here w |"
            "| 2 |   2-0    | Hello World With Big L |"
        )
    );
}

#[test]
fn max_width_truncate_priority_max() {
    let table = Matrix::new(3, 3)
        .insert((2, 1), "Hello World With Big Line")
        .with(Style::markdown())
        .with(Width::truncate(35).priority::<PriorityMax>())
        .to_string();

    assert!(is_lines_equal(&table, 35));
    assert_eq!(
        table,
        static_table!(
            "| N | column  | column  | column  |"
            "|---|---------|---------|---------|"
            "| 0 |   0-0   |   0-1   |   0-2   |"
            "| 1 | Hello W |   1-1   |   1-2   |"
            "| 2 |   2-0   |   2-1   |   2-2   |"
        )
    );

    let table = Matrix::new(3, 3)
        .insert((2, 1), "Hello World With Big Line")
        .with(Style::markdown())
        .with(Width::truncate(20).priority::<PriorityMax>())
        .to_string();

    assert!(is_lines_equal(&table, 20));
    assert_eq!(
        table,
        static_table!(
            "| N | co | co | co |"
            "|---|----|----|----|"
            "| 0 | 0- | 0- | 0- |"
            "| 1 | He | 1- | 1- |"
            "| 2 | 2- | 2- | 2- |"
        )
    );

    let table = Matrix::new(3, 3)
        .insert((2, 1), "Hello World With Big Line")
        .with(Style::markdown())
        .with(Width::truncate(0).priority::<PriorityMax>())
        .to_string();

    assert!(is_lines_equal(&table, 13));
    assert_eq!(
        table,
        static_table!(
            "|  |  |  |  |"
            "|--|--|--|--|"
            "|  |  |  |  |"
            "|  |  |  |  |"
            "|  |  |  |  |"
        )
    );
}

#[test]
fn max_width_truncate_priority_max_with_span() {
    let table = Matrix::new(3, 3)
        .insert((2, 1), "Hello World With Big Line")
        .with(Style::markdown())
        .with(Modify::new((2, 1)).with(Span::column(2)))
        .with(Width::truncate(15).priority::<PriorityMax>())
        .to_string();

    assert!(is_lines_equal(&table, 15));
    assert_eq!(
        table,
        static_table!(
            "| N | c |  |  |"
            "|---|---|--|--|"
            "| 0 | 0 |  |  |"
            "| 1 | Hell |  |"
            "| 2 | 2 |  |  |"
        )
    );
}

#[test]
fn max_width_wrap_priority_max() {
    let table = Matrix::new(3, 3)
        .insert((2, 1), "Hello World With Big Line")
        .with(Style::markdown())
        .with(Width::wrap(35).priority::<PriorityMax>())
        .to_string();

    assert!(is_lines_equal(&table, 35));
    assert_eq!(
        table,
        static_table!(
            "| N | column  | column  | column  |"
            "|   | 0       | 1       | 2       |"
            "|---|---------|---------|---------|"
            "| 0 |   0-0   |   0-1   |   0-2   |"
            "| 1 | Hello W |   1-1   |   1-2   |"
            "|   | orld Wi |         |         |"
            "|   | th Big  |         |         |"
            "|   | Line    |         |         |"
            "| 2 |   2-0   |   2-1   |   2-2   |"
        )
    );

    let table = Matrix::new(3, 3)
        .insert((2, 1), "Hello World With Big Line")
        .with(Style::markdown())
        .with(Width::wrap(20).priority::<PriorityMax>())
        .to_string();

    assert!(is_lines_equal(&table, 20));
    assert_eq!(
        table,
        static_table!(
            "| N | co | co | co |"
            "|   | lu | lu | lu |"
            "|   | mn | mn | mn |"
            "|   |  0 |  1 |  2 |"
            "|---|----|----|----|"
            "| 0 | 0- | 0- | 0- |"
            "|   | 0  | 1  | 2  |"
            "| 1 | He | 1- | 1- |"
            "|   | ll | 1  | 2  |"
            "|   | o  |    |    |"
            "|   | Wo |    |    |"
            "|   | rl |    |    |"
            "|   | d  |    |    |"
            "|   | Wi |    |    |"
            "|   | th |    |    |"
            "|   |  B |    |    |"
            "|   | ig |    |    |"
            "|   |  L |    |    |"
            "|   | in |    |    |"
            "|   | e  |    |    |"
            "| 2 | 2- | 2- | 2- |"
            "|   | 0  | 1  | 2  |"
        )
    );

    let table = Matrix::new(3, 3)
        .insert((2, 1), "Hello World With Big Line")
        .with(Style::markdown())
        .with(Width::wrap(0).priority::<PriorityMax>())
        .to_string();

    assert!(is_lines_equal(&table, 13));
    assert_eq!(
        table,
        static_table!(
            "|  |  |  |  |"
            "|--|--|--|--|"
            "|  |  |  |  |"
            "|  |  |  |  |"
            "|  |  |  |  |"
        )
    );
}

#[test]
fn max_width_wrap_priority_max_with_span() {
    let table = Matrix::new(3, 3)
        .insert((2, 1), "Hello World With Big Line")
        .with(Style::markdown())
        .with(Modify::new((2, 1)).with(Span::column(2)))
        .with(Width::wrap(15).priority::<PriorityMax>())
        .to_string();

    assert!(is_lines_equal(&table, 15));
    assert_eq!(
        table,
        static_table!(
            "| N | c |  |  |"
            "|   | o |  |  |"
            "|   | l |  |  |"
            "|   | u |  |  |"
            "|   | m |  |  |"
            "|   | n |  |  |"
            "|   |   |  |  |"
            "|   | 0 |  |  |"
            "|---|---|--|--|"
            "| 0 | 0 |  |  |"
            "|   | - |  |  |"
            "|   | 0 |  |  |"
            "| 1 | Hell |  |"
            "|   | o Wo |  |"
            "|   | rld  |  |"
            "|   | With |  |"
            "|   |  Big |  |"
            "|   |  Lin |  |"
            "|   | e    |  |"
            "| 2 | 2 |  |  |"
            "|   | - |  |  |"
            "|   | 0 |  |  |"
        )
    );
}

#[test]
fn max_width_truncate_priority_min() {
    let table = Matrix::new(3, 3)
        .insert((2, 1), "Hello World With Big Line")
        .with(Style::markdown())
        .with(Width::truncate(35).priority::<PriorityMin>())
        .to_string();

    assert!(is_lines_equal(&table, 35));
    assert_eq!(
        table,
        static_table!(
            "|  |        column 0        |  |  |"
            "|--|------------------------|--|--|"
            "|  |          0-0           |  |  |"
            "|  | Hello World With Big L |  |  |"
            "|  |          2-0           |  |  |"
        )
    );

    let table = Matrix::new(3, 3)
        .insert((2, 1), "Hello World With Big Line")
        .with(Style::markdown())
        .with(Width::truncate(20).priority::<PriorityMin>())
        .to_string();

    assert!(is_lines_equal(&table, 20));
    assert_eq!(
        table,
        static_table!(
            "|  | column  |  |  |"
            "|--|---------|--|--|"
            "|  |   0-0   |  |  |"
            "|  | Hello W |  |  |"
            "|  |   2-0   |  |  |"
        )
    );

    let table = Matrix::new(3, 3)
        .insert((2, 1), "Hello World With Big Line")
        .with(Style::markdown())
        .with(Width::truncate(0).priority::<PriorityMin>())
        .to_string();

    assert!(is_lines_equal(&table, 13));
    assert_eq!(
        table,
        static_table!(
            "|  |  |  |  |"
            "|--|--|--|--|"
            "|  |  |  |  |"
            "|  |  |  |  |"
            "|  |  |  |  |"
        )
    );
}

#[test]
fn max_width_truncate_priority_min_with_span() {
    let table = Matrix::new(3, 3)
        .insert((2, 1), "Hello World With Big Line")
        .with(Style::markdown())
        .with(Modify::new((2, 1)).with(Span::column(2)))
        .with(Width::truncate(15).priority::<PriorityMin>())
        .to_string();

    assert!(is_lines_equal(&table, 15));
    assert_eq!(
        table,
        static_table!(
            "|  |  | co |  |"
            "|--|--|----|--|"
            "|  |  | 0- |  |"
            "|  | Hello |  |"
            "|  |  | 2- |  |"
        )
    );

    let table = Matrix::new(3, 3)
        .insert((2, 1), "Hello World With Big Line")
        .with(Style::markdown())
        .with(Modify::new((2, 1)).with(Span::column(2)))
        .with(Width::truncate(17).priority::<PriorityMin>())
        .to_string();

    assert!(is_lines_equal(&table, 17));
    assert_eq!(
        table,
        static_table!(
            "|  |  | colu |  |"
            "|--|--|------|--|"
            "|  |  | 0-1  |  |"
            "|  | Hello W |  |"
            "|  |  | 2-1  |  |"
        )
    );
}

#[test]
fn max_width_wrap_priority_min() {
    let table = Matrix::new(3, 3)
        .insert((2, 1), "Hello World With Big Line")
        .with(Style::markdown())
        .with(Width::wrap(35).priority::<PriorityMin>())
        .to_string();

    assert!(is_lines_equal(&table, 35));
    assert_eq!(
        table,
        static_table!(
            "|  |        column 0        |  |  |"
            "|--|------------------------|--|--|"
            "|  |          0-0           |  |  |"
            "|  | Hello World With Big L |  |  |"
            "|  | ine                    |  |  |"
            "|  |          2-0           |  |  |"
        )
    );

    let table = Matrix::new(3, 3)
        .insert((2, 1), "Hello World With Big Line")
        .with(Style::markdown())
        .with(Width::wrap(20).priority::<PriorityMin>())
        .to_string();

    assert!(is_lines_equal(&table, 20));
    assert_eq!(
        table,
        static_table!(
            "|  | column  |  |  |"
            "|  | 0       |  |  |"
            "|--|---------|--|--|"
            "|  |   0-0   |  |  |"
            "|  | Hello W |  |  |"
            "|  | orld Wi |  |  |"
            "|  | th Big  |  |  |"
            "|  | Line    |  |  |"
            "|  |   2-0   |  |  |"
        )
    );

    let table = Matrix::new(3, 3)
        .insert((2, 1), "Hello World With Big Line")
        .with(Style::markdown())
        .with(Width::wrap(0).priority::<PriorityMin>())
        .to_string();

    assert!(is_lines_equal(&table, 13));
    assert_eq!(
        table,
        static_table!(
            "|  |  |  |  |"
            "|--|--|--|--|"
            "|  |  |  |  |"
            "|  |  |  |  |"
            "|  |  |  |  |"
        )
    );
}

#[test]
fn max_width_wrap_priority_min_with_span() {
    let table = Matrix::new(3, 3)
        .insert((2, 1), "Hello World With Big Line")
        .with(Style::markdown())
        .with(Modify::new((2, 1)).with(Span::column(2)))
        .with(Width::wrap(15).priority::<PriorityMin>())
        .to_string();

    assert!(is_lines_equal(&table, 15));
    assert_eq!(
        table,
        static_table!(
            "|  |  | co |  |"
            "|  |  | lu |  |"
            "|  |  | mn |  |"
            "|  |  |  1 |  |"
            "|--|--|----|--|"
            "|  |  | 0- |  |"
            "|  |  | 1  |  |"
            "|  | Hello |  |"
            "|  |  Worl |  |"
            "|  | d Wit |  |"
            "|  | h Big |  |"
            "|  |  Line |  |"
            "|  |  | 2- |  |"
            "|  |  | 1  |  |"
        )
    );
}

#[test]
fn min_width_priority_max() {
    let table = Matrix::new(3, 3)
        .with(Style::markdown())
        .with(MinWidth::new(60).priority::<PriorityMax>())
        .to_string();

    assert_eq!(string_width_multiline(&table), 60);
    assert_eq!(
        table,
        static_table!(
            "| N | column 0 | column 1 |            column 2            |"
            "|---|----------|----------|--------------------------------|"
            "| 0 |   0-0    |   0-1    |              0-2               |"
            "| 1 |   1-0    |   1-1    |              1-2               |"
            "| 2 |   2-0    |   2-1    |              2-2               |"
        ),
    );
}

#[test]
fn min_width_priority_min() {
    let table = Matrix::new(3, 3)
        .with(Style::markdown())
        .with(MinWidth::new(60).priority::<PriorityMin>())
        .to_string();

    assert_eq!(string_width_multiline(&table), 60);
    assert_eq!(
        table,
        static_table!(
            "|      N       |   column 0   |   column 1   |  column 2   |"
            "|--------------|--------------|--------------|-------------|"
            "|      0       |     0-0      |     0-1      |     0-2     |"
            "|      1       |     1-0      |     1-1      |     1-2     |"
            "|      2       |     2-0      |     2-1      |     2-2     |"
        ),
    );
}

#[test]
fn max_width_tab_0() {
    let table =
        Matrix::iter(["\t\tTigre Ecuador\tOMYA Andina\t3824909999\tCalcium carbonate\tColombia\t"])
            .with(TabSize::new(4))
            .with(Style::markdown())
            .with(Width::wrap(60))
            .to_string();

    assert!(is_lines_equal(&table, 60));
    assert_eq!(
        table,
        static_table!(
            "|                           &str                           |"
            "|----------------------------------------------------------|"
            "|         Tigre Ecuador    OMYA Andina    3824909999    Ca |"
            "| lcium carbonate    Colombia                              |"
        )
    );
}

#[test]
fn min_width_is_not_used_after_padding() {
    let table = Matrix::new(3, 3)
        .with(Style::markdown())
        .with(MinWidth::new(60))
        .with(Modify::new((0, 0)).with(Padding::new(2, 2, 0, 0)))
        .to_string();

    assert_eq!(string_width_multiline(&table), 40);
    assert_eq!(
        table,
        static_table!(
            "|  N  | column 0 | column 1 | column 2 |"
            "|-----|----------|----------|----------|"
            "|  0  |   0-0    |   0-1    |   0-2    |"
            "|  1  |   1-0    |   1-1    |   1-2    |"
            "|  2  |   2-0    |   2-1    |   2-2    |"
        ),
    );
}

#[test]
fn min_width_is_used_after_margin() {
    let table = Matrix::new(3, 3)
        .with(Style::markdown())
        .with(Margin::new(1, 1, 1, 1))
        .with(Width::increase(60))
        .to_string();

    assert_eq!(string_width_multiline(&table), 60);
    assert_eq!(
        table,
        static_table!(
            "                                                            "
            " |   N    |   column 0    |   column 1    |   column 2    | "
            " |--------|---------------|---------------|---------------| "
            " |   0    |      0-0      |      0-1      |      0-2      | "
            " |   1    |      1-0      |      1-1      |      1-2      | "
            " |   2    |      2-0      |      2-1      |      2-2      | "
            "                                                            "
        ),
    );
}

#[test]
fn wrap_keeping_words_0() {
    let data = vec![["Hello world"]];
    let table = tabled::Table::new(data)
        .with(Width::wrap(8).keep_words())
        .to_string();

    assert_eq!(
        tabled::grid::util::string::string_width_multiline(&table),
        8
    );

    assert_eq!(
        table,
        static_table!(
            "+------+"
            "| 0    |"
            "+------+"
            "| Hell |"
            "| o wo |"
            "| rld  |"
            "+------+"
        )
    );
}

#[test]
fn cell_truncate_multiline() {
    let table = Matrix::new(3, 3)
        .insert((1, 1), "H\nel\nlo World")
        .insert((3, 2), "multi\nline string\n")
        .with(Style::markdown())
        .with(
            Modify::new(Columns::new(1..2).not(Rows::single(0)))
                .with(Width::truncate(1).multiline()),
        )
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "| N | column 0 |  column 1   | column 2 |"
            "|---|----------|-------------|----------|"
            "| 0 |    H     |     0-1     |   0-2    |"
            "|   |    e     |             |          |"
            "|   |    l     |             |          |"
            "| 1 |    1     |     1-1     |   1-2    |"
            "| 2 |    2     | multi       |   2-2    |"
            "|   |          | line string |          |"
            "|   |          |             |          |"
        )
    );
}

#[test]
fn cell_truncate_multiline_with_suffix() {
    let table = Matrix::new(3, 3)
        .insert((1, 1), "H\nel\nlo World")
        .insert((3, 2), "multi\nline string\n")
        .with(Style::markdown())
        .with(
            Modify::new(Columns::new(1..2).not(Rows::single(0)))
                .with(Width::truncate(1).multiline().suffix(".")),
        )
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "| N | column 0 |  column 1   | column 2 |"
            "|---|----------|-------------|----------|"
            "| 0 |    .     |     0-1     |   0-2    |"
            "|   |    .     |             |          |"
            "|   |    .     |             |          |"
            "| 1 |    .     |     1-1     |   1-2    |"
            "| 2 |    .     | multi       |   2-2    |"
            "|   |          | line string |          |"
            "|   |          |             |          |"
        )
    );
}

#[test]
fn table_truncate_multiline() {
    let table = Matrix::new(3, 3)
        .insert((1, 1), "H\nel\nlo World")
        .insert((3, 2), "multi\nline string\n")
        .with(Style::markdown())
        .with(Width::truncate(20).multiline())
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "|  | c | colu | co |"
            "|--|---|------|----|"
            "|  | H | 0-1  | 0- |"
            "|  | e |      |    |"
            "|  | l |      |    |"
            "|  | 1 | 1-1  | 1- |"
            "|  | 2 | mult | 2- |"
            "|  |   | line |    |"
            "|  |   |      |    |"
        )
    );
}

#[test]
fn table_truncate_multiline_with_suffix() {
    let table = Matrix::new(3, 3)
        .insert((1, 1), "H\nel\nlo World")
        .insert((3, 2), "multi\nline string\n")
        .with(Style::markdown())
        .with(Width::truncate(20).suffix(".").multiline())
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "|  | . | col. | c. |"
            "|--|---|------|----|"
            "|  | . | 0-1  | 0. |"
            "|  | . |      |    |"
            "|  | . |      |    |"
            "|  | . | 1-1  | 1. |"
            "|  | . | mul. | 2. |"
            "|  |   | lin. |    |"
            "|  |   | .    |    |"
        )
    );
}

#[cfg(feature = "derive")]
mod derived {
    use super::*;

    use tabled::Tabled;

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

        let table = Matrix::iter(&data)
            .with(Style::markdown())
            .with(Modify::new(Segment::all()).with(Alignment::left()))
            .with(Width::wrap(57))
            .to_string();

        assert_eq!(
            table,
            static_table!(
                "| ver | published_d | is_act | major_feature            |"
                "| sio | ate         | ive    |                          |"
                "| n   |             |        |                          |"
                "|-----|-------------|--------|--------------------------|"
                "| 0.2 | 2021-06-23  | true   | #[header(inline)] attrib |"
                "| .1  |             |        | ute                      |"
                "| 0.2 | 2021-06-19  | false  | API changes              |"
                "| .0  |             |        |                          |"
                "| 0.1 | 2021-06-07  | false  | display_with attribute   |"
                "| .4  |             |        |                          |"
            )
        );
        assert!(is_lines_equal(&table, 57));

        let table = Matrix::iter(&data)
            .with(Style::markdown())
            .with(Modify::new(Segment::all()).with(Alignment::left()))
            .with(Width::wrap(57).keep_words())
            .to_string();

        assert_eq!(
            table,
            static_table!(
                "| ver | published_d | is_act | major_feature            |"
                "| sio | ate         | ive    |                          |"
                "| n   |             |        |                          |"
                "|-----|-------------|--------|--------------------------|"
                "| 0.2 | 2021-06-23  | true   | #[header(inline)]        |"
                "| .1  |             |        | attribute                |"
                "| 0.2 | 2021-06-19  | false  | API changes              |"
                "| .0  |             |        |                          |"
                "| 0.1 | 2021-06-07  | false  | display_with attribute   |"
                "| .4  |             |        |                          |"
            )
        );
        assert!(is_lines_equal(&table, 57));
    }

    #[cfg(feature = "color")]
    #[test]
    fn wrapping_as_total_multiline_color() {
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

        let table = Matrix::iter(&data)
            .with(Style::markdown())
            .with(Modify::new(Segment::all()).with(Alignment::left()))
            .with(Width::wrap(57))
            .to_string();

        assert_eq!(
            table,
            static_table!(
                "| ver | published_d | is_act | major_feature            |"
                "| sio | ate         | ive    |                          |"
                "| n   |             |        |                          |"
                "|-----|-------------|--------|--------------------------|"
                "| \u{1b}[31m0.2\u{1b}[39m | \u{1b}[48;2;8;10;30m\u{1b}[31m2021-06-23\u{1b}[39m\u{1b}[49m  | true   | \u{1b}[34m\u{1b}[42m#[header(inline)] attrib\u{1b}[39m\u{1b}[49m |"
                "| \u{1b}[31m.1\u{1b}[39m  |             |        | \u{1b}[34m\u{1b}[42mute\u{1b}[39m\u{1b}[49m                      |"
                "| \u{1b}[31m0.2\u{1b}[39m | \u{1b}[48;2;8;100;30m\u{1b}[32m2021-06-19\u{1b}[39m\u{1b}[49m  | false  | \u{1b}[33mAPI changes\u{1b}[39m              |"
                "| \u{1b}[31m.0\u{1b}[39m  |             |        |                          |"
                "| \u{1b}[37m0.1\u{1b}[39m | \u{1b}[48;2;8;10;30m\u{1b}[31m2021-06-07\u{1b}[39m\u{1b}[49m  | false  | \u{1b}[31;40mdisplay_with attribute\u{1b}[0m   |"
                "| \u{1b}[37m.4\u{1b}[39m  |             |        |                          |"
            )
        );
        assert_eq!(string_width_multiline(&table), 57);

        let table = Matrix::iter(&data)
            .with(Style::markdown())
            .with(Modify::new(Segment::all()).with(Alignment::left()))
            .with(Width::wrap(57).keep_words())
            .to_string();

        assert_eq!(
            table,
            static_table!(
                "| ver | published_d | is_act | major_feature            |"
                "| sio | ate         | ive    |                          |"
                "| n   |             |        |                          |"
                "|-----|-------------|--------|--------------------------|"
                "| \u{1b}[31m0.2\u{1b}[39m | \u{1b}[48;2;8;10;30m\u{1b}[31m2021-06-23\u{1b}[39m\u{1b}[49m  | true   | \u{1b}[34m\u{1b}[42m#[header(inline)] \u{1b}[39m\u{1b}[49m       |"
                "| \u{1b}[31m.1\u{1b}[39m  |             |        | \u{1b}[34m\u{1b}[42mattribute\u{1b}[39m\u{1b}[49m                |"
                "| \u{1b}[31m0.2\u{1b}[39m | \u{1b}[48;2;8;100;30m\u{1b}[32m2021-06-19\u{1b}[39m\u{1b}[49m  | false  | \u{1b}[33mAPI changes\u{1b}[39m              |"
                "| \u{1b}[31m.0\u{1b}[39m  |             |        |                          |"
                "| \u{1b}[37m0.1\u{1b}[39m | \u{1b}[48;2;8;10;30m\u{1b}[31m2021-06-07\u{1b}[39m\u{1b}[49m  | false  | \u{1b}[31;40mdisplay_with attribute\u{1b}[0m   |"
                "| \u{1b}[37m.4\u{1b}[39m  |             |        |                          |"
            )
        );
        assert_eq!(string_width_multiline(&table), 57);
    }

    #[cfg(feature = "color")]
    #[test]
    fn truncating_as_total_multiline_color() {
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

        let table = Matrix::iter(data)
            .with(Style::markdown())
            .with(Modify::new(Segment::all()).with(Alignment::left()))
            .with(Width::truncate(57))
            .to_string();

        assert_eq!(
            ansi_str::AnsiStr::ansi_strip(&table),
            static_table!(
                "| ver | published_d | is_act | major_feature            |"
                "|-----|-------------|--------|--------------------------|"
                "| 0.2 | 2021-06-23  | true   | #[header(inline)] attrib |"
                "| 0.2 | 2021-06-19  | false  | API changes              |"
                "| 0.1 | 2021-06-07  | false  | display_with attribute   |"
            )
        );

        assert_eq!(
            table,
            "| ver | published_d | is_act | major_feature            |\n|-----|-------------|--------|--------------------------|\n| \u{1b}[31m0.2\u{1b}[39m | \u{1b}[48;2;8;10;30m\u{1b}[31m2021-06-23\u{1b}[39m\u{1b}[49m  | true   | \u{1b}[34;42m#[header(inline)] attrib\u{1b}[39m\u{1b}[49m |\n| \u{1b}[31m0.2\u{1b}[39m | \u{1b}[48;2;8;100;30m\u{1b}[32m2021-06-19\u{1b}[39m\u{1b}[49m  | false  | \u{1b}[33mAPI changes\u{1b}[39m              |\n| \u{1b}[37m0.1\u{1b}[39m | \u{1b}[48;2;8;10;30m\u{1b}[31m2021-06-07\u{1b}[39m\u{1b}[49m  | false  | \u{1b}[31;40mdisplay_with attribute\u{1b}[0m   |"
        );
        assert_eq!(string_width_multiline(&table), 57);
    }

    #[cfg(feature = "color")]
    fn format_osc8_hyperlink(url: &str, text: &str) -> String {
        format!(
            "{osc}8;;{url}{st}{text}{osc}8;;{st}",
            url = url,
            text = text,
            osc = "\x1b]",
            st = "\x1b\\"
        )
    }

    #[cfg(feature = "color")]
    #[test]
    fn hyperlinks() {
        #[derive(Tabled)]
        struct Distribution {
            name: String,
            is_hyperlink: bool,
        }

        let table = |text: &str| {
            let data = [Distribution {
                name: text.to_owned(),
                is_hyperlink: true,
            }];
            tabled::Table::new(data)
                .with(
                    Modify::new(Segment::all())
                        .with(Width::wrap(5).keep_words())
                        .with(Alignment::left()),
                )
                .to_string()
        };

        let text = format_osc8_hyperlink("https://www.debian.org/", "Debian");
        assert_eq!(
            table(&text),
            "+-------+-------+\n\
             | name  | is_hy |\n\
             |       | perli |\n\
             |       | nk    |\n\
             +-------+-------+\n\
             | \u{1b}]8;;https://www.debian.org/\u{1b}\\Debia\u{1b}]8;;\u{1b}\\ | true  |\n\
             | \u{1b}]8;;https://www.debian.org/\u{1b}\\n\u{1b}]8;;\u{1b}\\     |       |\n\
             +-------+-------+"
        );

        // if there's more text than a link it will be ignored
        let text = format!(
            "{} :link",
            format_osc8_hyperlink("https://www.debian.org/", "Debian"),
        );
        assert_eq!(
            table(&text),
            "+-------+-------+\n\
             | name  | is_hy |\n\
             |       | perli |\n\
             |       | nk    |\n\
             +-------+-------+\n\
             | Debia | true  |\n\
             | n     |       |\n\
             | :link |       |\n\
             +-------+-------+"
        );

        let text = format!(
            "asd {} 2 links in a string {}",
            format_osc8_hyperlink("https://www.debian.org/", "Debian"),
            format_osc8_hyperlink("https://www.wikipedia.org/", "Debian"),
        );
        assert_eq!(
            table(&text),
            static_table!(
                "+-------+-------+"
                "| name  | is_hy |"
                "|       | perli |"
                "|       | nk    |"
                "+-------+-------+"
                "| asd D | true  |"
                "| ebian |       |"
                "|  2    |       |"
                "| links |       |"
                "|  in a |       |"
                "|  stri |       |"
                "| ng De |       |"
                "| bian  |       |"
                "+-------+-------+"
            )
        );
    }

    #[cfg(feature = "color")]
    #[test]
    fn hyperlinks_with_color() {
        use owo_colors::OwoColorize;

        #[derive(Tabled)]
        struct Distribution {
            name: String,
            is_hyperlink: bool,
        }

        let table = |text: &str| {
            let data = [Distribution {
                name: text.to_owned(),
                is_hyperlink: true,
            }];
            tabled::Table::new(data)
                .with(
                    Modify::new(Segment::all())
                        .with(Width::wrap(6).keep_words())
                        .with(Alignment::left()),
                )
                .to_string()
        };

        let text = format_osc8_hyperlink(
            "https://www.debian.org/",
            "Debian".red().to_string().as_str(),
        );
        assert_eq!(
            table(&text),
            static_table!(
                "+--------+--------+"
                "| name   | is_hyp |"
                "|        | erlink |"
                "+--------+--------+"
                "| \u{1b}]8;;https://www.debian.org/\u{1b}\\\u{1b}[31mDebian\u{1b}[39m\u{1b}]8;;\u{1b}\\ | true   |"
                "+--------+--------+"
            )
        );

        // if there's more text than a link it will be ignored
        let text = format!(
            "{} :link",
            format_osc8_hyperlink("https://www.debian.org/", "Debian"),
        );
        assert_eq!(
            table(&text),
            static_table!(
                "+--------+--------+"
                "| name   | is_hyp |"
                "|        | erlink |"
                "+--------+--------+"
                "| Debian | true   |"
                "|  :link |        |"
                "+--------+--------+"
            )
        );

        let text = format!(
            "asd {} 2 links in a string {}",
            format_osc8_hyperlink("https://www.debian.org/", "Debian"),
            format_osc8_hyperlink("https://www.wikipedia.org/", "Debian"),
        );
        assert_eq!(
            table(&text),
            static_table!(
                "+--------+--------+"
                "| name   | is_hyp |"
                "|        | erlink |"
                "+--------+--------+"
                "| asd    | true   |"
                "| Debian |        |"
                "|  2     |        |"
                "| links  |        |"
                "| in a   |        |"
                "| string |        |"
                "|        |        |"
                "| Debian |        |"
                "+--------+--------+"
            )
        );
    }
}
