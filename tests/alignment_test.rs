use tabled::{Alignment, Column, Full, Head, Modify, Row, Style, Table, Tabled};

#[derive(Tabled)]
struct Linux {
    id: u8,
    destribution: &'static str,
    link: &'static str,
}

#[test]
fn full_alignment() {
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
        "id|destribution|link                     \n",
        "--+------------+-------------------------\n",
        "0 |Fedora      |https://getfedora.org/   \n",
        "2 |OpenSUSE    |https://www.opensuse.org/\n",
        "3 |Endeavouros |https://endeavouros.com/ \n",
    );

    let table = Table::new(&data)
        .with(Style::psql())
        .with(Modify::new(Full).with(Alignment::left()))
        .to_string();

    assert_eq!(table, expected);
}

#[test]
fn head_and_data_alignment() {
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
        "+--+------------+-------------------------+\n",
        "|id|destribution|link                     |\n",
        "+--+------------+-------------------------+\n",
        "| 0|      Fedora|   https://getfedora.org/|\n",
        "+--+------------+-------------------------+\n",
        "| 2|    OpenSUSE|https://www.opensuse.org/|\n",
        "+--+------------+-------------------------+\n",
        "| 3| Endeavouros| https://endeavouros.com/|\n",
        "+--+------------+-------------------------+\n",
    );

    let table = Table::new(&data)
        .with(Style::default())
        .with(Modify::new(Head).with(Alignment::left()))
        .with(Modify::new(Row(1..)).with(Alignment::right()))
        .to_string();

    assert_eq!(table, expected);
}

#[test]
fn full_alignment_multiline() {
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
        Linux {
            id: 4,
            destribution: "Red\nHat",
            link: "https\n://\nwww\n.\nredhat\n.\ncom\n/en",
        },
    ];

    let expected = concat!(
        "id|destribution|link                     \n",
        "--+------------+-------------------------\n",
        "0 |Fedora      |https://getfedora.org/   \n",
        "2 |OpenSUSE    |https://www.opensuse.org/\n",
        "3 |Endeavouros |https://endeavouros.com/ \n",
        "4 |Red         |https                    \n",
        "  |Hat         |://                      \n",
        "  |            |www                      \n",
        "  |            |.                        \n",
        "  |            |redhat                   \n",
        "  |            |.                        \n",
        "  |            |com                      \n",
        "  |            |/en                      \n",
    );

    let table = Table::new(&data)
        .with(Style::psql())
        .with(Modify::new(Full).with(Alignment::left()))
        .to_string();

    assert_eq!(table, expected);
}

#[test]
fn vertical_alignment_test() {
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
            destribution: "E\nnde\navou\nros",
            link: "https://endeavouros.com/",
        },
        Linux {
            id: 4,
            destribution: "Red\nHat",
            link: "https\n://\nwww\n.\nredhat\n.\ncom\n/en",
        },
    ];

    let table = Table::new(&data)
        .with(Style::psql())
        .with(Modify::new(Column(1..)).with(Alignment::bottom()))
        .to_string();

    let expected = concat!(
        " id |destribution|link                     \n",
        "----+------------+-------------------------\n",
        " 0  |Fedora      |https://getfedora.org/   \n",
        " 2  |OpenSUSE    |https://www.opensuse.org/\n",
        " 3  |E           |                         \n",
        "    |nde         |                         \n",
        "    |avou        |                         \n",
        "    |ros         |https://endeavouros.com/ \n",
        " 4  |            |https                    \n",
        "    |            |://                      \n",
        "    |            |www                      \n",
        "    |            |.                        \n",
        "    |            |redhat                   \n",
        "    |            |.                        \n",
        "    |Red         |com                      \n",
        "    |Hat         |/en                      \n",
    );

    assert_eq!(table, expected);
}
