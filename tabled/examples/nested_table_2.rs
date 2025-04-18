use tabled::{settings::Style, Table, Tabled};

fn main() {
    #[rustfmt::skip]
    let data = [
        Vendor::new("Azure", Dist::new("Windows", None), Dist::new("Manjaro", Some("Arch"))),
        Vendor::new("AWS", Dist::new("Debian", None), Dist::new("Arch", None)),
        Vendor::new("GCP", Dist::new("Debian", None), Dist::new("Arch", None)),
    ];

    let mut table = Table::new(data);
    table.with(Style::modern());

    println!("{table}");
}

#[derive(Tabled)]
struct Vendor {
    name: String,
    #[tabled(display = "display_distribution")]
    main_os: Dist,
    #[tabled(display = "display_distribution")]
    switch_os: Dist,
}

impl Vendor {
    fn new(name: &str, main_os: Dist, switch_os: Dist) -> Self {
        Self {
            name: name.to_string(),
            main_os,
            switch_os,
        }
    }
}

fn display_distribution(d: &Dist) -> String {
    Table::new([d]).with(Style::extended()).to_string()
}

#[derive(Tabled)]
struct Dist {
    name: String,
    #[tabled(format("{}", self.based_on.as_deref().unwrap_or_else(|| "Independent")))]
    based_on: Option<String>,
}

impl Dist {
    fn new(name: &str, based_on: Option<&str>) -> Self {
        Self {
            name: name.to_string(),
            based_on: based_on.map(|s| s.to_string()),
        }
    }
}
