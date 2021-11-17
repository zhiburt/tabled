use papergrid::{Border, Entity};
use tabled::{Alignment, Footer, Full, Header, Modify, Object, Panel, Row, Style, Table, Tabled};

#[test]
fn panel_has_no_style_by_default() {
    let table = Table::new(test_data())
        .with(Style::psql())
        .with(Panel("Linux Distributions", 0))
        .to_string();

    // todo: it would be better if vertical split was not set in panel line
    // it is because it has right border printed
    let expected = concat!(
        "Linux Distributions                             \n",
        " id | destribution |           link            \n",
        "----+--------------+---------------------------\n",
        " 0  |    Fedora    |  https://getfedora.org/   \n",
        " 2  |   OpenSUSE   | https://www.opensuse.org/ \n",
        " 3  | Endeavouros  | https://endeavouros.com/  \n",
    );

    println!("{}", table);

    assert_eq!(table, expected);
}

#[test]
fn highligt_panel() {
    let table = Table::new(test_data())
        .with(Panel("Linux Distributions", 0))
        .with(Style::psql().highlight(
            Entity::Cell(0, 0),
            Border::full('#', '#', '#', '#', '#', '#', '#', '#'),
        ))
        .to_string();

    // todo: it would be better if vertical split was not set in panel line
    // it is because it has right border printed
    let expected = concat!(
        "#################################################\n",
        "#Linux Distributions                            #\n",
        "#################################################\n",
        "  id | destribution |           link            \n",
        "  0  |    Fedora    |  https://getfedora.org/   \n",
        "  2  |   OpenSUSE   | https://www.opensuse.org/ \n",
        "  3  | Endeavouros  | https://endeavouros.com/  \n",
    );

    println!("{}", table);

    assert_eq!(table, expected);
}

#[test]
fn top_panel() {
    let table = Table::new(test_data())
        .with(Panel("Linux Distributions", 0))
        .with(Modify::new(Full).with(Alignment::center_horizontal()))
        .with(Style::psql())
        .to_string();

    let expected = concat!(
        "              Linux Distributions              |\n",
        "-----------------------------------------------+\n",
        " id | destribution |           link            \n",
        " 0  |    Fedora    |  https://getfedora.org/   \n",
        " 2  |   OpenSUSE   | https://www.opensuse.org/ \n",
        " 3  | Endeavouros  | https://endeavouros.com/  \n",
    );

    println!("{}", table);

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
        "              Linux Distributions              |\n",
    );

    println!("{}", table);

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
        "              Linux Distributions              |\n",
        " 2  |   OpenSUSE   | https://www.opensuse.org/ \n",
        " 3  | Endeavouros  | https://endeavouros.com/  \n",
    );

    println!("{}", table);

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
        "              Linux Distributions              |\n",
        "-----------------------------------------------+\n",
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
        "              Linux Distributions              |\n",
        "-----------------------------------------------+\n",
        " id | destribution |           link            \n",
        " 0  |    Fedora    |  https://getfedora.org/   \n",
        " 2  |   OpenSUSE   | https://www.opensuse.org/ \n",
        " 3  | Endeavouros  | https://endeavouros.com/  \n",
        "                    The end                    |\n",
    );

    assert_eq!(table, expected);
}

#[derive(Tabled)]
struct Linux {
    id: u8,
    destribution: &'static str,
    link: &'static str,
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
