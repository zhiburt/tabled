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

use tabled::{Column, Full, MaxWidth, Modify, Object, Row, Style, Table, Tabled};

#[derive(Tabled)]
struct Linux {
    id: u8,
    destribution: &'static str,
    link: &'static str,
}

#[test]
fn max_width() {
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
        "| id | destribution |  link  |\n",
        "|----+--------------+--------|\n",
        "| 0  |    Fed...    | htt... |\n",
        "| 2  |    Ope...    | htt... |\n",
        "| 3  |    End...    | htt... |\n",
    );

    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Modify::new(Column(1..).not(Row(..1))).with(MaxWidth(3, "...")))
        .to_string();

    assert_eq!(table, expected);
}

#[test]
fn dont_change_content_if_width_is_less_then_max_width() {
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

    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Modify::new(Full).with(MaxWidth(1000, "...")))
        .to_string();

    assert_eq!(table, expected);
}

#[test]
fn max_width_with_emoji() {
    let data = &["🤠", "😳🥵🥶😱😨", "🚴🏻‍♀️🚴🏻🚴🏻‍♂️🚵🏻‍♀️🚵🏻🚵🏻‍♂️"];

    // Is it right behaiviour?
    let expected = concat!(
        "| &st... |\n",
        "|--------|\n",
        "|   🤠   |\n",
        "| 😳...  |\n",
        "| 🚴...  |\n",
    );

    let table = Table::new(data)
        .with(Style::github_markdown())
        .with(Modify::new(Full).with(MaxWidth(3, "...")))
        .to_string();

    assert_eq!(table, expected);
}

#[cfg(feature = "color")]
#[test]
fn color_chars_are_stripped() {
    use colored::Colorize;

    let data = &[
        "asd".red().to_string(),
        "zxc".blue().to_string(),
        "asdasd".on_black().green().to_string(),
    ];

    let expected = concat!(
        "| Str... |\n",
        "|--------|\n",
        "|  \u{1b}[31masd\u{1b}[0m   |\n",
        "|  \u{1b}[34mzxc\u{1b}[0m   |\n",
        "| \u{1b}[40;32masd\u{1b}[0m... |\n",
    );

    let table = Table::new(data)
        .with(Style::github_markdown())
        .with(Modify::new(Full).with(MaxWidth(3, "...")))
        .to_string();

    assert_eq!(table, expected);
}
