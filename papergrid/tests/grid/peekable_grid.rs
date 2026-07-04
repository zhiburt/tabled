#![cfg(feature = "std")]

use papergrid::{
    colors::NoColors,
    config::{
        spanned::SpannedConfig, AlignmentHorizontal, AlignmentVertical, Borders, Entity, Indent,
        Sides,
    },
    dimension::{iterable::IterGridDimension, peekable::PeekableGridDimension, Dimension},
    grid::peekable::PeekableGrid,
    records::vec_records::{Text, VecRecords},
};

use testing_table::test_table;

struct Dims {
    width: Vec<usize>,
    height: Vec<usize>,
}

impl Dimension for Dims {
    fn get_width(&self, column: usize) -> usize {
        self.width[column]
    }

    fn get_height(&self, row: usize) -> usize {
        self.height[row]
    }
}

test_table!(
    continues_empty_rows_with_horizontal_lines,
    {
        let mut cfg = SpannedConfig::default();
        cfg.set_borders(Borders {
            top: Some('-'),
            top_left: Some('+'),
            top_right: Some('+'),
            top_intersection: Some('+'),
            bottom: Some('-'),
            bottom_left: Some('+'),
            bottom_right: Some('+'),
            bottom_intersection: Some('+'),
            horizontal: Some('-'),
            left_intersection: Some('+'),
            right_intersection: Some('+'),
            vertical: Some('|'),
            left: Some('|'),
            right: Some('|'),
            intersection: Some('+'),
        });
        cfg.set_alignment_horizontal((1, 0).into(), AlignmentHorizontal::Center);
        cfg.set_alignment_vertical(Entity::Global, AlignmentVertical::Center);
        cfg.set_padding(
            (0, 0).into(),
            Sides::new(
                Indent::spaced(4),
                Indent::spaced(4),
                Indent::spaced(1),
                Indent::spaced(1),
            ),
        );

        let data = [
            ["Papergrid", "is a library", "for print tables", "!"],
            ["", "", "", ""],
            ["", "", "", ""],
            ["", "", "", ""],
            ["?", "?", "?", "?"],
            ["", "", "", ""],
            ["is a library", "is a library", "is a library", "is a library"],
        ];

        let data = data
            .iter()
            .map(|row| row.iter().map(Text::new).collect())
            .collect();

        let records = VecRecords::new(data);

        let dims = Dims {
            width: IterGridDimension::width(&records, &cfg),
            height: vec![3, 0, 0, 0, 1, 0, 1],
        };

        PeekableGrid::new(&records, &cfg, &dims, NoColors).to_string()
    },
    "+-----------------+------------+----------------+------------+"
    "|                 |            |                |            |"
    "|    Papergrid    |is a library|for print tables|!           |"
    "|                 |            |                |            |"
    "+-----------------+------------+----------------+------------+"
    "+-----------------+------------+----------------+------------+"
    "+-----------------+------------+----------------+------------+"
    "+-----------------+------------+----------------+------------+"
    "|?                |?           |?               |?           |"
    "+-----------------+------------+----------------+------------+"
    "+-----------------+------------+----------------+------------+"
    "|is a library     |is a library|is a library    |is a library|"
    "+-----------------+------------+----------------+------------+"
);

test_table!(
    continues_empty_rows_with_horizontal_lines_peekable,
    {
        let mut cfg = SpannedConfig::default();
        cfg.set_borders(Borders {
            top: Some('-'),
            top_left: Some('+'),
            top_right: Some('+'),
            top_intersection: Some('+'),
            bottom: Some('-'),
            bottom_left: Some('+'),
            bottom_right: Some('+'),
            bottom_intersection: Some('+'),
            horizontal: Some('-'),
            left_intersection: Some('+'),
            right_intersection: Some('+'),
            vertical: Some('|'),
            left: Some('|'),
            right: Some('|'),
            intersection: Some('+'),
        });
        cfg.set_alignment_horizontal((1, 0).into(), AlignmentHorizontal::Center);
        cfg.set_alignment_vertical(Entity::Global, AlignmentVertical::Center);
        cfg.set_padding(
            (0, 0).into(),
            Sides::new(
                Indent::spaced(4),
                Indent::spaced(4),
                Indent::spaced(1),
                Indent::spaced(1),
            ),
        );

        let data = [
            ["Papergrid", "is a library", "for print tables", "!"],
            ["", "", "", ""],
            ["", "", "", ""],
            ["", "", "", ""],
            ["?", "?", "?", "?"],
            ["", "", "", ""],
            ["is a library", "is a library", "is a library", "is a library"],
        ];

        let data = data
            .iter()
            .map(|row| row.iter().map(Text::new).collect())
            .collect();

        let records = VecRecords::new(data);

        let dims = Dims {
            width: PeekableGridDimension::width(&records, &cfg),
            height: vec![3, 0, 0, 0, 1, 0, 1],
        };

        PeekableGrid::new(&records, &cfg, &dims, NoColors).to_string()
    },
    "+-----------------+------------+----------------+------------+"
    "|                 |            |                |            |"
    "|    Papergrid    |is a library|for print tables|!           |"
    "|                 |            |                |            |"
    "+-----------------+------------+----------------+------------+"
    "+-----------------+------------+----------------+------------+"
    "+-----------------+------------+----------------+------------+"
    "+-----------------+------------+----------------+------------+"
    "|?                |?           |?               |?           |"
    "+-----------------+------------+----------------+------------+"
    "+-----------------+------------+----------------+------------+"
    "|is a library     |is a library|is a library    |is a library|"
    "+-----------------+------------+----------------+------------+"
);

test_table!(
    continues_empty_rows_with_no_horizontal_lines,
    {
        let mut cfg = SpannedConfig::default();
        cfg.set_borders(Borders {
            top: Some('-'),
            top_left: Some('+'),
            top_right: Some('+'),
            top_intersection: Some('+'),
            bottom: Some('-'),
            bottom_left: Some('+'),
            bottom_right: Some('+'),
            bottom_intersection: Some('+'),
            horizontal: None,
            left_intersection: None,
            right_intersection: None,
            vertical: Some('|'),
            left: Some('|'),
            right: Some('|'),
            intersection: None,
        });
        cfg.set_alignment_horizontal((1, 0).into(), AlignmentHorizontal::Center);
        cfg.set_alignment_vertical(Entity::Global, AlignmentVertical::Center);
        cfg.set_padding(
            (0, 0).into(),
            Sides::new(
                Indent::spaced(4),
                Indent::spaced(4),
                Indent::spaced(1),
                Indent::spaced(1),
            ),
        );

        let data = [
            ["Papergrid", "is a library", "for print tables", "!"],
            ["", "", "", ""],
            ["", "", "", ""],
            ["", "", "", ""],
            ["?", "?", "?", "?"],
            ["", "", "", ""],
            ["is a library", "is a library", "is a library", "is a library"],
        ];

        let data = data
            .iter()
            .map(|row| row.iter().map(Text::new).collect())
            .collect();

        let records = VecRecords::new(data);

        let dims = Dims {
            width: IterGridDimension::width(&records, &cfg),
            height: vec![3, 0, 0, 0, 1, 0, 1],
        };

        PeekableGrid::new(&records, &cfg, &dims, NoColors).to_string()
    },
    "+-----------------+------------+----------------+------------+"
    "|                 |            |                |            |"
    "|    Papergrid    |is a library|for print tables|!           |"
    "|                 |            |                |            |"
    "|?                |?           |?               |?           |"
    "|is a library     |is a library|is a library    |is a library|"
    "+-----------------+------------+----------------+------------+"
);

#[test]
fn does_not_panic_when_dimension_width_is_less_than_content_width() {
    // Regression test for https://github.com/zhiburt/tabled/issues/567
    //
    // A custom `Dimension` implementation is allowed to report a column
    // width smaller than the actual content width (e.g. a heuristic that
    // estimates width ahead of a resize). `calculate_indent` used to do an
    // unchecked `available - width` subtraction, which panics with
    // "attempt to subtract with overflow" in that case.
    let data = [["hello world"]];
    let data = data
        .iter()
        .map(|row| row.iter().map(Text::new).collect())
        .collect();

    let records = VecRecords::new(data);
    let cfg = SpannedConfig::default();

    let dims = Dims {
        width: vec![1],
        height: vec![1],
    };

    let _ = PeekableGrid::new(&records, &cfg, &dims, NoColors).to_string();
}

#[test]
fn does_not_panic_when_dimension_width_is_less_than_content_width_not_spanned_path() {
    // Regression test for https://github.com/zhiburt/tabled/issues/567
    //
    // Same root cause as `does_not_panic_when_dimension_width_is_less_than_content_width`,
    // but forces the `grid_not_spanned` code path (the exact module/line the
    // issue's backtrace pointed at) by setting a justification, which makes
    // `is_basic` false in `print_grid`.
    let data = [["hello world"]];
    let data = data
        .iter()
        .map(|row| row.iter().map(Text::new).collect())
        .collect();

    let records = VecRecords::new(data);
    let mut cfg = SpannedConfig::default();
    cfg.set_justification((0, 0).into(), '.');

    let dims = Dims {
        width: vec![1],
        height: vec![1],
    };

    let _ = PeekableGrid::new(&records, &cfg, &dims, NoColors).to_string();
}

#[test]
fn does_not_panic_when_dimension_width_is_less_than_content_width_spanned_path() {
    // Regression test for https://github.com/zhiburt/tabled/issues/567
    //
    // Same root cause, forcing the `grid_spanned` code path by setting a
    // column span so `has_column_spans()` is true.
    let data = [["hello world", "b"]];
    let data = data
        .iter()
        .map(|row| row.iter().map(Text::new).collect())
        .collect();

    let records = VecRecords::new(data);
    let mut cfg = SpannedConfig::default();
    cfg.set_column_span((0, 0).into(), 2);

    let dims = Dims {
        width: vec![1, 1],
        height: vec![1],
    };

    let _ = PeekableGrid::new(&records, &cfg, &dims, NoColors).to_string();
}
