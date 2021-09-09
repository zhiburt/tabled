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
        "-[ RECORD 0 ]-\n",
        "id           | \"0\"\n",
        "destribution | \"Fedora\"\n",
        "link         | \"https://getfedora.org/\"\n",
        "-[ RECORD 1 ]-\n",
        "id           | \"2\"\n",
        "destribution | \"OpenSUSE\"\n",
        "link         | \"https://www.opensuse.org/\"\n",
        "-[ RECORD 2 ]-\n",
        "id           | \"3\"\n",
        "destribution | \"Endeavouros\"\n",
        "link         | \"https://endeavouros.com/\"\n",
    );

    let table = ExpandedDisplay::new(&data).to_string();

    assert_eq!(table, expected);
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
        "-[ RECORD 0 ]-\n",
        "Hello\\nWorld | \"\"\n",
        "123          | \"\"\n",
        "asd          | \"\"\n",
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
            vec![String::new(), String::new(), String::new()]
        }
    }

    let data = vec![Linux {
        id: 0,
        destribution: "Fedora",
        link: "https://getfedora.org/",
    }];

    let expected = concat!(
        "-[ RECORD 0 ]-\n",
        " | \"Hello\\nWorld\"\n",
        " | \"123\"\n | \"asd\"\n",
    );

    let table = ExpandedDisplay::new(&data).to_string();

    assert_eq!(table, expected);
}

#[test]
fn display_with_custom_record_split() {
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
        "id           | \"0\"\n",
        "destribution | \"Fedora\"\n",
        "link         | \"https://getfedora.org/\"\n",
        "=== Record => 1\n",
        "id           | \"2\"\n",
        "destribution | \"OpenSUSE\"\n",
        "link         | \"https://www.opensuse.org/\"\n",
        "=== Record => 2\n",
        "id           | \"3\"\n",
        "destribution | \"Endeavouros\"\n",
        "link         | \"https://endeavouros.com/\"\n",
    );

    let table = ExpandedDisplay::new(&data)
        .format_record_head(|i| format!("=== Record => {}", i))
        .to_string();

    assert_eq!(table, expected);
}
