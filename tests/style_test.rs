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

use tabled::{table, Tabled};

#[derive(Tabled)]
struct Linux {
    id: u8,
    destribution: &'static str,
    link: &'static str,
}

#[test]
fn default_style() {
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
        "+----+--------------+---------------------------+\n",
        "| id | destribution |           link            |\n",
        "+----+--------------+---------------------------+\n",
        "| 0  |    Fedora    |  https://getfedora.org/   |\n",
        "+----+--------------+---------------------------+\n",
        "| 2  |   OpenSUSE   | https://www.opensuse.org/ |\n",
        "+----+--------------+---------------------------+\n",
        "| 3  | Endeavouros  | https://endeavouros.com/  |\n",
        "+----+--------------+---------------------------+\n",
    );

    let table = table!(&data, tabled::Style::Default);

    assert_eq!(table, expected);
}

#[test]
fn psql_style() {
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
        " id | destribution |           link            \n",
        "----+--------------+---------------------------\n",
        " 0  |    Fedora    |  https://getfedora.org/   \n",
        " 2  |   OpenSUSE   | https://www.opensuse.org/ \n",
        " 3  | Endeavouros  | https://endeavouros.com/  \n",
    );

    let table = table!(&data, tabled::Style::Psql);

    assert_eq!(table, expected);
}

#[test]
fn github_markdown_style() {
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
        "| id | destribution |           link            |\n",
        "|----+--------------+---------------------------|\n",
        "| 0  |    Fedora    |  https://getfedora.org/   |\n",
        "| 2  |   OpenSUSE   | https://www.opensuse.org/ |\n",
        "| 3  | Endeavouros  | https://endeavouros.com/  |\n",
    );

    let table = table!(&data, tabled::Style::GithubMarkdown);

    assert_eq!(table, expected);
}

#[test]
fn pseudo_style() {
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
        "┌────┬──────────────┬───────────────────────────┐\n",
        "│ id │ destribution │           link            │\n",
        "├────┼──────────────┼───────────────────────────┤\n",
        "│ 0  │    Fedora    │  https://getfedora.org/   │\n",
        "├────┼──────────────┼───────────────────────────┤\n",
        "│ 2  │   OpenSUSE   │ https://www.opensuse.org/ │\n",
        "├────┼──────────────┼───────────────────────────┤\n",
        "│ 3  │ Endeavouros  │ https://endeavouros.com/  │\n",
        "└────┴──────────────┴───────────────────────────┘\n",
    );

    let table = table!(&data, tabled::Style::Pseudo);

    assert_eq!(table, expected);
}

#[test]
fn pseudo_clean_style() {
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
        "┌────┬──────────────┬───────────────────────────┐\n",
        "│ id │ destribution │           link            │\n",
        "├────┼──────────────┼───────────────────────────┤\n",
        "│ 0  │    Fedora    │  https://getfedora.org/   │\n",
        "│ 2  │   OpenSUSE   │ https://www.opensuse.org/ │\n",
        "│ 3  │ Endeavouros  │ https://endeavouros.com/  │\n",
        "└────┴──────────────┴───────────────────────────┘\n",
    );

    let table = table!(&data, tabled::Style::PseudoClean);

    assert_eq!(table, expected);
}

#[test]
fn noborder_style() {
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
        " id   destribution             link            \n",
        " 0       Fedora       https://getfedora.org/   \n",
        " 2      OpenSUSE     https://www.opensuse.org/ \n",
        " 3    Endeavouros    https://endeavouros.com/  \n",
    );

    let table = table!(&data, tabled::Style::NoBorder);

    assert_eq!(table, expected);
}
