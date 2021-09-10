use tabled::{Alignment, Footer, Full, Header, Modify, Object, Panel, Row, Style, Table, Tabled};

#[derive(Tabled)]
struct Linux {
    id: u8,
    destribution: &'static str,
    link: &'static str,
}

#[test]
fn top_panel() {
    let table = Table::new(test_data())
        .with(Modify::new(Full).with(Alignment::center_horizontal()))
        .with(Panel("Linux Distributions", 0))
        .with(Style::psql())
        .to_string();

    let expected = concat!(
        "              Linux Distributions              \n",
        "-----------------------------------------------\n",
        " id | destribution |           link            \n",
        " 0  |    Fedora    |  https://getfedora.org/   \n",
        " 2  |   OpenSUSE   | https://www.opensuse.org/ \n",
        " 3  | Endeavouros  | https://endeavouros.com/  \n",
    );

    assert_eq!(table, expected);
}

#[test]
fn bottom_panel() {
    let data = test_data();
    let table = Table::new(&data)
        .with(Panel("Linux Distributions", data.len() + 1))
        .with(Modify::new(Row(data.len() + 1..)).with(Alignment::center_horizontal()))
        .with(Style::psql())
        .to_string();

    let expected = concat!(
        " id | destribution |           link            \n",
        "----+--------------+---------------------------\n",
        " 0  |    Fedora    |  https://getfedora.org/   \n",
        " 2  |   OpenSUSE   | https://www.opensuse.org/ \n",
        " 3  | Endeavouros  | https://endeavouros.com/  \n",
        "              Linux Distributions              \n",
    );

    assert_eq!(table, expected);
}

#[test]
fn inner_panel() {
    let table = Table::new(test_data())
        .with(Panel("Linux Distributions", 2))
        .with(Modify::new(Row(2..)).with(Alignment::center_horizontal()))
        .with(Style::psql())
        .to_string();

    let expected = concat!(
        " id | destribution |           link            \n",
        "----+--------------+---------------------------\n",
        " 0  |    Fedora    |  https://getfedora.org/   \n",
        "              Linux Distributions              \n",
        " 2  |   OpenSUSE   | https://www.opensuse.org/ \n",
        " 3  | Endeavouros  | https://endeavouros.com/  \n",
    );

    assert_eq!(table, expected);
}

#[test]
fn header() {
    let table = Table::new(test_data())
        .with(Header("Linux Distributions"))
        .with(Style::psql())
        .with(Modify::new(Row(0..1)).with(Alignment::center_horizontal()))
        .to_string();

    let expected = concat!(
        "              Linux Distributions              \n",
        "-----------------------------------------------\n",
        " id | destribution |           link            \n",
        " 0  |    Fedora    |  https://getfedora.org/   \n",
        " 2  |   OpenSUSE   | https://www.opensuse.org/ \n",
        " 3  | Endeavouros  | https://endeavouros.com/  \n",
    );

    assert_eq!(table, expected);
}

#[test]
fn footer() {
    let data = test_data();
    let table = Table::new(&data)
        .with(Header("Linux Distributions"))
        .with(Footer("The end"))
        .with(Style::psql())
        .with(Modify::new(Row(0..1).and(Row(data.len()..))).with(Alignment::center_horizontal()))
        .to_string();

    let expected = concat!(
        "              Linux Distributions              \n",
        "-----------------------------------------------\n",
        " id | destribution |           link            \n",
        " 0  |    Fedora    |  https://getfedora.org/   \n",
        " 2  |   OpenSUSE   | https://www.opensuse.org/ \n",
        " 3  | Endeavouros  | https://endeavouros.com/  \n",
        "                    The end                    \n",
    );

    assert_eq!(table, expected);
}

fn test_data() -> Vec<Linux> {
    vec![
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
    ]
}
