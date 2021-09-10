use tabled::{display::ExpandedDisplay, Tabled};

#[test]
fn display() {
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

    let expected = concat!(
        "-[ RECORD 0 ]---------------------------\n",
        "id           | 0\n",
        "destribution | Fedora\n",
        "link         | https://getfedora.org/\n",
        "-[ RECORD 1 ]---------------------------\n",
        "id           | 2\n",
        "destribution | OpenSUSE\n",
        "link         | https://www.opensuse.org/\n",
        "-[ RECORD 2 ]---------------------------\n",
        "id           | 3\n",
        "destribution | Endeavouros\n",
        "link         | https://endeavouros.com/\n",
    );

    let table = ExpandedDisplay::new(&data).to_string();

    assert_eq!(table, expected);
}

#[cfg(feature = "color")]
#[test]
fn display_colored() {
    use owo_colors::{AnsiColors, OwoColorize};

    #[derive(Tabled)]
    struct Linux {
        id: u8,
        destribution: &'static str,
        link: String,
    }

    let data = vec![
        Linux {
            id: 0,
            destribution: "Fedora",
            link: "https://getfedora.org/"
                .red()
                .on_color(AnsiColors::Blue)
                .to_string(),
        },
        Linux {
            id: 2,
            destribution: "OpenSUSE",
            link: "https://www.opensuse.org/"
                .green()
                .on_color(AnsiColors::Black)
                .to_string(),
        },
        Linux {
            id: 3,
            destribution: "Endeavouros",
            link: "https://endeavouros.com/".blue().underline().to_string(),
        },
    ];

    let expected = concat!(
        "-[ RECORD 0 ]---------------------------\n",
        "id           | 0\n",
        "destribution | Fedora\n",
        "link         | \u{1b}[44m\u{1b}[31mhttps://getfedora.org/\u{1b}[0m\u{1b}[0m\n",
        "-[ RECORD 1 ]---------------------------\n",
        "id           | 2\n",
        "destribution | OpenSUSE\nlink         | \u{1b}[40m\u{1b}[32mhttps://www.opensuse.org/\u{1b}[0m\u{1b}[0m\n",
        "-[ RECORD 2 ]---------------------------\n",
        "id           | 3\ndestribution | Endeavouros\n",
        "link         | \u{1b}[4m\u{1b}[34mhttps://endeavouros.com/\u{1b}[0m\u{1b}[0m\n",
    );

    let table = ExpandedDisplay::new(&data).to_string();

    println!("{}", table);

    assert_eq!(table, expected);
}

#[test]
fn display_empty() {
    struct Type;

    impl Tabled for Type {
        fn fields(&self) -> Vec<String> {
            Vec::new()
        }

        fn headers() -> Vec<String> {
            Vec::new()
        }
    }

    let table = ExpandedDisplay::new(&[Type]).to_string();

    assert_eq!(table, "-[ RECORD 0 ]-\n");
}

#[test]
fn display_dynamic_header_template() {
    {
        struct Type;

        impl Tabled for Type {
            fn fields(&self) -> Vec<String> {
                vec!["He".to_string(), "123".to_string(), "asd".to_string()]
            }

            fn headers() -> Vec<String> {
                vec!["1".to_string(), "2".to_string(), "3".to_string()]
            }
        }

        let expected = concat!("-[ RECORD 0 ]-\n", "1 | He\n", "2 | 123\n", "3 | asd\n",);

        let table = ExpandedDisplay::new(&[Type]).to_string();

        assert_eq!(table, expected);
    }
    {
        struct Type;

        impl Tabled for Type {
            fn fields(&self) -> Vec<String> {
                vec!["He".to_string(), "123".to_string(), "asd".to_string()]
            }

            fn headers() -> Vec<String> {
                vec!["11".to_string(), "2222222".to_string(), "3".to_string()]
            }
        }

        let expected = concat!(
            "-[ RECORD 0 ]-\n",
            "11      | He\n",
            "2222222 | 123\n",
            "3       | asd\n",
        );

        let table = ExpandedDisplay::new(&[Type]).to_string();

        assert_eq!(table, expected);
    }
    {
        struct Type;

        impl Tabled for Type {
            fn fields(&self) -> Vec<String> {
                vec!["HeheHehe".to_string(), "123".to_string(), "asd".to_string()]
            }

            fn headers() -> Vec<String> {
                vec!["11".to_string(), "2222222".to_string(), "3".to_string()]
            }
        }

        let expected = concat!(
            "-[ RECORD 0 ]-----\n",
            "11      | HeheHehe\n",
            "2222222 | 123\n",
            "3       | asd\n",
        );

        let table = ExpandedDisplay::new(&[Type]).to_string();

        assert_eq!(table, expected);
    }
    {
        struct Type;

        impl Tabled for Type {
            fn fields(&self) -> Vec<String> {
                vec!["He".to_string(), "123".to_string(), "asd".to_string()]
            }

            fn headers() -> Vec<String> {
                vec!["11111111111".to_string(), "2".to_string(), "3".to_string()]
            }
        }

        let expected = concat!(
            "-[ RECORD 0 ]----\n",
            "11111111111 | He\n",
            "2           | 123\n",
            "3           | asd\n",
        );

        let table = ExpandedDisplay::new(&[Type]).to_string();

        assert_eq!(table, expected);
    }
    {
        struct Type;

        impl Tabled for Type {
            fn fields(&self) -> Vec<String> {
                vec!["He".to_string(), "123".to_string(), "asd".to_string()]
            }

            fn headers() -> Vec<String> {
                vec![
                    "1111111111111".to_string(),
                    "2".to_string(),
                    "3".to_string(),
                ]
            }
        }

        let expected = concat!(
            "-[ RECORD 0 ]-+----\n",
            "1111111111111 | He\n",
            "2             | 123\n",
            "3             | asd\n",
        );

        let table = ExpandedDisplay::new(&[Type]).to_string();

        assert_eq!(table, expected);
    }
    {
        struct Type;

        impl Tabled for Type {
            fn fields(&self) -> Vec<String> {
                vec!["He".to_string(), "123".to_string(), "asd".to_string()]
            }

            fn headers() -> Vec<String> {
                vec![
                    "11111111111111111111111111111".to_string(),
                    "2".to_string(),
                    "3".to_string(),
                ]
            }
        }

        let expected = concat!(
            "-[ RECORD 0 ]-----------------+----\n",
            "11111111111111111111111111111 | He\n",
            "2                             | 123\n",
            "3                             | asd\n",
        );

        let table = ExpandedDisplay::new(&[Type]).to_string();

        assert_eq!(table, expected);
    }
    {
        #[derive(Clone)]
        struct Type;

        impl Tabled for Type {
            fn fields(&self) -> Vec<String> {
                vec!["22".to_string()]
            }

            fn headers() -> Vec<String> {
                vec!["11111111111".to_string()]
            }
        }

        let expected = concat!(
            "-[ RECORD 0 ]---\n",
            "11111111111 | 22\n",
            "-[ RECORD 1 ]---\n",
            "11111111111 | 22\n",
            "-[ RECORD 2 ]---\n",
            "11111111111 | 22\n",
            "-[ RECORD 3 ]---\n",
            "11111111111 | 22\n",
            "-[ RECORD 4 ]---\n",
            "11111111111 | 22\n",
            "-[ RECORD 5 ]---\n",
            "11111111111 | 22\n",
            "-[ RECORD 6 ]---\n",
            "11111111111 | 22\n",
            "-[ RECORD 7 ]---\n",
            "11111111111 | 22\n",
            "-[ RECORD 8 ]---\n",
            "11111111111 | 22\n",
            "-[ RECORD 9 ]---\n",
            "11111111111 | 22\n",
            "-[ RECORD 10 ]--\n",
            "11111111111 | 22\n",
        );

        let table = ExpandedDisplay::new(std::iter::repeat(Type).take(11)).to_string();

        assert_eq!(table, expected);
    }
}

#[test]
fn display_multiline_field() {
    #[allow(dead_code)]
    struct Linux {
        id: u8,
        destribution: &'static str,
        link: &'static str,
    }

    impl Tabled for Linux {
        fn fields(&self) -> Vec<String> {
            vec![String::new(), String::new(), String::new()]
        }

        fn headers() -> Vec<String> {
            vec![
                "Hello\nWorld".to_string(),
                "123".to_string(),
                "asd".to_string(),
            ]
        }
    }

    let data = vec![Linux {
        id: 0,
        destribution: "Fedora",
        link: "https://getfedora.org/",
    }];

    let expected = concat!(
        "-[ RECORD 0 ]--\n",
        "Hello\\nWorld | \n",
        "123          | \n",
        "asd          | \n",
    );

    let table = ExpandedDisplay::new(&data).to_string();

    assert_eq!(table, expected);
}

#[test]
fn display_multiline_record_value() {
    #[allow(dead_code)]
    struct Linux {
        id: u8,
        destribution: &'static str,
        link: &'static str,
    }

    impl Tabled for Linux {
        fn fields(&self) -> Vec<String> {
            vec![
                "Hello\nWorld".to_string(),
                "123".to_string(),
                "asd".to_string(),
            ]
        }

        fn headers() -> Vec<String> {
            vec!["1".to_string(), "2".to_string(), "3".to_string()]
        }
    }

    let data = vec![Linux {
        id: 0,
        destribution: "Fedora",
        link: "https://getfedora.org/",
    }];

    let expected = concat!(
        "-[ RECORD 0 ]-\n",
        "1 | Hello\n",
        "  | World\n",
        "2 | 123\n",
        "3 | asd\n",
    );

    let table = ExpandedDisplay::new(&data).to_string();

    assert_eq!(table, expected);
}

#[test]
fn display_with_header_template() {
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

    let expected = concat!(
        "=== Record => 0\n",
        "id           | 0\n",
        "destribution | Fedora\n",
        "link         | https://getfedora.org/\n",
        "=== Record => 1\n",
        "id           | 2\n",
        "destribution | OpenSUSE\n",
        "link         | https://www.opensuse.org/\n",
        "=== Record => 2\n",
        "id           | 3\n",
        "destribution | Endeavouros\n",
        "link         | https://endeavouros.com/\n",
    );

    let table = ExpandedDisplay::new(&data)
        .header_template(|i| format!("=== Record => {}", i))
        .to_string();

    assert_eq!(table, expected);
}

#[test]
fn display_with_formatter() {
    #[derive(Tabled)]
    struct Linux {
        id: u8,
        destribution: &'static str,
        link: &'static str,
    }

    let data = vec![
        Linux {
            id: 0,
            destribution: "Fed\nora",
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
        "-[ RECORD 0 ]----------------------------\n",
        "id           | 0!\n",
        "             | \n",
        "destribution | Fed\n",
        "             | ora!\n",
        "             | \n",
        "link         | https://getfedora.org/!\n",
        "             | \n",
        "-[ RECORD 1 ]----------------------------\n",
        "id           | 2!\n",
        "             | \n",
        "destribution | OpenSUSE!\n",
        "             | \n",
        "link         | https://www.opensuse.org/!\n",
        "             | \n",
        "-[ RECORD 2 ]----------------------------\n",
        "id           | 3!\n",
        "             | \n",
        "destribution | Endeavouros!\n",
        "             | \n",
        "link         | https://endeavouros.com/!\n",
        "             | \n",
    );

    let table = ExpandedDisplay::new(&data)
        .formatter(|s| format!("{}!\n\n", s))
        .to_string();

    assert_eq!(table, expected);
}

#[test]
fn display_with_one_line_formatter() {
    #[allow(dead_code)]
    struct Linux {
        id: u8,
        destribution: &'static str,
        link: &'static str,
    }

    impl Tabled for Linux {
        fn fields(&self) -> Vec<String> {
            vec![
                "Hello\nWorld".to_string(),
                "123".to_string(),
                "asd".to_string(),
            ]
        }

        fn headers() -> Vec<String> {
            vec!["1".to_string(), "2".to_string(), "3".to_string()]
        }
    }

    let data = vec![Linux {
        id: 0,
        destribution: "Fedora",
        link: "https://getfedora.org/",
    }];

    let expected = concat!(
        "-[ RECORD 0 ]---\n",
        "1 | Hello\\nWorld\n",
        "2 | 123\n",
        "3 | asd\n",
    );

    let table = ExpandedDisplay::new(&data)
        .formatter(|s| s.escape_debug().to_string())
        .to_string();

    assert_eq!(table, expected);
}

#[test]
fn display_with_truncate() {
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

    let expected = concat!(
        "-[ RECORD 0 ]-----\n",
        "id           | 0\n",
        "destribution | Fed\n",
        "link         | htt\n",
        "-[ RECORD 1 ]-----\n",
        "id           | 2\n",
        "destribution | Ope\n",
        "link         | htt\n",
        "-[ RECORD 2 ]-----\n",
        "id           | 3\n",
        "destribution | End\n",
        "link         | htt\n",
    );

    let table = ExpandedDisplay::new(&data).truncate(3, "").to_string();

    assert_eq!(table, expected);
}

#[test]
fn display_with_truncate_with_tail() {
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

    let expected = concat!(
        "-[ RECORD 0 ]--------\n",
        "id           | 0\n",
        "destribution | Fed...\n",
        "link         | htt...\n",
        "-[ RECORD 1 ]--------\n",
        "id           | 2\n",
        "destribution | Ope...\n",
        "link         | htt...\n",
        "-[ RECORD 2 ]--------\n",
        "id           | 3\n",
        "destribution | End...\n",
        "link         | htt...\n",
    );

    let table = ExpandedDisplay::new(&data).truncate(3, "...").to_string();

    assert_eq!(table, expected);
}

#[test]
fn display_with_wrap() {
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

    let expected = concat!(
        "-[ RECORD 0 ]-----\n",
        "id           | 0\n",
        "destribution | Fed\n",
        "             | ora\n",
        "link         | htt\n",
        "             | ps:\n",
        "             | //g\n",
        "             | etf\n",
        "             | edo\n",
        "             | ra.\n",
        "             | org\n",
        "             | /\n",
        "-[ RECORD 1 ]-----\n",
        "id           | 2\n",
        "destribution | Ope\n",
        "             | nSU\n",
        "             | SE\n",
        "link         | htt\n",
        "             | ps:\n",
        "             | //w\n",
        "             | ww.\n",
        "             | ope\n",
        "             | nsu\n",
        "             | se.\n",
        "             | org\n",
        "             | /\n",
        "-[ RECORD 2 ]-----\n",
        "id           | 3\n",
        "destribution | End\n",
        "             | eav\n",
        "             | our\n",
        "             | os\n",
        "link         | htt\n",
        "             | ps:\n",
        "             | //e\n",
        "             | nde\n",
        "             | avo\n",
        "             | uro\n",
        "             | s.c\n",
        "             | om/\n",
    );

    let table = ExpandedDisplay::new(&data).wrap(3).to_string();

    assert_eq!(table, expected);
}

#[cfg(feature = "color")]
#[test]
fn display_with_wrap_colored() {
    use owo_colors::{AnsiColors, OwoColorize};

    #[derive(Tabled)]
    struct Linux {
        id: u8,
        destribution: &'static str,
        link: String,
    }

    let data = vec![
        Linux {
            id: 0,
            destribution: "Fedora",
            link: "https://getfedora.org/".red().to_string(),
        },
        Linux {
            id: 2,
            destribution: "OpenSUSE",
            link: "https://www.opensuse.org/".to_string(),
        },
        Linux {
            id: 3,
            destribution: "Endeavouros",
            link: "https://endeavouros.com/"
                .white()
                .on_color(AnsiColors::Black)
                .to_string(),
        },
    ];

    let expected = concat!(
        "-[ RECORD 0 ]-----\n",
        "id           | 0\n",
        "destribution | Fed",
        "\n             | ora\n",
        "link         | \u{1b}[31mhtt\u{1b}[0m\n",
        "             | \u{1b}[31mps:\u{1b}[0m\n",
        "             | \u{1b}[31m//g\u{1b}[0m\n",
        "             | \u{1b}[31metf\u{1b}[0m\n",
        "             | \u{1b}[31medo\u{1b}[0m\n",
        "             | \u{1b}[31mra.\u{1b}[0m\n",
        "             | \u{1b}[31morg\u{1b}[0m\n",
        "             | \u{1b}[31m/\u{1b}[0m\n",
        "-[ RECORD 1 ]-----\n",
        "id           | 2\n",
        "destribution | Ope\n",
        "             | nSU\n",
        "             | SE\n",
        "link         | htt\n",
        "             | ps:\n",
        "             | //w\n",
        "             | ww.\n",
        "             | ope\n",
        "             | nsu\n",
        "             | se.\n",
        "             | org\n",
        "             | /\n",
        "-[ RECORD 2 ]-----\n",
        "id           | 3\n",
        "destribution | End\n",
        "             | eav\n",
        "             | our\n",
        "             | os\n",
        "link         | \u{1b}[40m\u{1b}[37mhtt\u{1b}[0m\u{1b}[0m\n",
        "             | \u{1b}[40m\u{1b}[37mps:\u{1b}[0m\u{1b}[0m\n",
        "             | \u{1b}[40m\u{1b}[37m//e\u{1b}[0m\u{1b}[0m\n",
        "             | \u{1b}[40m\u{1b}[37mnde\u{1b}[0m\u{1b}[0m\n",
        "             | \u{1b}[40m\u{1b}[37mavo\u{1b}[0m\u{1b}[0m\n",
        "             | \u{1b}[40m\u{1b}[37muro\u{1b}[0m\u{1b}[0m\n",
        "             | \u{1b}[40m\u{1b}[37ms.c\u{1b}[0m\u{1b}[0m\n",
        "             | \u{1b}[40m\u{1b}[37mom/\u{1b}[0m\u{1b}[0m\n",
    );

    let table = ExpandedDisplay::new(&data).wrap(3).to_string();

    println!("{}", table);

    assert_eq!(table, expected);
}
