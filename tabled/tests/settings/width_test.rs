#![cfg(feature = "std")]

use tabled::{
    settings::{
        formatting::{TabSize, TrimStrategy},
        object::{Columns, Object, Rows, Segment},
        peaker::{PriorityLeft, PriorityMax, PriorityMin, PriorityRight},
        width::{Justify, MinWidth, SuffixLimit, Width},
        Alignment, Margin, Modify, Padding, Panel, Settings, Span, Style,
    },
    Table,
};

use crate::matrix::Matrix;
use testing_table::{assert_width, test_table};

#[cfg(feature = "ansi")]
use ::{
    ansi_str::AnsiStr,
    tabled::settings::{style::HorizontalLine, Color},
};

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
    max_width_doesnt_increase_width_if_it_is_smaller,
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
    Matrix::iter(vec!["this is a long sentence"])
        .with(Style::markdown())
        .with(Modify::new(Segment::all()).with(Alignment::left()))
        .with(Modify::new(Segment::all()).with(Width::wrap(17).keep_words(true)))
        .to_string(),
    "| &str            |"
    "|-----------------|"
    "| this is a long  |"
    "| sentence        |"
);

test_table!(
    max_width_wrapped_keep_words_1,
    Matrix::iter(vec!["this is a long  sentence"])
        .with(Style::markdown())
        .with(Modify::new(Segment::all()).with(Alignment::left()))
        .with(Modify::new(Segment::all()).with(Width::wrap(17).keep_words(true)))
        .to_string(),
    "| &str             |"
    "|------------------|"
    "| this is a long   |"
    "| sentence         |"
);

test_table!(
    max_width_wrapped_keep_words_2,
    {
        let table = Matrix::iter(vec!["this is a long   sentence"])
            .with(Style::markdown())
            .with(Modify::new(Segment::all()).with(Alignment::left()))
            .with(Modify::new(Segment::all()).with(Width::wrap(17).keep_words(true)))
            .to_string();

        assert_width!(&table, 17 + 2 + 2);

        table
    },
    "| &str              |"
    "|-------------------|"
    "| this is a long    |"
    "| sentence          |"
);

#[cfg(feature = "ansi")]
test_table!(
    max_width_wrapped_keep_words_3,
    {
        let table = Matrix::iter(vec!["this is a long    sentence"])
            .with(Style::markdown())
            .with(Modify::new(Segment::all()).with(Alignment::left()))
            .with(Modify::new(Segment::all()).with(Width::wrap(17).keep_words(true)))
            .to_string();

        assert_width!(&table, 17 + 2 + 2);

        table
    },
    // 'sentence' doesn't have a space ' sentence' because we use left alignment
    "| &str              |"
    "|-------------------|"
    "| this is a long    |"
    "|  sentence         |"
);

#[cfg(not(feature = "ansi"))]
test_table!(
    max_width_wrapped_keep_words_3,
    {
        let table = Matrix::iter(vec!["this is a long    sentence"])
            .with(Style::markdown())
            .with(Modify::new(Segment::all()).with(Alignment::left()))
            .with(Modify::new(Segment::all()).with(Width::wrap(17).keep_words(true)))
            .to_string();

        assert_width!(&table, 17 + 2 + 2);

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
            .with(Modify::new(Segment::all()).with(Width::wrap(10).keep_words(true)))
            .to_string();

        assert_width!(&table, 8);

        table
    },
    "| &str |"
    "|------|"
    "| this |"
);

#[cfg(feature = "ansi")]
test_table!(
    max_width_wrapped_keep_words_color_0,
    {
        let table = Matrix::iter(vec![(Color::BG_BLACK | Color::FG_GREEN).colorize("this is a long sentence")])
            .with(Style::markdown())
            .with(Modify::new(Segment::all()).with(Alignment::left()))
            .with(Modify::new(Segment::all()).with(Width::wrap(17).keep_words(true)))
            .to_string();

        AnsiStr::ansi_strip(&table).to_string()
    },
    "| String          |"
    "|-----------------|"
    "| this is a long  |"
    "| sentence        |"
);

#[cfg(feature = "ansi")]
test_table!(
    max_width_wrapped_keep_words_color_0_1,
    Matrix::iter(vec![(Color::BG_BLACK | Color::FG_GREEN).colorize("this is a long sentence")])
        .with(Style::markdown())
        .with(Modify::new(Segment::all()).with(Alignment::left()))
        .with(Modify::new(Segment::all()).with(Width::wrap(17).keep_words(true))),
        "| String          |"
        "|-----------------|"
        "| \u{1b}[32m\u{1b}[40mthis is a long \u{1b}[39m\u{1b}[49m |"
        "| \u{1b}[32m\u{1b}[40msentence\u{1b}[39m\u{1b}[49m        |"
);

#[cfg(feature = "ansi")]
test_table!(
    max_width_wrapped_keep_words_color_1,
    {
        let table = Matrix::iter(vec![(Color::BG_BLACK | Color::FG_GREEN).colorize("this is a long  sentence")])
            .with(Style::markdown())
            .with(Modify::new(Segment::all()).with(Alignment::left()))
            .with(Modify::new(Segment::all()).with(Width::wrap(17).keep_words(true)))
            .to_string();

        AnsiStr::ansi_strip(&table).to_string()
    },
    "| String           |"
    "|------------------|"
    "| this is a long   |"
    "| sentence         |"
);

#[cfg(feature = "ansi")]
test_table!(
    max_width_wrapped_keep_words_color_1_1,
    Matrix::iter(vec![(Color::BG_BLACK | Color::FG_GREEN).colorize("this is a long  sentence")])
        .with(Style::markdown())
        .with(Modify::new(Segment::all()).with(Alignment::left()))
        .with(Modify::new(Segment::all()).with(Width::wrap(17).keep_words(true))),
    "| String           |"
    "|------------------|"
    "| \u{1b}[32m\u{1b}[40mthis is a long  \u{1b}[39m\u{1b}[49m |"
    "| \u{1b}[32m\u{1b}[40msentence\u{1b}[39m\u{1b}[49m         |"
);

#[cfg(feature = "ansi")]
test_table!(
    max_width_wrapped_keep_words_color_2,
    {
        let table = Matrix::iter(vec![(Color::BG_BLACK | Color::FG_GREEN).colorize("this is a long   sentence")])
            .with(Style::markdown())
            .with(Modify::new(Segment::all()).with(Alignment::left()))
            .with(Modify::new(Segment::all()).with(Width::wrap(17).keep_words(true)))
            .to_string();

        AnsiStr::ansi_strip(&table).to_string()
    },
    "| String            |"
    "|-------------------|"
    "| this is a long    |"
    "| sentence          |"
);

#[cfg(feature = "ansi")]
test_table!(
    max_width_wrapped_keep_words_color_2_1,
    Matrix::iter(vec![(Color::BG_BLACK | Color::FG_GREEN).colorize("this is a long   sentence")])
        .with(Style::markdown())
        .with(Modify::new(Segment::all()).with(Alignment::left()))
        .with(Modify::new(Segment::all()).with(Width::wrap(17).keep_words(true))),
    "| String            |"
    "|-------------------|"
    "| \u{1b}[32m\u{1b}[40mthis is a long   \u{1b}[39m\u{1b}[49m |"
    "| \u{1b}[32m\u{1b}[40msentence\u{1b}[39m\u{1b}[49m          |"
);

#[cfg(feature = "ansi")]
test_table!(
    max_width_wrapped_keep_words_color_3,
    {
        let table = Matrix::iter(vec![(Color::BG_BLACK | Color::FG_GREEN).colorize("this is a long    sentence")])
            .with(Style::markdown())
            .with(Modify::new(Segment::all()).with(Alignment::left()))
            .with(Modify::new(Segment::all()).with(Width::wrap(17).keep_words(true)))
            .to_string();

        AnsiStr::ansi_strip(&table).to_string()
    },
    "| String            |"
    "|-------------------|"
    "| this is a long    |"
    "|  sentence         |"
);

#[cfg(feature = "ansi")]
test_table!(
    max_width_wrapped_keep_words_color_3_1,
    Matrix::iter(vec![(Color::BG_BLACK | Color::FG_GREEN).colorize("this is a long    sentence")])
        .with(Style::markdown())
        .with(Modify::new(Segment::all()).with(Alignment::left()))
        .with(Modify::new(Segment::all()).with(Width::wrap(17).keep_words(true))),
    "| String            |"
    "|-------------------|"
    "| \u{1b}[32m\u{1b}[40mthis is a long   \u{1b}[39m\u{1b}[49m |"
    "| \u{1b}[32m\u{1b}[40m sentence\u{1b}[39m\u{1b}[49m         |"
);

#[cfg(feature = "ansi")]
test_table!(
    max_width_wrapped_keep_words_color_4,
    {
        let table = Matrix::iter(vec![(Color::BG_BLACK | Color::FG_GREEN).colorize("this")])
            .with(Style::markdown())
            .with(Modify::new(Segment::all()).with(Width::wrap(10).keep_words(true)))
            .to_string();

        AnsiStr::ansi_strip(&table).to_string()
    },
    "| String |"
    "|--------|"
    "|  this  |"
);

#[cfg(feature = "ansi")]
test_table!(
    max_width_wrapped_keep_words_color_4_1,
    Matrix::iter(vec![(Color::BG_BLACK | Color::FG_GREEN).colorize("this")])
        .with(Style::markdown())
        .with(Modify::new(Segment::all()).with(Width::wrap(10).keep_words(true))),
    "| String |"
    "|--------|"
    "|  \u{1b}[40m\u{1b}[32mthis\u{1b}[49m\u{1b}[39m  |"
);

test_table!(
    max_width_wrapped_keep_words_long_word,
    Matrix::iter(["this is a long sentencesentencesentence"])
        .with(Style::markdown())
        .with(Modify::new(Segment::all()).with(Alignment::left()))
        .with(Modify::new(Segment::all()).with(Width::wrap(17).keep_words(true))),
    "| &str              |"
    "|-------------------|"
    "| this is a long se |"
    "| ntencesentencesen |"
    "| tence             |"
);

#[cfg(feature = "ansi")]
test_table!(
    max_width_wrapped_keep_words_long_word_color,
    {
        let color = Color::BG_BLACK | Color::FG_GREEN;
        let data = vec![color.colorize("this is a long sentencesentencesentence")];
        Table::new(data)
            .with(Style::markdown())
            .with(Alignment::left())
            .with(Width::wrap(21).keep_words(true))
            .to_string()
    },
    "| String            |"
    "|-------------------|"
    "| \u{1b}[32m\u{1b}[40mthis is a long se\u{1b}[39m\u{1b}[49m |"
    "| \u{1b}[32m\u{1b}[40mntencesentencesen\u{1b}[39m\u{1b}[49m |"
    "| \u{1b}[32m\u{1b}[40mtence\u{1b}[39m\u{1b}[49m             |"
);

#[cfg(feature = "ansi")]
test_table!(
    max_width_keep_words_1,
    Matrix::iter(["asdf"])
        .with(Width::wrap(7).keep_words(true)),
    "+-----+"
    "| &st |"
    "| r   |"
    "+-----+"
    "| asd |"
    "| f   |"
    "+-----+"
);

#[cfg(feature = "ansi")]
test_table!(
    max_width_keep_words_2,
    Matrix::iter(["qweqw eqwe"])
        .with(Width::wrap(8).keep_words(true)),
    "+------+"
    "| &str |"
    "+------+"
    "| qweq |"
    "| w    |"
    "| eqwe |"
    "+------+"
);

#[cfg(feature = "ansi")]
test_table!(
    max_width_keep_words_3,
    {
        let mut m = Matrix::iter([
            ["123 45678", "qweqw eqwe", "..."],
            ["0", "1", "..."],
            ["0", "1", "..."],
        ]);

        m.with(Alignment::center())
            .with(
                Style::modern()
                    .remove_horizontal()
                    .horizontals([(1, HorizontalLine::inherit(Style::modern()))]),
            )
            .with(
                Width::wrap(21)
                    .keep_words(true)
                    .priority(PriorityMax::right()),
            )
            .to_string()
    },
    "┌──────┬──────┬─────┐"
    "│  0   │  1   │  2  │"
    "├──────┼──────┼─────┤"
    "│ 123  │ qweq │ ... │"
    "│ 4567 │ w    │     │"
    "│ 8    │ eqwe │     │"
    "│  0   │  1   │ ... │"
    "│  0   │  1   │ ... │"
    "└──────┴──────┴─────┘"
);

#[cfg(feature = "ansi")]
test_table!(
    max_width_wrapped_collored,
    {
        let data = &[
            Color::FG_RED.colorize("asd"),
            Color::FG_BLUE.colorize("zxc2"),
            (Color::FG_GREEN | Color::BG_BLACK).colorize("asdasd"),
        ];

        Matrix::iter(data)
            .with(Style::markdown())
            .modify(Segment::all(), Width::wrap(2))
            .to_string()
    },
    "| St |"
    "| ri |"
    "| ng |"
    "|----|"
    "| \u{1b}[31mas\u{1b}[39m |"
    "| \u{1b}[31md\u{1b}[39m  |"
    "| \u{1b}[34mzx\u{1b}[39m |"
    "| \u{1b}[34mc2\u{1b}[39m |"
    "| \u{1b}[32m\u{1b}[40mas\u{1b}[39m\u{1b}[49m |"
    "| \u{1b}[32m\u{1b}[40mda\u{1b}[39m\u{1b}[49m |"
    "| \u{1b}[32m\u{1b}[40msd\u{1b}[39m\u{1b}[49m |"
);

test_table!(
    dont_change_content_if_width_is_less_then_max_width,
    Matrix::new(3, 3)
        .with(Style::markdown())
        .with(Modify::new(Segment::all()).with(Width::truncate(1000).suffix("...")))
        .to_string(),
    "| N | column 0 | column 1 | column 2 |"
    "|---|----------|----------|----------|"
    "| 0 |   0-0    |   0-1    |   0-2    |"
    "| 1 |   1-0    |   1-1    |   1-2    |"
    "| 2 |   2-0    |   2-1    |   2-2    |"
);

test_table!(
    max_width_with_emoji,
    Matrix::iter(["🤠", "😳🥵🥶😱😨", "🚴🏻‍♀️🚴🏻🚴🏻‍♂️🚵🏻‍♀️🚵🏻🚵🏻‍♂️"])
        .with(Style::markdown())
        .modify(Segment::all(), Width::truncate(6).suffix("..."))
        .to_string(),
    "|  &str  |"
    "|--------|"
    "|   🤠   |"
    "| 😳�... |"
    "| 🚴�... |"
);

#[cfg(feature = "ansi")]
test_table!(
    color_chars_are_stripped,
    Matrix::iter(&[
        Color::FG_RED.colorize("asd"),
        Color::FG_BLUE.colorize("zxc"),
        (Color::FG_GREEN | Color::BG_BLACK).colorize("asdasd"),
    ])
    .with(Style::markdown())
    .modify(Segment::all(), Width::truncate(4).suffix("..."))
    .to_string(),
    "| S... |"
    "|------|"
    "| \u{1b}[31masd\u{1b}[39m  |"
    "| \u{1b}[34mzxc\u{1b}[39m  |"
    "| \u{1b}[32m\u{1b}[40ma\u{1b}[39m\u{1b}[49m... |"
);

test_table!(
    min_width_0,
    Matrix::table(3, 3)
        .with(Style::markdown())
        .modify(Rows::single(0), MinWidth::new(12)),
    "| N            | column 0     | column 1     | column 2     |"
    "|--------------|--------------|--------------|--------------|"
    "|      0       |     0-0      |     0-1      |     0-2      |"
    "|      1       |     1-0      |     1-1      |     1-2      |"
    "|      2       |     2-0      |     2-1      |     2-2      |"
);

test_table!(
    min_width_1,
    Matrix::table(3, 3)
        .with(Style::markdown())
        .modify(Rows::single(0), MinWidth::new(12))
        .modify(Segment::all(), TrimStrategy::None),
    "| N            | column 0     | column 1     | column 2     |"
    "|--------------|--------------|--------------|--------------|"
    "|      0       |     0-0      |     0-1      |     0-2      |"
    "|      1       |     1-0      |     1-1      |     1-2      |"
    "|      2       |     2-0      |     2-1      |     2-2      |"
);

test_table!(
    min_width_with_filler,
    Matrix::new(3, 3)
        .with(Style::markdown())
        .modify(Rows::single(0), MinWidth::new(12).fill_with('.'))
        .to_string(),
    "| N........... | column 0.... | column 1.... | column 2.... |"
    "|--------------|--------------|--------------|--------------|"
    "|      0       |     0-0      |     0-1      |     0-2      |"
    "|      1       |     1-0      |     1-1      |     1-2      |"
    "|      2       |     2-0      |     2-1      |     2-2      |"
);

test_table!(
    min_width_one_column_0,
    Matrix::table(3, 3)
        .with(Style::markdown())
        .modify((0, 0), MinWidth::new(5)),
    "| N     | column 0 | column 1 | column 2 |"
    "|-------|----------|----------|----------|"
    "|   0   |   0-0    |   0-1    |   0-2    |"
    "|   1   |   1-0    |   1-1    |   1-2    |"
    "|   2   |   2-0    |   2-1    |   2-2    |"
);

test_table!(
    min_width_one_column_1,
    Matrix::table(3, 3)
        .with(Style::markdown())
        .modify((0, 0), MinWidth::new(5))
        .modify(Segment::all(), TrimStrategy::None),
    "| N     | column 0 | column 1 | column 2 |"
    "|-------|----------|----------|----------|"
    "|   0   |   0-0    |   0-1    |   0-2    |"
    "|   1   |   1-0    |   1-1    |   1-2    |"
    "|   2   |   2-0    |   2-1    |   2-2    |"
);

test_table!(
    min_width_on_smaller_content,
    Matrix::new(3, 3)
        .with(Style::markdown())
        .modify(Rows::single(0), MinWidth::new(1))
        .to_string(),
    "| N | column 0 | column 1 | column 2 |"
    "|---|----------|----------|----------|"
    "| 0 |   0-0    |   0-1    |   0-2    |"
    "| 1 |   1-0    |   1-1    |   1-2    |"
    "| 2 |   2-0    |   2-1    |   2-2    |"
);

test_table!(
    min_with_max_width_0,
    Matrix::table(3, 3)
        .with(Style::markdown())
        .with(Modify::new(Rows::single(0)).with(MinWidth::new(3)))
        .with(Modify::new(Rows::single(0)).with(Width::truncate(3))),
    "| N   | col | col | col |"
    "|-----|-----|-----|-----|"
    "|  0  | 0-0 | 0-1 | 0-2 |"
    "|  1  | 1-0 | 1-1 | 1-2 |"
    "|  2  | 2-0 | 2-1 | 2-2 |"
);

test_table!(
    min_with_max_width_1,
    Matrix::table(3, 3)
        .with(Style::markdown())
        .with(Modify::new(Rows::single(0)).with(MinWidth::new(3)))
        .with(Modify::new(Rows::single(0)).with(Width::truncate(3)))
        .with(Modify::new(Segment::all()).with(TrimStrategy::None)),
    "| N   | col | col | col |"
    "|-----|-----|-----|-----|"
    "|  0  | 0-0 | 0-1 | 0-2 |"
    "|  1  | 1-0 | 1-1 | 1-2 |"
    "|  2  | 2-0 | 2-1 | 2-2 |"
);

test_table!(
    min_with_max_width_truncate_suffix_0,
    Matrix::table(3, 3)
        .with(Style::markdown())
        .with(Modify::new(Rows::single(0)).with(MinWidth::new(3)))
        .with(Modify::new(Rows::single(0)).with(Width::truncate(3).suffix("..."))),
    "| N   | ... | ... | ... |"
    "|-----|-----|-----|-----|"
    "|  0  | 0-0 | 0-1 | 0-2 |"
    "|  1  | 1-0 | 1-1 | 1-2 |"
    "|  2  | 2-0 | 2-1 | 2-2 |"
);

test_table!(
    min_with_max_width_truncate_suffix_1,
    Matrix::table(3, 3)
        .with(Style::markdown())
        .with(Modify::new(Rows::single(0)).with(MinWidth::new(3)))
        .with(Modify::new(Rows::single(0)).with(Width::truncate(3).suffix("...")))
        .with(Modify::new(Segment::all()).with(TrimStrategy::None)),
    "| N   | ... | ... | ... |"
    "|-----|-----|-----|-----|"
    "|  0  | 0-0 | 0-1 | 0-2 |"
    "|  1  | 1-0 | 1-1 | 1-2 |"
    "|  2  | 2-0 | 2-1 | 2-2 |"
);

test_table!(
    min_with_max_width_truncate_suffix_limit_replace,
    Matrix::new(3, 3)
        .with(Style::markdown())
        .modify(
            Rows::single(0),
            Width::truncate(3)
                .suffix("...")
                .suffix_limit(SuffixLimit::Replace('x'))
        )
        .to_string(),
    "| N | xxx | xxx | xxx |"
    "|---|-----|-----|-----|"
    "| 0 | 0-0 | 0-1 | 0-2 |"
    "| 1 | 1-0 | 1-1 | 1-2 |"
    "| 2 | 2-0 | 2-1 | 2-2 |"
);

test_table!(
    min_with_max_width_truncate_suffix_limit_cut,
    Matrix::new(3, 3)
        .with(Style::markdown())
        .modify(
            Rows::single(0),
            Width::truncate(3)
                .suffix("qwert")
                .suffix_limit(SuffixLimit::Cut)
        ),
    "| N | qwe | qwe | qwe |"
    "|---|-----|-----|-----|"
    "| 0 | 0-0 | 0-1 | 0-2 |"
    "| 1 | 1-0 | 1-1 | 1-2 |"
    "| 2 | 2-0 | 2-1 | 2-2 |"
);

test_table!(
    min_with_max_width_truncate_suffix_limit_ignore,
    Matrix::new(3, 3)
        .with(Style::markdown())
        .modify(
            Rows::single(0),
            Width::truncate(3)
                .suffix("qwert")
                .suffix_limit(SuffixLimit::Ignore),
        ),
    "| N | col | col | col |"
    "|---|-----|-----|-----|"
    "| 0 | 0-0 | 0-1 | 0-2 |"
    "| 1 | 1-0 | 1-1 | 1-2 |"
    "| 2 | 2-0 | 2-1 | 2-2 |"
);

#[cfg(feature = "ansi")]
test_table!(
    min_with_max_width_truncate_suffix_try_color,
    {
        let data = &[
            Color::FG_RED.colorize("asd"),
            Color::FG_BLUE.colorize("zxc2"),
            (Color::FG_GREEN | Color::BG_BLACK).colorize("asdasd"),
        ];

        Matrix::iter(data)
            .with(Style::markdown())
            .with(Width::truncate(7).suffix("..").suffix_try_color(true))
    },
    "| S.. |"
    "|-----|"
    "| \u{1b}[31masd\u{1b}[39m |"
    "| \u{1b}[34mz\u{1b}[39m\u{1b}[34m..\u{1b}[39m |"
    "| \u{1b}[32m\u{1b}[40ma\u{1b}[39m\u{1b}[49m\u{1b}[32m\u{1b}[40m..\u{1b}[39m\u{1b}[49m |"
);

#[cfg(feature = "ansi")]
test_table!(
    min_width_color,
    {
        let data = &[
            Color::FG_RED.colorize("asd"),
            Color::FG_BLUE.colorize("zxc"),
            (Color::FG_GREEN | Color::BG_BLACK).colorize("asdasd"),
        ];

        Matrix::iter(data)
            .with(Style::markdown())
            .with(Modify::new(Segment::all()).with(MinWidth::new(10)))
    },
    "| String     |"
    "|------------|"
    "| \u{1b}[31masd\u{1b}[39m        |"
    "| \u{1b}[34mzxc\u{1b}[39m        |"
    "| \u{1b}[32m\u{1b}[40masdasd\u{1b}[39m\u{1b}[49m     |"
);

#[cfg(feature = "ansi")]
test_table!(
    min_width_color_with_smaller_then_width,
    {
        let data = &[
            Color::FG_RED.colorize("asd"),
            Color::FG_BLUE.colorize("zxc2"),
            (Color::FG_GREEN | Color::BG_BLACK).colorize("asdasd"),
        ];

        Matrix::iter(data)
            .modify(Segment::all(), MinWidth::new(1))
    },
    "+--------+"
    "| String |"
    "+--------+"
    "|  \u{1b}[31masd\u{1b}[39m   |"
    "+--------+"
    "|  \u{1b}[34mzxc2\u{1b}[39m  |"
    "+--------+"
    "| \u{1b}[32m\u{1b}[40masdasd\u{1b}[39m\u{1b}[49m |"
    "+--------+"
);

test_table!(
    total_width_big_0,
    Matrix::new(3, 3)
        .with(Style::markdown())
        .with(Width::truncate(80))
        .with(MinWidth::new(80)),
    "|      N       |      column 0       |      column 1      |      column 2      |"
    "|--------------|---------------------|--------------------|--------------------|"
    "|      0       |         0-0         |        0-1         |        0-2         |"
    "|      1       |         1-0         |        1-1         |        1-2         |"
    "|      2       |         2-0         |        2-1         |        2-2         |"
);

test_table!(
    total_width_big_1,
    Matrix::new(3, 3)
        .with(Style::markdown())
        .with(Modify::new(Segment::all()).with(TrimStrategy::None))
        .with(Settings::new(Width::truncate(80), Width::increase(80))),
    "|      N       |      column 0       |      column 1      |      column 2      |"
    "|--------------|---------------------|--------------------|--------------------|"
    "|      0       |         0-0         |        0-1         |        0-2         |"
    "|      1       |         1-0         |        1-1         |        1-2         |"
    "|      2       |         2-0         |        2-1         |        2-2         |"
);

test_table!(
    total_width_big_with_panel,
    Matrix::new(3, 3)
        .with(Panel::horizontal(0, "Hello World"))
        .with(Alignment::center())
        .with(Padding::zero())
        .with(Style::markdown())
        .with(Width::truncate(80))
        .with(MinWidth::new(80)),
    "|                                 Hello World                                  |"
    "|--------------|---------------------|--------------------|--------------------|"
    "|      N       |      column 0       |      column 1      |      column 2      |"
    "|      0       |         0-0         |        0-1         |        0-2         |"
    "|      1       |         1-0         |        1-1         |        1-2         |"
    "|      2       |         2-0         |        2-1         |        2-2         |"
);

test_table!(
    total_width_big_with_panel_with_wrapping_doesnt_affect_increase,
    Matrix::new(3, 3)
        .with(Panel::horizontal(0, "Hello World"))
        .with(Modify::new(Segment::all()).with(Alignment::center()))
        .with(Style::markdown())
        .with(Width::wrap(80))
        .with(MinWidth::new(80)),
    "|                                 Hello World                                  |"
    "|--------------|---------------------|--------------------|--------------------|"
    "|      N       |      column 0       |      column 1      |      column 2      |"
    "|      0       |         0-0         |        0-1         |        0-2         |"
    "|      1       |         1-0         |        1-1         |        1-2         |"
    "|      2       |         2-0         |        2-1         |        2-2         |"
);

test_table!(
    total_width_small,
    Matrix::new(3, 3)
        .with(Style::markdown())
        .with(Width::truncate(14))
        .with(MinWidth::new(14)),
    "|  |  |  | c |"
    "|--|--|--|---|"
    "|  |  |  | 0 |"
    "|  |  |  | 1 |"
    "|  |  |  | 2 |"
);

test_table!(
    total_width_smaller_then_content,
    Matrix::new(3, 3)
        .with(Style::markdown())
        .with(Width::truncate(8))
        .with(MinWidth::new(8)),
    "|  |  |  |  |"
    "|--|--|--|--|"
    "|  |  |  |  |"
    "|  |  |  |  |"
    "|  |  |  |  |"
);

test_table!(
    total_width_small_with_panel_0,
    Matrix::new(3, 3)
        .with(Modify::new(Segment::all()).with(Alignment::center()))
        .with(Style::markdown())
        .with(Width::truncate(20))
        .with(MinWidth::new(20)),
    "|  | co | co | col |"
    "|--|----|----|-----|"
    "|  | 0- | 0- | 0-2 |"
    "|  | 1- | 1- | 1-2 |"
    "|  | 2- | 2- | 2-2 |"
);

test_table!(
    total_width_small_with_panel_1,
    Matrix::iter(Vec::<usize>::new())
        .with(Panel::horizontal(0, "Hello World"))
        .with(
            Modify::new(Segment::all())
                .with(Alignment::center())
                .with(Padding::zero()),
        )
        .with(Width::truncate(5))
        .with(MinWidth::new(5)),
    "+---+"
    "|Hel|"
    "+---+"
    "|usi|"
    "+---+"
);

test_table!(
    total_width_small_with_panel_2,
    Matrix::table(1, 2)
        .with(Panel::horizontal(0, "Hello World"))
        .with(Modify::new(Segment::all()).with(Alignment::center()))
        .with(Style::markdown())
        .with(Width::truncate(20))
        .with(MinWidth::new(20)),
    "|   Hello World    |"
    "|--|-------|-------|"
    "|  | colum | colum |"
    "|  |  0-0  |  0-1  |"
);

test_table!(
    total_width_small_with_panel_3,
    Matrix::new(3, 3)
        .with(Panel::horizontal(0, "Hello World"))
        .with(Modify::new(Segment::all()).with(Alignment::center()))
        .with(Style::markdown())
        .with(Width::truncate(20))
        .with(MinWidth::new(20)),
    "|   Hello World    |"
    "|--|----|----|-----|"
    "|  | co | co | col |"
    "|  | 0- | 0- | 0-2 |"
    "|  | 1- | 1- | 1-2 |"
    "|  | 2- | 2- | 2-2 |"
);

test_table!(
    total_width_small_with_panel_4,
    Matrix::new(3, 3)
        .with(Panel::horizontal(0, "Hello World"))
        .with(Modify::new(Segment::all()).with(Alignment::center()))
        .with(Style::markdown())
        .with(Width::truncate(6))
        .with(MinWidth::new(6)),
    "| Hello Wor |"
    "|--|--|--|--|"
    "|  |  |  |  |"
    "|  |  |  |  |"
    "|  |  |  |  |"
    "|  |  |  |  |"
);

test_table!(
    total_width_small_with_panel_5,
    Matrix::new(3, 3)
        .with(Panel::horizontal(0, "Hello World"))
        .with(Modify::new(Segment::all()).with(Alignment::center()))
        .with(Style::markdown())
        .with(Width::truncate(14))
        .with(MinWidth::new(14)),
    "| Hello Worl |"
    "|--|--|--|---|"
    "|  |  |  | c |"
    "|  |  |  | 0 |"
    "|  |  |  | 1 |"
    "|  |  |  | 2 |"
);

test_table!(
    total_width_small_with_panel_6,
    Matrix::new(3, 3)
        .with(Panel::horizontal(0, "Hello World 123"))
        .with(Modify::new(Segment::all()).with(Alignment::center()))
        .with(Style::markdown())
        .with(Width::truncate(14))
        .with(MinWidth::new(14)),
    "| Hello Worl |"
    "|--|--|--|---|"
    "|  |  |  | c |"
    "|  |  |  | 0 |"
    "|  |  |  | 1 |"
    "|  |  |  | 2 |"
);

#[cfg(feature = "ansi")]
test_table!(
    total_width_wrapping_0,
    Matrix::new(3, 3)
        .with(Modify::new(Segment::all()).with(Alignment::center()))
        .with(Style::markdown())
        .with(Width::wrap(20))
        .with(MinWidth::new(20)),
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
);

#[cfg(feature = "ansi")]
test_table!(
    total_width_wrapping_1,
    Matrix::new(3, 3)
        .insert((3, 2).into(), "some loong string")
        .with(Modify::new(Segment::all()).with(Alignment::center()))
        .with(Style::markdown())
        .with(Width::wrap(20).keep_words(true))
        .with(MinWidth::new(20)),
    "|  |  | column  |  |"
    "|  |  | 1       |  |"
    "|--|--|---------|--|"
    "|  |  |   0-1   |  |"
    "|  |  |   1-1   |  |"
    "|  |  | some    |  |"
    "|  |  | loong   |  |"
    "|  |  | string  |  |"
);

test_table!(
    total_width_small_with_panel_using_wrapping_0,
    Matrix::new(3, 3)
        .with(Panel::horizontal(0, "Hello World"))
        .with(Modify::new(Segment::all()).with(Alignment::center()))
        .with(Style::markdown())
        .with(Width::wrap(20))
        .with(MinWidth::new(20)),
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
);

test_table!(
    total_width_small_with_panel_using_wrapping_1,
    Matrix::new(3, 3)
        .with(Panel::horizontal(0, "Hello World"))
        .with(Modify::new(Segment::all()).with(Alignment::center()))
        .with(Style::markdown())
        .with(Width::wrap(14))
        .with(MinWidth::new(14)),
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
);

test_table!(
    total_width_small_with_panel_using_wrapping_2,
    Matrix::new(3, 3)
        .with(Panel::horizontal(0, "Hello World 123"))
        .with(Modify::new(Segment::all()).with(Alignment::center()))
        .with(Style::markdown())
        .with(Width::wrap(14))
        .with(MinWidth::new(14)),
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
);

test_table!(
    max_width_with_span_0,
    Matrix::new(3, 3)
        .insert((1, 1).into(), "a long string")
        .to_table()
        .with(Style::psql())
        .with(Modify::new((1, 1)).with(Span::column(2)))
        .with(Modify::new((2, 2)).with(Span::column(2)))
        .with(Width::truncate(40)),
    " N | column 0 | column 1 | column 2 "
    "---+----------+----------+----------"
    " 0 |    a long string    |   0-2    "
    " 1 |   1-0    |         1-1         "
    " 2 |   2-0    |   2-1    |   2-2    "
);

test_table!(
    max_width_with_span_1,
    Matrix::new(3, 3)
        .insert((1, 1).into(), "a long string")
        .to_table()
        .with(Style::psql())
        .with(Modify::new((1, 1)).with(Span::column(2)))
        .with(Modify::new((2, 2)).with(Span::column(2)))
        .with(Width::truncate(40))
        .with(Width::truncate(20)),
    "  | col | col | col "
    "--+-----+-----+-----"
    "  | a long st | 0-2 "
    "  | 1-0 |    1-1    "
    "  | 2-0 | 2-1 | 2-2 "
);

test_table!(
    max_width_with_span_2,
    Matrix::new(3, 3)
        .insert((1, 1).into(), "a long string")
        .to_table()
        .with(Style::psql())
        .with(Modify::new((1, 1)).with(Span::column(2)))
        .with(Modify::new((2, 2)).with(Span::column(2)))
        .with(Width::truncate(40))
        .with(Width::truncate(20))
        .with(Width::truncate(10)),
    "  |  |  |  "
    "--+--+--+--"
    "  | a l |  "
    "  |  | 1-1 "
    "  |  |  |  "
);

test_table!(
    min_width_works_with_right_alignment_0,
    {
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

        Matrix::iter([json])
            .with(Style::markdown())
            .with(Alignment::right())
            .with(TrimStrategy::None)
            .with(MinWidth::new(50))
    },
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
);

test_table!(
    min_width_works_with_right_alignment_1,
    {
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

        Matrix::iter([json])
            .with(Style::markdown())
            .with(Alignment::right())
            .with(TrimStrategy::None)
            .with(MinWidth::new(50))
            .with(TrimStrategy::Horizontal)
            .with(MinWidth::new(50))
    },
    r#"|                          &str |"#
    r#"|-------------------------------|"#
    r#"|                               |"#
    r#"|             {                 |"#
    r#"|             "some": "random", |"#
    r#"|             "json": [         |"#
    r#"|             { "1": "2" },     |"#
    r#"|             { "1": "2" },     |"#
    r#"|             { "1": "2" }      |"#
    r#"|             ]                 |"#
    r#"|             }                 |"#
    r#"|                               |"#
);

test_table!(
    min_width_works_with_right_alignment_2,
    {
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

        Matrix::iter([json])
            .with(Style::markdown())
            .with(Alignment::right())
            .with(TrimStrategy::None)
            .with(MinWidth::new(50))
            .with(TrimStrategy::Both)
            .with(MinWidth::new(50))
    },
    r#"|                          &str |"#
    r#"|-------------------------------|"#
    r#"|             {                 |"#
    r#"|             "some": "random", |"#
    r#"|             "json": [         |"#
    r#"|             { "1": "2" },     |"#
    r#"|             { "1": "2" },     |"#
    r#"|             { "1": "2" }      |"#
    r#"|             ]                 |"#
    r#"|             }                 |"#
    r#"|                               |"#
    r#"|                               |"#
);

test_table!(
    min_width_works_with_right_alignment_3,
    {
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

        Matrix::iter([json])
            .with(Style::markdown())
            .with(Alignment::center())
            .with(TrimStrategy::None)
            .with(MinWidth::new(50))
    },
    r#"|                      &str                      |"#
    r#"|------------------------------------------------|"#
    r#"|                                                |"#
    r#"|                 {                              |"#
    r#"|                     "some": "random",          |"#
    r#"|                     "json": [                  |"#
    r#"|                         { "1": "2" },          |"#
    r#"|                         { "1": "2" },          |"#
    r#"|                         { "1": "2" }           |"#
    r#"|                     ]                          |"#
    r#"|                 }                              |"#
    r#"|                                                |"#
);

test_table!(
    min_width_works_with_right_alignment_4,
    {
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

        Matrix::iter([json])
            .with(Style::markdown())
            .with(Alignment::center())
            .with(TrimStrategy::Horizontal)
            .with(MinWidth::new(50))
    },
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
);

test_table!(
    min_width_works_with_right_alignment_5,
    {
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

        Matrix::iter([json])
            .with(Style::markdown())
            .with(Alignment::center())
            .with(TrimStrategy::Both)
            .with(MinWidth::new(50))
    },
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
);

test_table!(
    min_width_with_span_1,
    Matrix::iter([
        ["0", "1"],
        ["a long string which will affect min width logic", ""],
        ["2", "3"],
    ])
    .with(Style::markdown())
    .with(Modify::new((1, 0)).with(Span::column(2)))
    .with(MinWidth::new(100)),
    "|                                   0                                    |            1            |"
    "|------------------------------------------------------------------------|-------------------------|"
    "|                                                0                                                 |"
    "|            a long string which will affect min width logic             |                         |"
    "|                                   2                                    |            3            |"
);

test_table!(
    min_width_with_span_2,
    Matrix::iter([
        ["0", "1"],
        ["a long string which will affect min width logic", ""],
        ["2", "3"],
    ])
    .with(Style::markdown())
    .with(Modify::new((2, 0)).with(Span::column(2)))
    .with(MinWidth::new(100)),
    "|                        0                        |                       1                        |"
    "|-------------------------------------------------|------------------------------------------------|"
    "|                        0                        |                       1                        |"
    "|                         a long string which will affect min width logic                          |"
    "|                        2                        |                       3                        |"
);

test_table!(
    justify_width_constant_test,
    Matrix::new(3, 3)
        .with(Style::markdown())
        .with(Justify::new(3)),
    "| N   | col | col | col |"
    "|-----|-----|-----|-----|"
    "| 0   | 0-0 | 0-1 | 0-2 |"
    "| 1   | 1-0 | 1-1 | 1-2 |"
    "| 2   | 2-0 | 2-1 | 2-2 |"
);

test_table!(
    justify_width_constant_different_sizes_test,
    Matrix::new(3, 3)
        .insert((1, 1).into(), "Hello World")
        .insert((3, 2).into(), "multi\nline string\n")
        .with(Style::markdown())
        .with(Justify::new(3)),
    "| N   | col | col | col |"
    "|-----|-----|-----|-----|"
    "| 0   | Hel | 0-1 | 0-2 |"
    "| 1   | 1-0 | 1-1 | 1-2 |"
    "| 2   | 2-0 | mul | 2-2 |"
);

test_table!(
    justify_width_constant_0_test,
    Matrix::new(3, 3)
        .with(Style::markdown())
        .with(Justify::new(0)),
    "|  |  |  |  |"
    "|--|--|--|--|"
    "|  |  |  |  |"
    "|  |  |  |  |"
    "|  |  |  |  |"
);

test_table!(
    justify_width_min_test,
    Matrix::new(3, 3)
        .with(Style::markdown())
        .with(Justify::min()),
    "| N | c | c | c |"
    "|---|---|---|---|"
    "| 0 | 0 | 0 | 0 |"
    "| 1 | 1 | 1 | 1 |"
    "| 2 | 2 | 2 | 2 |"
);

test_table!(
    justify_width_max_test,
    Matrix::new(3, 3)
        .with(Style::markdown())
        .with(Justify::max()),
    "| N        | column 0 | column 1 | column 2 |"
    "|----------|----------|----------|----------|"
    "| 0        | 0-0      | 0-1      | 0-2      |"
    "| 1        | 1-0      | 1-1      | 1-2      |"
    "| 2        | 2-0      | 2-1      | 2-2      |"
);

test_table!(
    max_width_when_cell_has_tabs,
    Matrix::new(3, 3)
        .insert((2, 1).into(), "\tHello\tWorld\t")
        .with(TabSize::new(4))
        .with(Style::markdown())
        .with(Modify::new(Columns::new(..)).with(Width::truncate(1))),
    "| N | c | c | c |"
    "|---|---|---|---|"
    "| 0 | 0 | 0 | 0 |"
    "| 1 |   | 1 | 1 |"
    "| 2 | 2 | 2 | 2 |"
);

test_table!(
    max_width_table_when_cell_has_tabs,
    Matrix::new(3, 3)
        .insert((2, 1).into(), "\tHello\tWorld\t")
        .with(TabSize::new(4))
        .with(Style::markdown())
        .with(Width::truncate(15)),
    "|  | co |  |  |"
    "|--|----|--|--|"
    "|  | 0- |  |  |"
    "|  |    |  |  |"
    "|  | 2- |  |  |"
);

test_table!(
    max_width_truncate_with_big_span_0,
    Matrix::new(3, 3)
        .insert((2, 1).into(), "Hello World With Big Line; Here we gooooooo")
        .with(Style::markdown())
        .with(Modify::new((2, 1)).with(Span::column(3)))
        .with(Width::truncate(40)),
    "|  | column 0  | column 1  | column 2  |"
    "|--|-----------|-----------|-----------|"
    "|  |    0-0    |    0-1    |    0-2    |"
    "|  | Hello World With Big Line; Here w |"
    "|  |    2-0    |    2-1    |    2-2    |"
);

test_table!(
    max_width_truncate_with_big_span_1,
    Matrix::new(3, 3)
        .insert((2, 1).into(), "Hello World With Big Line; Here we gooooooo")
        .insert((3, 2).into(), "Hello World With Big Line; Here")
        .with(Style::markdown())
        .with(Modify::new((2, 1)).with(Span::column(3)))
        .with(Modify::new((3, 2)).with(Span::column(2))),
    "| N | column 0  |    column 1    |    column 2    |"
    "|---|-----------|----------------|----------------|"
    "| 0 |    0-0    |      0-1       |      0-2       |"
    "| 1 | Hello World With Big Line; Here we gooooooo |"
    "| 2 |    2-0    | Hello World With Big Line; Here |"
);

test_table!(
    max_width_truncate_with_big_span_2,
    Matrix::new(3, 3)
        .insert((2, 1).into(), "Hello World With Big Line; Here we gooooooo")
        .insert((3, 2).into(), "Hello World With Big Line; Here")
        .with(Style::markdown())
        .with(Modify::new((2, 1)).with(Span::column(3)))
        .with(Modify::new((3, 2)).with(Span::column(2)))
        .with(Width::truncate(40)),
    "|  | colum |  column 1   |  column 2   |"
    "|--|-------|-------------|-------------|"
    "|  |  0-0  |     0-1     |     0-2     |"
    "|  | Hello World With Big Line; Here w |"
    "|  |  2-0  | Hello World With Big Line |"
);

test_table!(
    max_width_truncate_with_big_span_3,
    Matrix::new(3, 3)
        .insert((2, 1).into(), "Hello World With Big Line; Here we gooooooo")
        .insert((3, 2).into(), "Hello World With Big Line; Here")
        .with(Style::markdown())
        .with(Modify::new((2, 1)).with(Span::column(2)))
        .with(Modify::new((3, 2)).with(Span::column(2)))
        .with(Width::truncate(40)),
    "|  |   column 0    |   column 1    | c |"
    "|--|---------------|---------------|---|"
    "|  |      0-0      |      0-1      | 0 |"
    "|  | Hello World With Big Line; He | 1 |"
    "|  |      2-0      | Hello World With  |"
);

test_table!(
    max_width_truncate_with_big_span_4,
    Matrix::new(3, 3)
        .insert((2, 1).into(), "Hello World With Big Line; Here w")
        .insert((3, 2).into(), "Hello World With Big L")
        .with(Style::markdown())
        .with(Modify::new((2, 1)).with(Span::column(3)))
        .with(Modify::new((3, 2)).with(Span::column(2))),
    "| N | column 0 |  column 1  | column 2  |"
    "|---|----------|------------|-----------|"
    "| 0 |   0-0    |    0-1     |    0-2    |"
    "| 1 | Hello World With Big Line; Here w |"
    "| 2 |   2-0    | Hello World With Big L |"
);

test_table!(
    max_width_truncate_priority_max_0,
    Matrix::new(3, 3)
        .insert((2, 1).into(), "Hello World With Big Line")
        .with(Style::markdown())
        .with(Width::truncate(35).priority(PriorityMax::right())),
    "| N | column  | column  | column  |"
    "|---|---------|---------|---------|"
    "| 0 |   0-0   |   0-1   |   0-2   |"
    "| 1 | Hello W |   1-1   |   1-2   |"
    "| 2 |   2-0   |   2-1   |   2-2   |"
);

test_table!(
    max_width_truncate_priority_max_1,
    Matrix::new(3, 3)
        .insert((2, 1).into(), "Hello World With Big Line")
        .with(Style::markdown())
        .with(Width::truncate(20).priority(PriorityMax::right())),
    "| N | co | co | co |"
    "|---|----|----|----|"
    "| 0 | 0- | 0- | 0- |"
    "| 1 | He | 1- | 1- |"
    "| 2 | 2- | 2- | 2- |"
);

test_table!(
    max_width_truncate_priority_max_2,
    Matrix::new(3, 3)
        .insert((2, 1).into(), "Hello World With Big Line")
        .with(Style::markdown())
        .with(Width::truncate(0).priority(PriorityMax::right())),
    "|  |  |  |  |"
    "|--|--|--|--|"
    "|  |  |  |  |"
    "|  |  |  |  |"
    "|  |  |  |  |"
);

test_table!(
    max_width_truncate_priority_max_with_span,
    Matrix::new(3, 3)
        .insert((2, 1).into(), "Hello World With Big Line")
        .with(Style::markdown())
        .with(Modify::new((2, 1)).with(Span::column(2)))
        .with(Width::truncate(15).priority(PriorityMax::right())),
    "| N | c |  |  |"
    "|---|---|--|--|"
    "| 0 | 0 |  |  |"
    "| 1 | Hell |  |"
    "| 2 | 2 |  |  |"
);

test_table!(
    max_width_wrap_priority_max_0,
    Matrix::new(3, 3)
        .insert((2, 1).into(), "Hello World With Big Line")
        .with(Style::markdown())
        .with(Width::wrap(35).priority(PriorityMax::right())),
    "| N | column  | column  | column  |"
    "|   | 0       | 1       | 2       |"
    "|---|---------|---------|---------|"
    "| 0 |   0-0   |   0-1   |   0-2   |"
    "| 1 | Hello W |   1-1   |   1-2   |"
    "|   | orld Wi |         |         |"
    "|   | th Big  |         |         |"
    "|   | Line    |         |         |"
    "| 2 |   2-0   |   2-1   |   2-2   |"
);

test_table!(
    max_width_wrap_priority_max_1,
    Matrix::new(3, 3)
        .insert((2, 1).into(), "Hello World With Big Line")
        .with(Style::markdown())
        .with(Width::wrap(20).priority(PriorityMax::right())),
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
);

test_table!(
    max_width_wrap_priority_max_2,
    Matrix::new(3, 3)
        .insert((2, 1).into(), "Hello World With Big Line")
        .with(Style::markdown())
        .with(Width::wrap(0).priority(PriorityMax::right())),
    "|  |  |  |  |"
    "|--|--|--|--|"
    "|  |  |  |  |"
    "|  |  |  |  |"
    "|  |  |  |  |"
);

test_table!(
    max_width_wrap_priority_max_with_span,
    Matrix::new(3, 3)
        .insert((2, 1).into(), "Hello World With Big Line")
        .with(Style::markdown())
        .with(Modify::new((2, 1)).with(Span::column(2)))
        .with(Width::wrap(15).priority(PriorityMax::right())),
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
);

test_table!(
    max_width_truncate_priority_min_0,
    Matrix::new(3, 3)
        .insert((2, 1).into(), "Hello World With Big Line")
        .with(Style::markdown())
        .with(Width::truncate(35).priority(PriorityMin::right())),
    "|  |        column 0        |  |  |"
    "|--|------------------------|--|--|"
    "|  |          0-0           |  |  |"
    "|  | Hello World With Big L |  |  |"
    "|  |          2-0           |  |  |"
);

test_table!(
    max_width_truncate_priority_min_1,
    Matrix::new(3, 3)
        .insert((2, 1).into(), "Hello World With Big Line")
        .with(Style::markdown())
        .with(Width::truncate(20).priority(PriorityMin::right())),
    "|  | column  |  |  |"
    "|--|---------|--|--|"
    "|  |   0-0   |  |  |"
    "|  | Hello W |  |  |"
    "|  |   2-0   |  |  |"
);

test_table!(
    max_width_truncate_priority_min_2,
    Matrix::new(3, 3)
        .insert((2, 1).into(), "Hello World With Big Line")
        .with(Style::markdown())
        .with(Width::truncate(0).priority(PriorityMin::right())),
    "|  |  |  |  |"
    "|--|--|--|--|"
    "|  |  |  |  |"
    "|  |  |  |  |"
    "|  |  |  |  |"
);

test_table!(
    max_width_truncate_priority_min_with_span_0,
    Matrix::new(3, 3)
        .insert((2, 1).into(), "Hello World With Big Line")
        .with(Style::markdown())
        .with(Modify::new((2, 1)).with(Span::column(2)))
        .with(Width::truncate(15).priority(PriorityMin::right())),
    "|  |  | co |  |"
    "|--|--|----|--|"
    "|  |  | 0- |  |"
    "|  | Hello |  |"
    "|  |  | 2- |  |"
);

test_table!(
    max_width_truncate_priority_min_with_span_1,
    Matrix::new(3, 3)
        .insert((2, 1).into(), "Hello World With Big Line")
        .with(Style::markdown())
        .with(Modify::new((2, 1)).with(Span::column(2)))
        .with(Width::truncate(17).priority(PriorityMin::right())),
    "|  |  | colu |  |"
    "|--|--|------|--|"
    "|  |  | 0-1  |  |"
    "|  | Hello W |  |"
    "|  |  | 2-1  |  |"
);

test_table!(
    max_width_wrap_priority_min_0,
    Matrix::new(3, 3)
        .insert((2, 1).into(), "Hello World With Big Line")
        .with(Style::markdown())
        .with(Width::wrap(35).priority(PriorityMin::right())),
    "|  |        column 0        |  |  |"
    "|--|------------------------|--|--|"
    "|  |          0-0           |  |  |"
    "|  | Hello World With Big L |  |  |"
    "|  | ine                    |  |  |"
    "|  |          2-0           |  |  |"
);

test_table!(
    max_width_wrap_priority_min_1,
    Matrix::new(3, 3)
        .insert((2, 1).into(), "Hello World With Big Line")
        .with(Style::markdown())
        .with(Width::wrap(20).priority(PriorityMin::right())),
    "|  | column  |  |  |"
    "|  | 0       |  |  |"
    "|--|---------|--|--|"
    "|  |   0-0   |  |  |"
    "|  | Hello W |  |  |"
    "|  | orld Wi |  |  |"
    "|  | th Big  |  |  |"
    "|  | Line    |  |  |"
    "|  |   2-0   |  |  |"
);

test_table!(
    max_width_wrap_priority_min_2,
    Matrix::new(3, 3)
        .insert((2, 1).into(), "Hello World With Big Line")
        .with(Style::markdown())
        .with(Width::wrap(0).priority(PriorityMin::right())),
    "|  |  |  |  |"
    "|--|--|--|--|"
    "|  |  |  |  |"
    "|  |  |  |  |"
    "|  |  |  |  |"
);

test_table!(
    max_width_wrap_priority_min_with_span,
    Matrix::new(3, 3)
        .insert((2, 1).into(), "Hello World With Big Line")
        .with(Style::markdown())
        .with(Modify::new((2, 1)).with(Span::column(2)))
        .with(Width::wrap(15).priority(PriorityMin::right())),
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
);

test_table!(
    min_width_priority_max,
    Matrix::new(3, 3)
        .with(Style::markdown())
        .with(MinWidth::new(60).priority(PriorityMax::right())),
    "| N | column 0 | column 1 |            column 2            |"
    "|---|----------|----------|--------------------------------|"
    "| 0 |   0-0    |   0-1    |              0-2               |"
    "| 1 |   1-0    |   1-1    |              1-2               |"
    "| 2 |   2-0    |   2-1    |              2-2               |"
);

test_table!(
    min_width_priority_min,
    Matrix::new(3, 3)
        .with(Style::markdown())
        .with(MinWidth::new(60).priority(PriorityMin::right())),
    "|      N       |   column 0   |   column 1   |  column 2   |"
    "|--------------|--------------|--------------|-------------|"
    "|      0       |     0-0      |     0-1      |     0-2     |"
    "|      1       |     1-0      |     1-1      |     1-2     |"
    "|      2       |     2-0      |     2-1      |     2-2     |"
);

test_table!(
    max_width_tab_0,
    Matrix::iter(["\t\tTigre Ecuador\tOMYA Andina\t3824909999\tCalcium carbonate\tColombia\t"])
            .with(TabSize::new(4))
            .with(Style::markdown())
            .with(Width::wrap(60)),
    "|                           &str                           |"
    "|----------------------------------------------------------|"
    "|         Tigre Ecuador    OMYA Andina    3824909999    Ca |"
    "| lcium carbonate    Colombia                              |"
);

test_table!(
    min_width_is_not_used_after_padding,
    Matrix::new(3, 3)
        .with(Style::markdown())
        .with(MinWidth::new(60))
        .modify((0, 0), Padding::new(2, 2, 0, 0)),
    "|  N  | column 0 | column 1 | column 2 |"
    "|-----|----------|----------|----------|"
    "|  0  |   0-0    |   0-1    |   0-2    |"
    "|  1  |   1-0    |   1-1    |   1-2    |"
    "|  2  |   2-0    |   2-1    |   2-2    |"
);

test_table!(
    min_width_is_used_after_margin,
    Matrix::new(3, 3)
        .with(Style::markdown())
        .with(Margin::new(1, 1, 1, 1))
        .with(Width::increase(60)),
    "                                                            "
    " |   N    |   column 0    |   column 1    |   column 2    | "
    " |--------|---------------|---------------|---------------| "
    " |   0    |      0-0      |      0-1      |      0-2      | "
    " |   1    |      1-0      |      1-1      |      1-2      | "
    " |   2    |      2-0      |      2-1      |      2-2      | "
    "                                                            "
);

test_table!(
    wrap_keeping_words_0,
    Table::new(vec![["Hello world"]])
        .with(Width::wrap(8).keep_words(true)),
    "+------+"
    "| 0    |"
    "+------+"
    "| Hell |"
    "| o wo |"
    "| rld  |"
    "+------+"
);

test_table!(
    cell_truncate_multiline,
    Matrix::new(3, 3)
        .insert((1, 1).into(), "H\nel\nlo World")
        .insert((3, 2).into(), "multi\nline string\n")
        .with(Style::markdown())
        .modify(
            Columns::new(1..2).not(Rows::single(0)),
            Width::truncate(1).multiline(true),
        ),
    "| N | column 0 |  column 1   | column 2 |"
    "|---|----------|-------------|----------|"
    "| 0 |    H     |     0-1     |   0-2    |"
    "|   |    e     |             |          |"
    "|   |    l     |             |          |"
    "| 1 |    1     |     1-1     |   1-2    |"
    "| 2 |    2     | multi       |   2-2    |"
    "|   |          | line string |          |"
    "|   |          |             |          |"
);

test_table!(
    cell_truncate_multiline_with_suffix,
    Matrix::new(3, 3)
        .insert((1, 1).into(), "H\nel\nlo World")
        .insert((3, 2).into(), "multi\nline string\n")
        .with(Style::markdown())
        .with(
            Modify::new(Columns::new(1..2).not(Rows::single(0)))
                .with(Width::truncate(1).multiline(true).suffix(".")),
        ),
    "| N | column 0 |  column 1   | column 2 |"
    "|---|----------|-------------|----------|"
    "| 0 |    .     |     0-1     |   0-2    |"
    "|   |    .     |             |          |"
    "|   |    .     |             |          |"
    "| 1 |    .     |     1-1     |   1-2    |"
    "| 2 |    .     | multi       |   2-2    |"
    "|   |          | line string |          |"
    "|   |          |             |          |"
);

test_table!(
    table_truncate_multiline,
    Matrix::new(3, 3)
        .insert((1, 1).into(), "H\nel\nlo World")
        .insert((3, 2).into(), "multi\nline string\n")
        .with(Style::markdown())
        .with(Width::truncate(20).multiline(true)),
    "|  | c | colu | co |"
    "|--|---|------|----|"
    "|  | H | 0-1  | 0- |"
    "|  | e |      |    |"
    "|  | l |      |    |"
    "|  | 1 | 1-1  | 1- |"
    "|  | 2 | mult | 2- |"
    "|  |   | line |    |"
    "|  |   |      |    |"
);

test_table!(
    table_truncate_multiline_with_suffix,
    Matrix::new(3, 3)
        .insert((1, 1).into(), "H\nel\nlo World")
        .insert((3, 2).into(), "multi\nline string\n")
        .with(Style::markdown())
        .with(Width::truncate(20).suffix(".").multiline(true)),
    "|  | . | col. | c. |"
    "|--|---|------|----|"
    "|  | . | 0-1  | 0. |"
    "|  | . |      |    |"
    "|  | . |      |    |"
    "|  | . | 1-1  | 1. |"
    "|  | . | mul. | 2. |"
    "|  |   | lin. |    |"
    "|  |   | .    |    |"
);

test_table!(
    test_priority_left,
    Matrix::new(3, 10)
        .with(Style::markdown())
        .with(Width::wrap(60).priority(PriorityLeft::default())),
    "|  |  |  |  |  |  |  | co | column 7 | column 8 | column 9 |"
    "|  |  |  |  |  |  |  | lu |          |          |          |"
    "|  |  |  |  |  |  |  | mn |          |          |          |"
    "|  |  |  |  |  |  |  |  6 |          |          |          |"
    "|--|--|--|--|--|--|--|----|----------|----------|----------|"
    "|  |  |  |  |  |  |  | 0- |   0-7    |   0-8    |   0-9    |"
    "|  |  |  |  |  |  |  | 6  |          |          |          |"
    "|  |  |  |  |  |  |  | 1- |   1-7    |   1-8    |   1-9    |"
    "|  |  |  |  |  |  |  | 6  |          |          |          |"
    "|  |  |  |  |  |  |  | 2- |   2-7    |   2-8    |   2-9    |"
    "|  |  |  |  |  |  |  | 6  |          |          |          |"
);

test_table!(
    test_priority_right,
    Matrix::new(3, 10)
        .with(Style::markdown())
        .with(Width::wrap(60).priority(PriorityRight::default())),
    "| N | column 0 | column 1 | column 2 | c |  |  |  |  |  |  |"
    "|   |          |          |          | o |  |  |  |  |  |  |"
    "|   |          |          |          | l |  |  |  |  |  |  |"
    "|   |          |          |          | u |  |  |  |  |  |  |"
    "|   |          |          |          | m |  |  |  |  |  |  |"
    "|   |          |          |          | n |  |  |  |  |  |  |"
    "|   |          |          |          |   |  |  |  |  |  |  |"
    "|   |          |          |          | 3 |  |  |  |  |  |  |"
    "|---|----------|----------|----------|---|--|--|--|--|--|--|"
    "| 0 |   0-0    |   0-1    |   0-2    | 0 |  |  |  |  |  |  |"
    "|   |          |          |          | - |  |  |  |  |  |  |"
    "|   |          |          |          | 3 |  |  |  |  |  |  |"
    "| 1 |   1-0    |   1-1    |   1-2    | 1 |  |  |  |  |  |  |"
    "|   |          |          |          | - |  |  |  |  |  |  |"
    "|   |          |          |          | 3 |  |  |  |  |  |  |"
    "| 2 |   2-0    |   2-1    |   2-2    | 2 |  |  |  |  |  |  |"
    "|   |          |          |          | - |  |  |  |  |  |  |"
    "|   |          |          |          | 3 |  |  |  |  |  |  |"
);

test_table!(
    priority_max_left,
    Matrix::new(3, 3)
        .insert((2, 1).into(), "Hello World With Big Line")
        .insert((2, 2).into(), "Hello World With Big Line")
        .with(Style::markdown())
        .with(Width::truncate(37).priority(PriorityMax::left())),
    "| N | column  | column 1 | column 2 |"
    "|---|---------|----------|----------|"
    "| 0 |   0-0   |   0-1    |   0-2    |"
    "| 1 | Hello W | Hello Wo |   1-2    |"
    "| 2 |   2-0   |   2-1    |   2-2    |"
);

test_table!(
    priority_max_right,
    Matrix::new(3, 3)
        .insert((2, 1).into(), "Hello World With Big Line")
        .insert((2, 2).into(), "Hello World With Big Line")
        .with(Style::markdown())
        .with(Width::truncate(37).priority(PriorityMax::right())),
    "| N | column 0 | column 1 | column  |"
    "|---|----------|----------|---------|"
    "| 0 |   0-0    |   0-1    |   0-2   |"
    "| 1 | Hello Wo | Hello Wo |   1-2   |"
    "| 2 |   2-0    |   2-1    |   2-2   |"
);

test_table!(
    priority_min_left,
    Matrix::new(3, 3).with(Width::truncate(30).priority(PriorityMin::left())),
    "+--+----------+----------+---+"
    "|  | column 0 | column 1 | c |"
    "+--+----------+----------+---+"
    "|  |   0-0    |   0-1    | 0 |"
    "+--+----------+----------+---+"
    "|  |   1-0    |   1-1    | 1 |"
    "+--+----------+----------+---+"
    "|  |   2-0    |   2-1    | 2 |"
    "+--+----------+----------+---+"
);

test_table!(
    priority_min_right,
    Matrix::new(3, 3).with(Width::truncate(30).priority(PriorityMin::right())),
    "+--+---+----------+----------+"
    "|  | c | column 1 | column 2 |"
    "+--+---+----------+----------+"
    "|  | 0 |   0-1    |   0-2    |"
    "+--+---+----------+----------+"
    "|  | 1 |   1-1    |   1-2    |"
    "+--+---+----------+----------+"
    "|  | 2 |   2-1    |   2-2    |"
    "+--+---+----------+----------+"
);

#[cfg(feature = "ansi")]
test_table!(
    wrap_issue_0,
    {
        tabled::Table::new(vec![
            ("x xxx xx xxxxxxx xxxxxxxx xx xxxx xxxxxxx. xx xxxxxxxx xxx ❤️ xx xxxxx xx xxxxxxx x xx xxxxxx x xxxxxx xxxxxxxxxxxx xx xxxxxx xxx xxx xxxxxx xxxxxxx. xx xxxxxxxx xx xx xxxxxxxxxx"),
        ])
        .with(Width::wrap(40).keep_words(true))
        .to_string()
    },
    "+--------------------------------------+"
    "| &str                                 |"
    "+--------------------------------------+"
    "| x xxx xx xxxxxxx xxxxxxxx xx xxxx    |"
    "| xxxxxxx. xx xxxxxxxx xxx ❤️ xx xxxxx |"
    "|  xx xxxxxxx x xx xxxxxx x xxxxxx     |"
    "| xxxxxxxxxxxx xx xxxxxx xxx xxx       |"
    "| xxxxxx xxxxxxx. xx xxxxxxxx xx xx    |"
    "| xxxxxxxxxx                           |"
    "+--------------------------------------+"
);

#[cfg(feature = "ansi")]
test_table!(
    wrap_issue_1,
    {
        tabled::Table::new(vec![
            ("x xxx xx xxxxxxx xxxxxxxx xx xxxx xxxxxxx. xx xxxxxxxx xxx ❤️ xx xxxxx xx xxxxxxx x xx xxxxxx x xxxxxx xxxxxxxxxxxx xx xxxxxx xxx xxx xxxxxx xxxxxxx. xx xxxxxxxx xx xx xxxxxxxxxx"),
        ])
        .with(Width::wrap(40).keep_words(false))
        .to_string()
    },
    "+--------------------------------------+"
    "| &str                                 |"
    "+--------------------------------------+"
    "| x xxx xx xxxxxxx xxxxxxxx xx xxxx xx |"
    "| xxxxx. xx xxxxxxxx xxx ❤️ xx xxxxx x |"
    "| x xxxxxxx x xx xxxxxx x xxxxxx xxxxx |"
    "| xxxxxxx xx xxxxxx xxx xxx xxxxxx xxx |"
    "| xxxx. xx xxxxxxxx xx xx xxxxxxxxxx   |"
    "+--------------------------------------+"
);

#[cfg(feature = "derive")]
mod derived {
    use super::*;

    #[cfg(feature = "ansi")]
    use tabled::grid::util::string::get_text_width;
    use tabled::Tabled;
    use testing_table::static_table;

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
        assert_width!(table, 57);

        let table = Matrix::iter(&data)
            .with(Style::markdown())
            .with(Modify::new(Segment::all()).with(Alignment::left()))
            .with(Width::wrap(57).keep_words(true))
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
        assert_width!(table, 57);
    }

    #[cfg(feature = "ansi")]
    #[test]
    fn wrapping_as_total_multiline_color() {
        use testing_table::assert_table;

        #[derive(Tabled)]
        struct D(
            #[tabled(rename = "version")] String,
            #[tabled(rename = "published_date")] String,
            #[tabled(rename = "is_active")] String,
            #[tabled(rename = "major_feature")] String,
        );

        let data = vec![
            D(
                Color::FG_RED.colorize("0.2.1"),
                (Color::FG_RED | Color::rgb_bg(8, 10, 30)).colorize("2021-06-23"),
                "true".to_string(),
                (Color::FG_BLUE | Color::BG_GREEN).colorize("#[header(inline)] attribute"),
            ),
            D(
                Color::FG_RED.colorize("0.2.0"),
                (Color::FG_GREEN | Color::rgb_bg(8, 100, 30)).colorize("2021-06-19"),
                "false".to_string(),
                Color::FG_YELLOW.colorize("API changes"),
            ),
            D(
                Color::FG_WHITE.colorize("0.1.4"),
                (Color::FG_RED | Color::rgb_bg(8, 10, 30)).colorize("2021-06-07"),
                "false".to_string(),
                (Color::FG_RED | Color::BG_BLACK).colorize("display_with attribute"),
            ),
        ];

        let table = Matrix::iter(&data)
            .with(Style::markdown())
            .with(Alignment::left())
            .with(Width::wrap(57))
            .to_string();

        assert_eq!(
            table,
            static_table!(
                "| ver | published_d | is_act | major_feature            |"
                "| sio | ate         | ive    |                          |"
                "| n   |             |        |                          |"
                "|-----|-------------|--------|--------------------------|"
                "| \u{1b}[31m0.2\u{1b}[39m | \u{1b}[31m\u{1b}[48;2;8;10;30m2021-06-23\u{1b}[39m\u{1b}[49m  | true   | \u{1b}[34m\u{1b}[42m#[header(inline)] attrib\u{1b}[39m\u{1b}[49m |"
                "| \u{1b}[31m.1\u{1b}[39m  |             |        | \u{1b}[34m\u{1b}[42mute\u{1b}[39m\u{1b}[49m                      |"
                "| \u{1b}[31m0.2\u{1b}[39m | \u{1b}[32m\u{1b}[48;2;8;100;30m2021-06-19\u{1b}[39m\u{1b}[49m  | false  | \u{1b}[33mAPI changes\u{1b}[39m              |"
                "| \u{1b}[31m.0\u{1b}[39m  |             |        |                          |"
                "| \u{1b}[37m0.1\u{1b}[39m | \u{1b}[31m\u{1b}[48;2;8;10;30m2021-06-07\u{1b}[39m\u{1b}[49m  | false  | \u{1b}[31m\u{1b}[40mdisplay_with attribute\u{1b}[39m\u{1b}[49m   |"
                "| \u{1b}[37m.4\u{1b}[39m  |             |        |                          |"
            )
        );
        assert_eq!(get_text_width(&table), 57);

        let table = Matrix::iter(&data)
            .with(Style::markdown())
            .with(Modify::new(Segment::all()).with(Alignment::left()))
            .with(Width::wrap(57).keep_words(true))
            .to_string();

        assert_table!(
            table,
            "| ver | published_d | is_act | major_feature            |"
            "| sio | ate         | ive    |                          |"
            "| n   |             |        |                          |"
            "|-----|-------------|--------|--------------------------|"
            "| \u{1b}[31m0.2\u{1b}[39m | \u{1b}[31m\u{1b}[48;2;8;10;30m2021-06-23\u{1b}[39m\u{1b}[49m  | true   | \u{1b}[34m\u{1b}[42m#[header(inline)] \u{1b}[39m\u{1b}[49m       |"
            "| \u{1b}[31m.1\u{1b}[39m  |             |        | \u{1b}[34m\u{1b}[42mattribute\u{1b}[39m\u{1b}[49m                |"
            "| \u{1b}[31m0.2\u{1b}[39m | \u{1b}[32m\u{1b}[48;2;8;100;30m2021-06-19\u{1b}[39m\u{1b}[49m  | false  | \u{1b}[33mAPI changes\u{1b}[39m              |"
            "| \u{1b}[31m.0\u{1b}[39m  |             |        |                          |"
            "| \u{1b}[37m0.1\u{1b}[39m | \u{1b}[31m\u{1b}[48;2;8;10;30m2021-06-07\u{1b}[39m\u{1b}[49m  | false  | \u{1b}[31m\u{1b}[40mdisplay_with attribute\u{1b}[39m\u{1b}[49m   |"
            "| \u{1b}[37m.4\u{1b}[39m  |             |        |                          |"
        );
        assert_eq!(get_text_width(&table), 57);
    }

    #[cfg(feature = "ansi")]
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
                Color::FG_RED.colorize("0.2.1"),
                (Color::FG_RED | Color::rgb_bg(8, 10, 30)).colorize("2021-06-23"),
                "true".to_string(),
                (Color::FG_BLUE | Color::BG_GREEN).colorize("#[header(inline)] attribute"),
            ),
            D(
                Color::FG_RED.colorize("0.2.0"),
                (Color::FG_GREEN | Color::rgb_bg(8, 100, 30)).colorize("2021-06-19"),
                "false".to_string(),
                Color::FG_YELLOW.colorize("API changes"),
            ),
            D(
                Color::FG_WHITE.colorize("0.1.4"),
                (Color::FG_RED | Color::rgb_bg(8, 10, 30)).colorize("2021-06-07"),
                "false".to_string(),
                (Color::FG_RED | Color::BG_BLACK).colorize("display_with attribute"),
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
            "| ver | published_d | is_act | major_feature            |\n|-----|-------------|--------|--------------------------|\n| \u{1b}[31m0.2\u{1b}[39m | \u{1b}[31m\u{1b}[48;2;8;10;30m2021-06-23\u{1b}[39m\u{1b}[49m  | true   | \u{1b}[34m\u{1b}[42m#[header(inline)] attrib\u{1b}[39m\u{1b}[49m |\n| \u{1b}[31m0.2\u{1b}[39m | \u{1b}[32m\u{1b}[48;2;8;100;30m2021-06-19\u{1b}[39m\u{1b}[49m  | false  | \u{1b}[33mAPI changes\u{1b}[39m              |\n| \u{1b}[37m0.1\u{1b}[39m | \u{1b}[31m\u{1b}[48;2;8;10;30m2021-06-07\u{1b}[39m\u{1b}[49m  | false  | \u{1b}[31m\u{1b}[40mdisplay_with attribute\u{1b}[39m\u{1b}[49m   |"
        );
        assert_eq!(get_text_width(&table), 57);
    }

    #[cfg(feature = "ansi")]
    fn format_osc8_hyperlink(url: &str, text: &str) -> String {
        format!(
            "{osc}8;;{url}{st}{text}{osc}8;;{st}",
            url = url,
            text = text,
            osc = "\x1b]",
            st = "\x1b\\"
        )
    }

    #[cfg(feature = "ansi")]
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
                        .with(Width::wrap(5).keep_words(true))
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

    #[cfg(feature = "ansi")]
    #[test]
    fn hyperlinks_with_color() {
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
                        .with(Width::wrap(6).keep_words(true))
                        .with(Alignment::left()),
                )
                .to_string()
        };

        let text =
            format_osc8_hyperlink("https://www.debian.org/", &Color::FG_RED.colorize("Debian"));
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
