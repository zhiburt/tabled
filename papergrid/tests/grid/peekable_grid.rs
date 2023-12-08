#![cfg(feature = "std")]

use papergrid::{
    colors::NoColors,
    config::{
        spanned::SpannedConfig, AlignmentHorizontal, AlignmentVertical, Borders, Entity, Indent,
        Sides,
    },
    dimension::{spanned::SpannedGridDimension, Dimension},
    grid::peekable::PeekableGrid,
    records::vec_records::{CellInfo, VecRecords},
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
            .map(|row| row.iter().map(CellInfo::new).collect())
            .collect();

        let records = VecRecords::new(data);

        let dims = Dims {
            width: SpannedGridDimension::width(&records, &cfg),
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
            .map(|row| row.iter().map(CellInfo::new).collect())
            .collect();

        let records = VecRecords::new(data);

        let dims = Dims {
            width: SpannedGridDimension::width(&records, &cfg),
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
