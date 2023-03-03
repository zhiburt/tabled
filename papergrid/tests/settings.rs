#![cfg(feature = "std")]

mod util;

use papergrid::config::{AlignmentHorizontal, Border, Borders, Entity, Indent, Sides};

use util::{grid, test_table};

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
                Sides {
                    bottom: Indent::new(3, '$'),
                    ..Default::default()
                },
            );
            cfg.set_padding(
                Entity::Column(1),
                Sides {
                    bottom: Indent::new(2, '#'),
                    ..Default::default()
                },
            );
        })
        .build(),
    "0-00-1\n$$$###\n$$$###\n$$$###\n1-01-1\n   ###\n   ###"
);
