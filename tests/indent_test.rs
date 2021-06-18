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

use tabled::{table, Alignment, Full, Indent, Row, Style, Tabled};

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

    let table = table!(
        &data,
        Style::psql(),
        Alignment::left(Full),
        Indent::new(Row(1..), 1, 1, 0, 2)
    );

    assert_eq!(expected, table);
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

    let table = table!(&data, Style::psql(), Indent::new(Row(1..), 1, 1, 1, 1));

    assert_eq!(expected, table);
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

    let table = table!(
        &data,
        Style::psql(),
        Alignment::center_horizontal(Full),
        Alignment::center_vertical(Full),
        Indent::new(Row(1..), 1, 1, 1, 1)
    );

    println!("{}", table);

    assert_eq!(expected, table);
}
