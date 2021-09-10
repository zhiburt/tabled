use tabled::{Alignment, Disable, Full, Modify, Style, Table, Tabled};

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
        " id | destribution | link                     \n",
        "----+--------------+--------------------------\n",
        " 3  | Endeavouros  | https://endeavouros.com/ \n",
    );

    let table = Table::new(&data)
        .with(Style::psql())
        .with(Modify::new(Full).with(Alignment::left()))
        .with(Disable::Row(1..=2))
        .to_string();

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
        " 0 | Fedora      | https://getfedora.org/    \n",
        " 2 | OpenSUSE    | https://www.opensuse.org/ \n",
        " 3 | Endeavouros | https://endeavouros.com/  \n",
    );

    let table = Table::new(&data)
        .with(Style::psql())
        .with(Modify::new(Full).with(Alignment::left()))
        .with(Disable::Row(..1))
        .to_string();

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

    let table = Table::new(&data)
        .with(Style::psql())
        .with(Modify::new(Full).with(Alignment::left()))
        .with(Disable::Row(..))
        .to_string();

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
        "┌───┬─────────────┬───────────────────────────┐\n",
        "│ 0 │ Fedora      │ https://getfedora.org/    │\n",
        "├───┼─────────────┼───────────────────────────┤\n",
        "│ 2 │ OpenSUSE    │ https://www.opensuse.org/ │\n",
        "│ 3 │ Endeavouros │ https://endeavouros.com/  │\n",
        "└───┴─────────────┴───────────────────────────┘\n",
    );

    let table = Table::new(&data)
        .with(Modify::new(Full).with(Alignment::left()))
        .with(Disable::Row(..1))
        .with(Style::pseudo_clean())
        .to_string();

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
        " destribution | link                      \n",
        "--------------+---------------------------\n",
        " Fedora       | https://getfedora.org/    \n",
        " OpenSUSE     | https://www.opensuse.org/ \n",
        " Endeavouros  | https://endeavouros.com/  \n",
    );

    let table = Table::new(&data)
        .with(Style::psql())
        .with(Modify::new(Full).with(Alignment::left()))
        .with(Disable::Column(..1))
        .to_string();

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

    let table = Table::new(&data)
        .with(Style::psql())
        .with(Modify::new(Full).with(Alignment::left()))
        .with(Disable::Column(..))
        .to_string();

    assert_eq!(table, expected);
}
