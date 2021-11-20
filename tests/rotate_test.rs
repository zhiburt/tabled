// todo: add method for SPACING between cells.
//       add MARGIN && PADDING instead of indent?
use tabled::{Full, Indent, Modify, Rotate, Style, Table, Tabled};

#[test]
fn test_rotate() {
    let table = || Table::new([(123, 456, 789), (234, 567, 891)]);

    assert_eq!(
        table()
            .with(Rotate::Left)
            .with(Rotate::Left)
            .with(Rotate::Left)
            .with(Rotate::Left)
            .to_string(),
        table().to_string()
    );
    assert_eq!(
        table()
            .with(Rotate::Right)
            .with(Rotate::Right)
            .with(Rotate::Right)
            .with(Rotate::Right)
            .to_string(),
        table().to_string()
    );
    assert_eq!(
        table().with(Rotate::Right).with(Rotate::Left).to_string(),
        table().to_string()
    );
    assert_eq!(
        table().with(Rotate::Left).with(Rotate::Right).to_string(),
        table().to_string()
    );
    assert_eq!(
        table().with(Rotate::Bottom).with(Rotate::Top).to_string(),
        table().to_string()
    );
    assert_eq!(
        table()
            .with(Rotate::Bottom)
            .with(Rotate::Bottom)
            .to_string(),
        table().to_string()
    );
    assert_eq!(
        table().with(Rotate::Top).with(Rotate::Top).to_string(),
        table().to_string()
    );
}

#[test]
fn test_3x3_box() {
    let table = Table::new([(123, 456, 789), (234, 567, 891)]);

    let table = table.with(Rotate::Left);
    assert_eq!(
        table.to_string(),
        "+-----+-----+-----+\n\
         | i32 | 789 | 891 |\n\
         +-----+-----+-----+\n\
         | i32 | 456 | 567 |\n\
         +-----+-----+-----+\n\
         | i32 | 123 | 234 |\n\
         +-----+-----+-----+\n"
    );

    let table = table.with(Rotate::Right).with(Rotate::Right);
    assert_eq!(
        table.to_string(),
        "+-----+-----+-----+\n\
         | 234 | 123 | i32 |\n\
         +-----+-----+-----+\n\
         | 567 | 456 | i32 |\n\
         +-----+-----+-----+\n\
         | 891 | 789 | i32 |\n\
         +-----+-----+-----+\n"
    );
}

#[test]
fn test_left_rotate() {
    let table = Table::new([(123, 456, 789), (234, 567, 891), (111, 222, 333)]);

    let table = table.with(Rotate::Left);
    assert_eq!(
        table.to_string(),
        "+-----+-----+-----+-----+\n\
         | i32 | 789 | 891 | 333 |\n\
         +-----+-----+-----+-----+\n\
         | i32 | 456 | 567 | 222 |\n\
         +-----+-----+-----+-----+\n\
         | i32 | 123 | 234 | 111 |\n\
         +-----+-----+-----+-----+\n"
    );
}

#[test]
fn test_right_rotate() {
    let table = Table::new([(123, 456, 789), (234, 567, 891), (111, 222, 333)]);

    let table = table.with(Rotate::Right);
    assert_eq!(
        table.to_string(),
        "+-----+-----+-----+-----+\n\
         | 111 | 234 | 123 | i32 |\n\
         +-----+-----+-----+-----+\n\
         | 222 | 567 | 456 | i32 |\n\
         +-----+-----+-----+-----+\n\
         | 333 | 891 | 789 | i32 |\n\
         +-----+-----+-----+-----+\n"
    );
}

#[test]
fn test_bottom_rotate() {
    let table = Table::new([(123, 456, 789), (234, 567, 891), (111, 222, 333)]);

    let table = table.with(Rotate::Bottom);
    assert_eq!(
        table.to_string(),
        "+-----+-----+-----+\n\
         | 111 | 222 | 333 |\n\
         +-----+-----+-----+\n\
         | 234 | 567 | 891 |\n\
         +-----+-----+-----+\n\
         | 123 | 456 | 789 |\n\
         +-----+-----+-----+\n\
         | i32 | i32 | i32 |\n\
         +-----+-----+-----+\n"
    );
}

#[test]
fn test_top_rotate() {
    let table = Table::new([(123, 456, 789), (234, 567, 891), (111, 222, 333)]);

    let table = table.with(Rotate::Top);
    assert_eq!(
        table.to_string(),
        "+-----+-----+-----+\n\
         | 111 | 222 | 333 |\n\
         +-----+-----+-----+\n\
         | 234 | 567 | 891 |\n\
         +-----+-----+-----+\n\
         | 123 | 456 | 789 |\n\
         +-----+-----+-----+\n\
         | i32 | i32 | i32 |\n\
         +-----+-----+-----+\n"
    );
}

#[test]
fn rotate_left_test() {
    #[derive(Tabled)]
    struct Linux {
        id: u8,
        destribution: &'static str,
        link: &'static str,
    }

    let data = vec![
        Linux {
            id: 0,
            destribution: "Fedora",
            link: "https://getfedora.org/",
        },
        Linux {
            id: 2,
            destribution: "OpenSUSE",
            link: "https://www.opensuse.org/",
        },
        Linux {
            id: 3,
            destribution: "Endeavouros",
            link: "https://endeavouros.com/",
        },
    ];

    let table = Table::new(&data)
        .with(Rotate::Left)
        .with(Style::noborder())
        .with(Modify::new(Full).with(Indent::new(1, 1, 0, 0)))
        .to_string();

    assert_eq!(
        table,
        concat!(
            "     link       https://getfedora.org/   https://www.opensuse.org/   https://endeavouros.com/ \n",
            " destribution           Fedora                   OpenSUSE                  Endeavouros        \n",
            "      id                  0                          2                          3             \n"
        ),
    );
}
