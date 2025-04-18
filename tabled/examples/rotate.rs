use tabled::{settings::Rotate, Table, Tabled};

#[derive(Tabled)]
struct Linux {
    id: u8,
    distribution: &'static str,
    link: &'static str,
}

fn main() {
    #[rustfmt::skip]
    let data = vec![
        Linux { id: 0, distribution: "Fedora", link: "https://getfedora.org/" },
        Linux { id: 2, distribution: "OpenSUSE", link: "https://www.opensuse.org/" },
        Linux { id: 3, distribution: "Endeavouros", link: "https://endeavouros.com/" },
    ];

    let mut table = Table::new(data);
    table.with(Rotate::Left);

    println!("{table}");
}
