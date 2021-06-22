use tabled::{Alignment, Full, Indent, Modify, Row, Style, Table, Tabled};

#[derive(Tabled)]
struct Linux {
    id: u8,
    destribution: &'static str,
    link: &'static str,
}

#[test]
fn indent() {
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

    let expected = concat!(
        "id |destribution |link                       \n",
        "---+-------------+---------------------------\n",
        " 0 | Fedora      | https://getfedora.org/    \n",
        "   |             |                           \n",
        "   |             |                           \n",
        " 2 | OpenSUSE    | https://www.opensuse.org/ \n",
        "   |             |                           \n",
        "   |             |                           \n",
        " 3 | Endeavouros | https://endeavouros.com/  \n",
        "   |             |                           \n",
        "   |             |                           \n",
    );

    let table = Table::new(&data)
        .with(Style::psql())
        .with(Modify::new(Full).with(Alignment::left()))
        .with(Modify::new(Row(1..)).with(Indent::new(1, 1, 0, 2)))
        .to_string();

    assert_eq!(table, expected);
}

#[test]
fn indent_multiline() {
    let data = vec![
        Linux {
            id: 0,
            destribution: "Fedora",
            link: "https://getfedora.org/",
        },
        Linux {
            id: 2,
            destribution: "Open\nSUSE",
            link: "https://www.\nopensuse\n.org/",
        },
        Linux {
            id: 3,
            destribution: "Endeavouros",
            link: "https://endeavouros.com/",
        },
    ];

    let expected = concat!(
        " id | destribution |           link           \n",
        "----+--------------+--------------------------\n",
        "    |              |                          \n",
        " 0  | Fedora       | https://getfedora.org/   \n",
        "    |              |                          \n",
        "    |              |                          \n",
        " 2  | Open         | https://www.             \n",
        "    | SUSE         | opensuse                 \n",
        "    |              | .org/                    \n",
        "    |              |                          \n",
        "    |              |                          \n",
        " 3  | Endeavouros  | https://endeavouros.com/ \n",
        "    |              |                          \n",
    );

    let table = Table::new(&data)
        .with(Style::psql())
        .with(Modify::new(Row(1..)).with(Indent::new(1, 1, 1, 1)))
        .to_string();

    assert_eq!(table, expected);
}

#[test]
fn indent_multiline_with_vertical_alignment() {
    let data = vec![
        Linux {
            id: 0,
            destribution: "Fedora",
            link: "https://getfedora.org/",
        },
        Linux {
            id: 2,
            destribution: "Open\nSUSE",
            link: "https://www.\nopensuse\n.org/",
        },
        Linux {
            id: 3,
            destribution: "Endeavouros",
            link: "https://endeavouros.com/",
        },
    ];

    let expected = concat!(
        "id |destribution |           link           \n",
        "---+-------------+--------------------------\n",
        "   |             |                          \n",
        " 0 |   Fedora    |  https://getfedora.org/  \n",
        "   |             |                          \n",
        "   |             |                          \n",
        "   |    Open     |       https://www.       \n",
        " 2 |    SUSE     |         opensuse         \n",
        "   |             |          .org/           \n",
        "   |             |                          \n",
        "   |             |                          \n",
        " 3 | Endeavouros | https://endeavouros.com/ \n",
        "   |             |                          \n",
    );

    let table = Table::new(&data)
        .with(Style::psql())
        .with(
            Modify::new(Full)
                .with(Alignment::center_horizontal())
                .with(Alignment::center_vertical()),
        )
        .with(Modify::new(Row(1..)).with(Indent::new(1, 1, 1, 1)))
        .to_string();

    println!("{}", table);

    assert_eq!(table, expected);
}
