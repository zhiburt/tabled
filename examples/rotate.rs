use tabled::{Full, Modify, Padding, Rotate, Style, Table, Tabled};

#[derive(Tabled)]
struct Linux {
    id: u8,
    destribution: &'static str,
    link: &'static str,
}

fn main() {
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
        .with(Style::blank())
        .with(Modify::new(Full).with(Padding::new(1, 1, 0, 0)));

    println!("{}", table);
}
