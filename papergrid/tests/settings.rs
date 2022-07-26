use papergrid::{AlignmentHorizontal, Border, Borders, Entity, Indent, Padding};

mod util;

#[test]
fn override_by_global_alignment() {
    let mut grid = util::grid_with_data::<2, 2>(&[
        ((0, 0), "xxxxx"),
        ((0, 1), "xx"),
        ((1, 0), "y"),
        ((1, 1), "yyyyyyyyyy"),
    ]);

    grid.set_alignment_horizontal(Entity::Cell(0, 1), AlignmentHorizontal::Right);

    assert_eq!(
        grid.to_string(),
        "+-----+----------+\n\
         |xxxxx|        xx|\n\
         +-----+----------+\n\
         |y    |yyyyyyyyyy|\n\
         +-----+----------+",
    );

    grid.set_alignment_horizontal(Entity::Global, AlignmentHorizontal::Center);

    assert_eq!(
        grid.to_string(),
        "+-----+----------+\n\
         |xxxxx|    xx    |\n\
         +-----+----------+\n\
         |  y  |yyyyyyyyyy|\n\
         +-----+----------+",
    );
}

#[test]
fn remove_border_test() {
    let mut grid = util::grid::<2, 2>();
    grid.set_borders(Borders::default());

    grid.set_border(
        Entity::Cell(0, 0),
        Border {
            top: Some('x'),
            bottom: Some('o'),
            left: Some('q'),
            ..Default::default()
        },
    );

    grid.remove_border(Entity::Cell(0, 0));

    assert_eq!(grid.to_string(), "0-00-1\n1-01-1");
}

#[test]
fn entity_row_overrides_column_intersection() {
    let mut grid = util::grid::<2, 2>();
    grid.set_borders(Borders::default());

    grid.set_padding(
        Entity::Column(0),
        Padding {
            bottom: Indent::new(3, '$'),
            ..Default::default()
        },
    );

    assert_eq!(
        grid.to_string(),
        "0-00-1\n$$$   \n$$$   \n$$$   \n1-01-1\n$$$   \n$$$   \n$$$   "
    );

    grid.set_padding(
        Entity::Row(1),
        Padding {
            bottom: Indent::new(2, '#'),
            ..Default::default()
        },
    );

    assert_eq!(
        grid.to_string(),
        "0-00-1\n$$$   \n$$$   \n$$$   \n1-01-1\n######\n######"
    );
}

#[test]
fn entity_column_overrides_row_intersection() {
    let mut grid = util::grid::<2, 2>();
    grid.set_borders(Borders::default());

    grid.set_padding(
        Entity::Row(0),
        Padding {
            bottom: Indent::new(3, '$'),
            ..Default::default()
        },
    );

    assert_eq!(grid.to_string(), "0-00-1\n$$$$$$\n$$$$$$\n$$$$$$\n1-01-1");

    grid.set_padding(
        Entity::Column(1),
        Padding {
            bottom: Indent::new(2, '#'),
            ..Default::default()
        },
    );

    assert_eq!(
        grid.to_string(),
        "0-00-1\n$$$###\n$$$###\n$$$###\n1-01-1\n   ###\n   ###"
    );
}
