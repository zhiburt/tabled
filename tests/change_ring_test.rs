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

use colored::Colorize;
use papergrid::Alignment;
use tabled::{
    table, AlignmentObject, ChangeRing, Column, Full, Head, HorizontalAlignment, Row, Style, Tabled,
};

#[derive(Tabled)]
struct Linux {
    id: u8,
    destribution: &'static str,
    link: &'static str,
}

#[test]
fn formatting_full_test() {
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
        "+-----+----------------+----------------------------+\n",
        "| :id | [destribution] |           :link            |\n",
        "+-----+----------------+----------------------------+\n",
        "| [0] |    :Fedora     |  [https://getfedora.org/]  |\n",
        "+-----+----------------+----------------------------+\n",
        "| :2  |   [OpenSUSE]   | :https://www.opensuse.org/ |\n",
        "+-----+----------------+----------------------------+\n",
        "| [3] |  :Endeavouros  | [https://endeavouros.com/] |\n",
        "+-----+----------------+----------------------------+\n",
    );

    let table = table!(
        &data,
        ChangeRing(
            Full,
            vec![
                Box::new(|s| { format!(":{}", s) }),
                Box::new(|s| { format!("[{}]", s) }),
            ]
        ),
    );

    assert_eq!(table, expected);
}

#[test]
fn formatting_head_test() {
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
        "| :id | :destribution |           :link           |\n",
        "|-----+---------------+---------------------------|\n",
        "|  0  |    Fedora     |  https://getfedora.org/   |\n",
        "|  2  |   OpenSUSE    | https://www.opensuse.org/ |\n",
        "|  3  |  Endeavouros  | https://endeavouros.com/  |\n",
    );

    let table = table!(
        &data,
        Style::GithubMarkdown,
        ChangeRing(Head, vec![Box::new(|s| { format!(":{}", s) }),]),
    );

    assert_eq!(table, expected);
}

#[test]
fn formatting_row_test() {
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
        " id  | destribution  |            link             \n",
        "-----+---------------+-----------------------------\n",
        " <0> |   <Fedora>    |  <https://getfedora.org/>   \n",
        " <2> |  <OpenSUSE>   | <https://www.opensuse.org/> \n",
        " <3> | <Endeavouros> | <https://endeavouros.com/>  \n",
    );

    let table = table!(
        &data,
        Style::Psql,
        ChangeRing(Row(1..), vec![Box::new(|s| { format!("<{}>", s) }),]),
    );

    assert_eq!(table, expected);
}

#[test]
fn formatting_column_test() {
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
        " (x) id | destribution |           link            \n",
        "--------+--------------+---------------------------\n",
        " (x) 0  |    Fedora    |  https://getfedora.org/   \n",
        " (x) 2  |   OpenSUSE   | https://www.opensuse.org/ \n",
        " (x) 3  | Endeavouros  | https://endeavouros.com/  \n",
    );

    let table = table!(
        &data,
        Style::Psql,
        ChangeRing(Column(..1), vec![Box::new(|s| { format!("(x) {}", s) }),]),
    );

    assert_eq!(table, expected);
}
