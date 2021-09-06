// todo: add method for SPACING between cells.
//       add MARGIN && PADDING instead of indent?
use tabled::{Full, Indent, Modify, Rotate, Style, Table, Tabled};

#[derive(Tabled)]
struct Linux {
    id: u8,
    destribution: &'static str,
    link: &'static str,
}

#[test]
fn rotate_left() {
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
        .with(Rotate::Left)
        .with(Style::noborder())
        .with(Modify::new(Full).with(Indent::new(1, 1, 0, 0)))
        .to_string();

    assert_eq!(
        table,
        concat!(
            "     link       https://getfedora.org/   https://www.opensuse.org/   https://endeavouros.com/ \n",
            " destribution           Fedora                   OpenSUSE                  Endeavouros        \n",
            "      id                  0                          2                          3             \n"
        ),
    );
}
