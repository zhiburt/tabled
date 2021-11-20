use papergrid::Border;
use tabled::{Highlight, Style, Table, Tabled};


#[derive(Tabled)]
struct Linux {
    id: u8,
    destribution: &'static str,
    link: &'static str,
}

#[test]
fn style_highlingt_cell() {
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
        "++++++──────────────┬───────────────────────────┐\n",
        "+ id + destribution │           link            │\n",
        "+++++****************───────────────────────────┤\n",
        "│ 0  *    Fedora    *  https://getfedora.org/   │\n",
        "├────****************───────────────────────────┤\n",
        "│ 2  │   OpenSUSE   │ https://www.opensuse.org/ │\n",
        "├────┼──────────────┼───────────────────────────┤\n",
        "│ 3  │ Endeavouros  │ https://endeavouros.com/  │\n",
        "└────┴──────────────┴───────────────────────────┘\n",
    );

    let table = Table::new(&data)
        .with(Style::pseudo())
        .with(Highlight::cell(0, 0, Border::full('+', '+', '+', '+', '+', '+', '+', '+')))
        .with(Highlight::cell(1, 1, Border::full('*', '*', '*', '*', '*', '*', '*', '*')))
        .to_string();

    println!("{}", table);

    assert_eq!(table, expected);
}

#[test]
fn style_highlingt_row() {
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
        "+++++++++++++++++++++++++++++++++++++++++++++++++\n",
        "+ id │ destribution │           link            +\n",
        "+++++++++++++++++++++++++++++++++++++++++++++++++\n",
        "│ 0  │    Fedora    │  https://getfedora.org/   │\n",
        "├────┼──────────────┼───────────────────────────┤\n",
        "│ 2  │   OpenSUSE   │ https://www.opensuse.org/ │\n",
        "*************************************************\n",
        "* 3  │ Endeavouros  │ https://endeavouros.com/  *\n",
        "*************************************************\n",
    );

    let table = Table::new(&data)
        .with(Style::pseudo())
        .with(Highlight::row(0, Border::full('+', '+', '+', '+', '+', '+', '+', '+')))
        .with(Highlight::row(3, Border::full('*', '*', '*', '*', '*', '*', '*', '*')))
        .to_string();

    println!("{}", table);

    assert_eq!(table, expected);
}


#[test]
fn style_highlingt_column() {
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
        "++++++──────────────*****************************\n",
        "+ id + destribution *           link            *\n",
        "+────+──────────────*───────────────────────────*\n",
        "+ 0  +    Fedora    *  https://getfedora.org/   *\n",
        "+────+──────────────*───────────────────────────*\n",
        "+ 2  +   OpenSUSE   * https://www.opensuse.org/ *\n",
        "+────+──────────────*───────────────────────────*\n",
        "+ 3  + Endeavouros  * https://endeavouros.com/  *\n",
        "++++++──────────────*****************************\n",
    );


    let table = Table::new(&data)
        .with(Style::pseudo())
        .with(Highlight::column(0, Border::full('+', '+', '+', '+', '+', '+', '+', '+')))
        .with(Highlight::column(2, Border::full('*', '*', '*', '*', '*', '*', '*', '*')))
        .to_string();

    println!("{}", table);

    assert_eq!(table, expected);
}