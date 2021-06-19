// Copyright (c) 2021 Maxim Zhiburt
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

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
