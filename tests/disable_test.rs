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

use papergrid::Alignment;
use tabled::{table, Disable, Full, HorizontalAlignment, Style, Tabled};

#[derive(Tabled)]
struct Linux {
    id: u8,
    destribution: &'static str,
    link: &'static str,
}

#[test]
fn disable_rows() {
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
        "id|destribution|link                    \n",
        "--+------------+------------------------\n",
        "3 |Endeavouros |https://endeavouros.com/\n",
    );

    let table = table!(
        &data,
        Style::Psql,
        HorizontalAlignment(Full, Alignment::Left),
        Disable::Row(1..=2),
    );

    assert_eq!(table, expected);
}

#[test]
fn disable_header() {
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
        "0|Fedora     |https://getfedora.org/   \n",
        "2|OpenSUSE   |https://www.opensuse.org/\n",
        "3|Endeavouros|https://endeavouros.com/ \n",
    );

    let table = table!(
        &data,
        Style::Psql,
        HorizontalAlignment(Full, Alignment::Left),
        Disable::Row(..1),
    );

    assert_eq!(table, expected);
}

#[test]
fn disable_all_table_via_rows() {
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

    let expected = "";

    let table = table!(
        &data,
        Style::Psql,
        HorizontalAlignment(Full, Alignment::Left),
        Disable::Row(..),
    );

    assert_eq!(table, expected);
}

#[test]
fn disable_header_with_new_styling() {
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
        "┌─┬───────────┬─────────────────────────┐\n",
        "│0│Fedora     │https://getfedora.org/   │\n",
        "├─┼───────────┼─────────────────────────┤\n",
        "│2│OpenSUSE   │https://www.opensuse.org/│\n",
        "│3│Endeavouros│https://endeavouros.com/ │\n",
        "└─┴───────────┴─────────────────────────┘\n",
    );

    let table = table!(
        &data,
        HorizontalAlignment(Full, Alignment::Left),
        Disable::Row(..1),
        Style::PseudoClean,
    );

    assert_eq!(table, expected);
}

#[test]
fn disable_columns() {
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
        "destribution|link                     \n",
        "------------+-------------------------\n",
        "Fedora      |https://getfedora.org/   \n",
        "OpenSUSE    |https://www.opensuse.org/\n",
        "Endeavouros |https://endeavouros.com/ \n",
    );

    let table = table!(
        &data,
        Style::Psql,
        HorizontalAlignment(Full, Alignment::Left),
        Disable::Column(..1),
    );

    assert_eq!(table, expected);
}

#[test]
fn disable_all_table_via_columns() {
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

    let expected = "";

    let table = table!(
        &data,
        Style::Psql,
        HorizontalAlignment(Full, Alignment::Left),
        Disable::Column(..),
    );

    assert_eq!(table, expected);
}
