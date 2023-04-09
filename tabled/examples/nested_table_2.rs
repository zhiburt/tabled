//! This example demonstrates a minimalist implementation of [`Tabling`](Table) records
//! with struct fields.
//!
//! * This second nested [`Table`] example showcases the [`derive`] approach.
//!
//! * Note how the [`display_with`] attribute macro applies the custom `display_distribution`
//! filter function, which, in this case, applies styles to the final display.

use tabled::{settings::Style, Table, Tabled};

#[derive(Tabled)]
struct Vendor {
    name: &'static str,
    #[tabled(display_with = "display_distribution")]
    main_os: Distribution,
    #[tabled(display_with = "display_distribution")]
    switch_os: Distribution,
}

impl Vendor {
    fn new(name: &'static str, main_os: Distribution, switch_os: Distribution) -> Self {
        Self {
            name,
            main_os,
            switch_os,
        }
    }
}

fn display_distribution(d: &Distribution) -> String {
    Table::new([d]).with(Style::extended()).to_string()
}

#[derive(Tabled)]
struct Distribution {
    name: &'static str,
    #[tabled(display_with = "Self::display_based_on")]
    based_on: Option<&'static str>,
    is_active: bool,
    is_cool: bool,
}

impl Distribution {
    fn display_based_on(o: &Option<&'static str>) -> String {
        match o {
            &Some(s) => s.into(),
            None => "Independent".into(),
        }
    }
}

impl Distribution {
    fn new(
        name: &'static str,
        based_on: Option<&'static str>,
        is_active: bool,
        is_cool: bool,
    ) -> Self {
        Self {
            name,
            based_on,
            is_active,
            is_cool,
        }
    }
}

fn main() {
    let data = [
        Vendor::new(
            "Azure",
            Distribution::new("Windows", None, true, true),
            Distribution::new("Manjaro", Some("Arch"), true, true),
        ),
        Vendor::new(
            "AWS",
            Distribution::new("Debian", None, true, true),
            Distribution::new("Arch", None, true, true),
        ),
        Vendor::new(
            "GCP",
            Distribution::new("Debian", None, true, true),
            Distribution::new("Arch", None, true, true),
        ),
    ];

    let table = Table::new(data).with(Style::modern()).to_string();

    println!("{table}");
}
