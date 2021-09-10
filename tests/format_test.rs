use tabled::{
    multiline, Cell, Column, Format, FormatFrom, FormatWithIndex, Full, Head, Indent, Modify,
    Object, Row, Style, Table, Tabled,
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
        "+------+----------------+-----------------------------+\n",
        "| [id] | [destribution] |           [link]            |\n",
        "+------+----------------+-----------------------------+\n",
        "| [0]  |    [Fedora]    |  [https://getfedora.org/]   |\n",
        "+------+----------------+-----------------------------+\n",
        "| [2]  |   [OpenSUSE]   | [https://www.opensuse.org/] |\n",
        "+------+----------------+-----------------------------+\n",
        "| [3]  | [Endeavouros]  | [https://endeavouros.com/]  |\n",
        "+------+----------------+-----------------------------+\n",
    );

    let table = Table::new(&data)
        .with(Modify::new(Full).with(Format(|s| format!("[{}]", s))))
        .to_string();

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

    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Modify::new(Head).with(Format(|s| format!(":{}", s))))
        .to_string();

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

    let table = Table::new(&data)
        .with(Style::psql())
        .with(Modify::new(Row(1..)).with(Format(|s| format!("<{}>", s))))
        .to_string();

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

    let table = Table::new(&data)
        .with(Style::psql())
        .with(Modify::new(Column(..1)).with(Format(|s| format!("(x) {}", s))))
        .to_string();

    assert_eq!(table, expected);
}

#[test]
fn formatting_multiline_test() {
    let data = vec![
        Linux {
            id: 0,
            destribution: "Fedora",
            link: "https://getfedora.org/",
        },
        Linux {
            id: 1,
            destribution: "Open\nSUSE",
            link: "https\n://\nwww.opensuse.org/",
        },
        Linux {
            id: 2,
            destribution: "Endeavouros",
            link: "https://endeavouros.com/",
        },
        Linux {
            id: 3,
            destribution: "Red\nHat\nEnterprise",
            link: "https://redhat.com/",
        },
    ];

    let expected = concat!(
        " (x) id | (x) destribution |           (x) link           \n",
        "--------+------------------+------------------------------\n",
        " (x) 0  |    (x) Fedora    |  (x) https://getfedora.org/  \n",
        " (x) 1  |     (x) Open     |          (x) https           \n",
        "        |     (x) SUSE     |           (x) ://            \n",
        "        |                  |    (x) www.opensuse.org/     \n",
        " (x) 2  | (x) Endeavouros  | (x) https://endeavouros.com/ \n",
        " (x) 3  |     (x) Red      |   (x) https://redhat.com/    \n",
        "        |     (x) Hat      |                              \n",
        "        |  (x) Enterprise  |                              \n",
    );

    let table = Table::new(&data)
        .with(Style::psql())
        .with(Modify::new(Full).with(Format(multiline(|s| format!("(x) {}", s)))))
        .to_string();

    assert_eq!(table, expected);
}

#[test]
fn formatting_cell_test() {
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
        " (x) id | (x) destribution |         (x) link          \n",
        "--------+------------------+---------------------------\n",
        "   0    |      Fedora      |  https://getfedora.org/   \n",
        "   2    |     OpenSUSE     | https://www.opensuse.org/ \n",
        "   3    |   Endeavouros    | https://endeavouros.com/  \n",
    );

    let table = Table::new(&data)
        .with(Style::psql())
        .with(Modify::new(Cell(0, 0)).with(Format(|s| format!("(x) {}", s))))
        .with(Modify::new(Cell(0, 1)).with(Format(|s| format!("(x) {}", s))))
        .with(Modify::new(Cell(0, 2)).with(Format(|s| format!("(x) {}", s))))
        .to_string();

    assert_eq!(table, expected);
}

#[test]
fn formatting_and_combination_test() {
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
        " (x) id | (x) destribution |         (x) link          \n",
        "--------+------------------+---------------------------\n",
        " (x) 0  |      Fedora      |  https://getfedora.org/   \n",
        " (x) 2  |     OpenSUSE     | https://www.opensuse.org/ \n",
        " (x) 3  |   Endeavouros    | https://endeavouros.com/  \n",
    );

    let table = Table::new(&data)
        .with(Style::psql())
        .with(Modify::new(Column(..1).and(Row(..1))).with(Format(|s| format!("(x) {}", s))))
        .to_string();

    assert_eq!(table, expected);
}

#[test]
fn formatting_not_combination_test() {
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
        "  id   | (x) destribution |         (x) link          \n",
        "-------+------------------+---------------------------\n",
        " (x) 0 |      Fedora      |  https://getfedora.org/   \n",
        " (x) 2 |     OpenSUSE     | https://www.opensuse.org/ \n",
        " (x) 3 |   Endeavouros    | https://endeavouros.com/  \n",
    );

    let table = Table::new(&data)
        .with(Style::psql())
        .with(
            Modify::new(Column(..1).and(Row(..1)).not(Cell(0, 0)))
                .with(Format(|s| format!("(x) {}", s))),
        )
        .to_string();

    assert_eq!(table, expected);
}

#[test]
fn formatting_using_lambda_test() {
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

    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Modify::new(Head).with(|s: &str| format!(":{}", s)))
        .to_string();

    assert_eq!(table, expected);
}

#[test]
fn formatting_using_function_test() {
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
        "| ID | DESTRIBUTION |           LINK            |\n",
        "|----+--------------+---------------------------|\n",
        "| 0  |    Fedora    |  https://getfedora.org/   |\n",
        "| 2  |   OpenSUSE   | https://www.opensuse.org/ |\n",
        "| 3  | Endeavouros  | https://endeavouros.com/  |\n",
    );

    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Modify::new(Head).with(str::to_uppercase))
        .to_string();

    assert_eq!(table, expected);
}

#[test]
fn format_from() {
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
        .with(Style::github_markdown())
        .with(Modify::new(Head).with(FormatFrom(vec![
            "Header Name 1",
            "Header Name 2",
            "Header Name 3",
        ])))
        .to_string();

    let expected = concat!(
        "| Header Name 1 | Header Name 2 |       Header Name 3       |\n",
        "|---------------+---------------+---------------------------|\n",
        "|       0       |    Fedora     |  https://getfedora.org/   |\n",
        "|       2       |   OpenSUSE    | https://www.opensuse.org/ |\n",
        "|       3       |  Endeavouros  | https://endeavouros.com/  |\n",
    );

    assert_eq!(table, expected);
}

#[test]
fn format_with_index() {
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
        .with(Style::github_markdown())
        .with(
            Modify::new(Head).with(FormatWithIndex(|a, b, c| match (b, c) {
                (0, 0) => "(0, 0)".to_string(),
                (0, 1) => "(0, 1)".to_string(),
                (0, 2) => "(0, 2)".to_string(),
                _ => a.to_string(),
            })),
        )
        .to_string();

    let expected = concat!(
        "| (0, 0) |   (0, 1)    |          (0, 2)           |\n",
        "|--------+-------------+---------------------------|\n",
        "|   0    |   Fedora    |  https://getfedora.org/   |\n",
        "|   2    |  OpenSUSE   | https://www.opensuse.org/ |\n",
        "|   3    | Endeavouros | https://endeavouros.com/  |\n",
    );

    assert_eq!(table, expected);
}

#[cfg(feature = "color")]
mod color {

    use super::*;
    use owo_colors::OwoColorize;

    #[test]
    fn color_column_test() {
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
            " \u{1b}[31mid\u{1b}[0m | \u{1b}[34mdestribution\u{1b}[0m |           \u{1b}[31mlink\u{1b}[0m            \n",
            "----+--------------+---------------------------\n",
            " \u{1b}[31m0\u{1b}[0m  |    \u{1b}[34mFedora\u{1b}[0m    |  \u{1b}[31mhttps://getfedora.org/\u{1b}[0m   \n",
            " \u{1b}[31m2\u{1b}[0m  |   \u{1b}[34mOpenSUSE\u{1b}[0m   | \u{1b}[31mhttps://www.opensuse.org/\u{1b}[0m \n",
            " \u{1b}[31m3\u{1b}[0m  | \u{1b}[34mEndeavouros\u{1b}[0m  | \u{1b}[31mhttps://endeavouros.com/\u{1b}[0m  \n",
        );

        let table = Table::new(&data)
            .with(Style::psql())
            .with(Modify::new(Column(..1).and(Column(2..))).with(Format(|s| s.red().to_string())))
            .with(Modify::new(Column(1..2)).with(Format(|s| s.blue().to_string())))
            .to_string();

        println!("{}", table);

        assert_eq!(table, expected);
    }

    #[test]
    fn color_multiline_test() {
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
                destribution: "Red\nHat\nEnterprise",
                link: "https://redhat.com/",
            },
        ];

        let expected = concat!(
            " \u{1b}[31mid\u{1b}[0m | \u{1b}[34mdestribution\u{1b}[0m |           \u{1b}[32mlink\u{1b}[0m            \n",
            "----+--------------+---------------------------\n",
            " \u{1b}[31m0\u{1b}[0m  |    \u{1b}[34mFedora\u{1b}[0m    |  \u{1b}[32mhttps://getfedora.org/\u{1b}[0m   \n",
            " \u{1b}[31m2\u{1b}[0m  |   \u{1b}[34mOpenSUSE\u{1b}[0m   | \u{1b}[32mhttps://www.opensuse.org/\u{1b}[0m \n",
            " \u{1b}[31m3\u{1b}[0m  | \u{1b}[34mEndeavouros\u{1b}[0m  | \u{1b}[32mhttps://endeavouros.com/\u{1b}[0m  \n",
            " \u{1b}[31m4\u{1b}[0m  |     \u{1b}[34mRed\u{1b}[0m      |    \u{1b}[32mhttps://redhat.com/\u{1b}[0m    \n",
            "    |     \u{1b}[34mHat\u{1b}[0m      |                           \n",
            "    |  \u{1b}[34mEnterprise\u{1b}[0m  |                           \n",
        );

        let table = Table::new(&data)
            .with(Style::psql())
            .with(Modify::new(Column(..1)).with(Format(multiline(|s| s.red().to_string()))))
            .with(Modify::new(Column(1..2)).with(Format(multiline(|s| s.blue().to_string()))))
            .with(Modify::new(Column(2..)).with(Format(multiline(|s| s.green().to_string()))))
            .to_string();

        println!("{}", table);

        assert_eq!(table, expected);
    }
}

#[test]
fn format_doesnt_change_indent() {
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
        "+--------+------------------+-------------------------------+\n",
        "|   [id] |   [destribution] |   [link]                      |\n",
        "+--------+------------------+-------------------------------+\n",
        "|   [0]  |   [Fedora]       |   [https://getfedora.org/]    |\n",
        "+--------+------------------+-------------------------------+\n",
        "|   [2]  |   [OpenSUSE]     |   [https://www.opensuse.org/] |\n",
        "+--------+------------------+-------------------------------+\n",
        "|   [3]  |   [Endeavouros]  |   [https://endeavouros.com/]  |\n",
        "+--------+------------------+-------------------------------+\n",
    );

    let table = Table::new(&data)
        .with(Modify::new(Full).with(Indent::new(3, 1, 0, 0)))
        .with(Modify::new(Full).with(Format(|s| format!("[{}]", s))))
        .to_string();

    assert_eq!(table, expected);
}
