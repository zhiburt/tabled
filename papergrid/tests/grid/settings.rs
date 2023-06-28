#![cfg(feature = "std")]

use papergrid::color::AnsiColor;
use papergrid::config::{AlignmentHorizontal, Border, Borders, Entity, Indent, Sides};

use crate::util::grid;
use testing_table::test_table;

test_table!(
    override_by_global_alignment_0,
    grid(2, 2)
        .data([["xxxxx", "xx"], ["y", "yyyyyyyyyy"]])
        .config(|cfg| cfg.set_alignment_horizontal(Entity::Cell(0, 1), AlignmentHorizontal::Right))
        .build(),
    "+-----+----------+"
    "|xxxxx|        xx|"
    "+-----+----------+"
    "|y    |yyyyyyyyyy|"
    "+-----+----------+"
);

test_table!(
    override_by_global_alignment_1,
    grid(2, 2)
        .data([["xxxxx", "xx"], ["y", "yyyyyyyyyy"]])
        .config(|cfg| cfg.set_alignment_horizontal(Entity::Global, AlignmentHorizontal::Center))
        .build(),
    "+-----+----------+"
    "|xxxxx|    xx    |"
    "+-----+----------+"
    "|  y  |yyyyyyyyyy|"
    "+-----+----------+"
);

test_table!(
    remove_border_test,
    grid(2, 2)
        .config(|cfg| {
            cfg.set_borders(Borders::default());
            cfg.set_border(
                (0, 0),
                Border {
                    top: Some('x'),
                    bottom: Some('o'),
                    left: Some('q'),
                    ..Default::default()
                },
            );

            cfg.remove_border((0, 0), (2, 2));
        })
        .build(),
    "0-00-1\n1-01-1"
);

test_table!(
    entity_row_overrides_column_intersection_0,
    grid(2, 2)
        .config(|cfg| {
            cfg.set_borders(Borders::default());
            cfg.set_padding(
                Entity::Column(0),
                Sides {
                    bottom: Indent::new(3, '$'),
                    ..Default::default()
                },
            );
        })
        .build(),
        "0-00-1"
        "$$$   "
        "$$$   "
        "$$$   "
        "1-01-1"
        "$$$   "
        "$$$   "
        "$$$   "
);

test_table!(
    entity_row_overrides_column_intersection_1,
    grid(2, 2)
        .config(|cfg| {
            cfg.set_borders(Borders::default());
            cfg.set_padding(
                Entity::Column(0),
                Sides {
                    bottom: Indent::new(3, '$'),
                    ..Default::default()
                },
            );
            cfg.set_padding(
                Entity::Row(1),
                Sides {
                    bottom: Indent::new(2, '#'),
                    ..Default::default()
                },
            );
        })
        .build(),
        "0-00-1"
        "$$$   "
        "$$$   "
        "$$$   "
        "1-01-1"
        "######"
        "######"
);

test_table!(
    entity_column_overrides_row_intersection_0,
    grid(2, 2)
        .config(|cfg| {
            cfg.set_borders(Borders::default());
            cfg.set_padding(
                Entity::Row(0),
                Sides {
                    bottom: Indent::new(3, '$'),
                    ..Default::default()
                },
            );
        })
        .build(),
    "0-00-1\n$$$$$$\n$$$$$$\n$$$$$$\n1-01-1"
);

test_table!(
    entity_column_overrides_row_intersection_1,
    grid(2, 2)
        .config(|cfg| {
            cfg.set_borders(Borders::default());
            cfg.set_padding(
                Entity::Row(0),
                Sides::new(
                    Indent::default(),
                    Indent::default(),
                    Indent::default(),
                    Indent::new(3, '$'),
                ),
            );
            cfg.set_padding(
                Entity::Column(1),
                Sides::new(
                    Indent::default(),
                    Indent::default(),
                    Indent::default(),
                    Indent::new(2, '#'),
                ),
            );
        })
        .build(),
    "0-00-1\n$$$###\n$$$###\n$$$###\n1-01-1\n   ###\n   ###"
);

test_table!(
    test_justification_char_left_alignment,
    grid(2, 2)
        .data([["Hello", "World"], ["", "Hello Hello Hello Hello Hello"]])
        .config(|cfg| cfg.set_justification(Entity::Global, '$'))
        .build(),
    "+-----+-----------------------------+"
    "|Hello|World$$$$$$$$$$$$$$$$$$$$$$$$|"
    "+-----+-----------------------------+"
    "|$$$$$|Hello Hello Hello Hello Hello|"
    "+-----+-----------------------------+"
);

test_table!(
    test_justification_char_right_alignment,
    grid(2, 2)
        .data([["Hello", "World"], ["", "Hello Hello Hello Hello Hello"]])
        .config(|cfg| {
            cfg.set_justification(Entity::Global, '$');
            cfg.set_alignment_horizontal(Entity::Global, AlignmentHorizontal::Right);
        })
        .build(),
    "+-----+-----------------------------+"
    "|Hello|$$$$$$$$$$$$$$$$$$$$$$$$World|"
    "+-----+-----------------------------+"
    "|$$$$$|Hello Hello Hello Hello Hello|"
    "+-----+-----------------------------+"
);

test_table!(
    test_justification_char_center_alignment,
    grid(2, 2)
        .data([["Hello", "World"], ["", "Hello Hello Hello Hello Hello"]])
        .config(|cfg| {
            cfg.set_justification(Entity::Global, '$');
            cfg.set_alignment_horizontal(Entity::Global, AlignmentHorizontal::Center);
        })
        .build(),
    "+-----+-----------------------------+"
    "|Hello|$$$$$$$$$$$$World$$$$$$$$$$$$|"
    "+-----+-----------------------------+"
    "|$$$$$|Hello Hello Hello Hello Hello|"
    "+-----+-----------------------------+"
);

test_table!(
    test_justification_color_left_alignment,
    grid(2, 2)
        .data([["Hello", "World"], ["", "Hello Hello Hello Hello Hello"]])
        .config(|cfg| {
            cfg.set_justification(Entity::Global, '$');
            cfg.set_justification_color(Entity::Global, Some(AnsiColor::new("\u{1b}[34m".into(), "\u{1b}[39m".into())));
        })
        .build(),
        "+-----+-----------------------------+"
        "|Hello|World\u{1b}[34m$$$$$$$$$$$$$$$$$$$$$$$$\u{1b}[39m|"
        "+-----+-----------------------------+"
        "|\u{1b}[34m$$$$$\u{1b}[39m|Hello Hello Hello Hello Hello|"
        "+-----+-----------------------------+"
);

test_table!(
    test_justification_color_right_alignment,
    grid(2, 2)
        .data([["Hello", "World"], ["", "Hello Hello Hello Hello Hello"]])
        .config(|cfg| {
            cfg.set_justification(Entity::Global, '$');
            cfg.set_justification_color(Entity::Global, Some(AnsiColor::new("\u{1b}[34m".into(), "\u{1b}[39m".into())));
            cfg.set_alignment_horizontal(Entity::Global, AlignmentHorizontal::Right);
        })
        .build(),
        "+-----+-----------------------------+"
        "|Hello|\u{1b}[34m$$$$$$$$$$$$$$$$$$$$$$$$\u{1b}[39mWorld|"
        "+-----+-----------------------------+"
        "|\u{1b}[34m$$$$$\u{1b}[39m|Hello Hello Hello Hello Hello|"
        "+-----+-----------------------------+"
);

test_table!(
    test_justification_color_center_alignment,
    grid(2, 2)
        .data([["Hello", "World"], ["", "Hello Hello Hello Hello Hello"]])
        .config(|cfg| {
            cfg.set_justification(Entity::Global, '$');
            cfg.set_justification_color(Entity::Global, Some(AnsiColor::new("\u{1b}[34m".into(), "\u{1b}[39m".into())));
            cfg.set_alignment_horizontal(Entity::Global, AlignmentHorizontal::Center);
        })
        .build(),
    "+-----+-----------------------------+"
    "|Hello|\u{1b}[34m$$$$$$$$$$$$\u{1b}[39mWorld\u{1b}[34m$$$$$$$$$$$$\u{1b}[39m|"
    "+-----+-----------------------------+"
    "|\u{1b}[34m$$\u{1b}[39m\u{1b}[34m$$$\u{1b}[39m|Hello Hello Hello Hello Hello|"
    "+-----+-----------------------------+"
);

test_table!(
    test_justification_color_center_alignment_entity,
    grid(2, 2)
        .data([["Hello", "World"], ["", "Hello Hello Hello Hello Hello"]])
        .config(|cfg| {
            cfg.set_justification(Entity::Cell(0, 0), '$');
            cfg.set_justification_color(Entity::Column(1), Some(AnsiColor::new("\u{1b}[34m".into(), "\u{1b}[39m".into())));
            cfg.set_alignment_horizontal(Entity::Row(2), AlignmentHorizontal::Center);
        })
        .build(),
    "+-----+-----------------------------+"
    "|Hello|World\u{1b}[34m                        \u{1b}[39m|"
    "+-----+-----------------------------+"
    "|     |Hello Hello Hello Hello Hello|"
    "+-----+-----------------------------+"
);
