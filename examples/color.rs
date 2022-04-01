//! The example can be run by this command
//! `cargo run --features color --example color`

#[cfg(not(feature = "color"))]
fn main() {
    panic!("To run this example activate a color feature. You can to it by a flag `--features`")
}

#[cfg(feature = "color")]
fn main() {
    use owo_colors::OwoColorize;
    use tabled::{
        object::{Columns, Object, Rows},
        Alignment, Format, Modify, Style, Table, Tabled,
    };

    #[allow(clippy::upper_case_acronyms)]
    #[derive(Tabled)]
    struct BSD {
        distribution: &'static str,
        year_of_first_release: usize,
        is_active: bool,
    }

    let data = vec![
        BSD {
            distribution: "SunOS",
            year_of_first_release: 1982,
            is_active: false,
        },
        BSD {
            distribution: "NetBSD",
            year_of_first_release: 1993,
            is_active: true,
        },
        BSD {
            distribution: "FreeBSD",
            year_of_first_release: 1993,
            is_active: true,
        },
        BSD {
            distribution: "BSD",
            year_of_first_release: 1978,
            is_active: false,
        },
        BSD {
            distribution: "OpenBSD",
            year_of_first_release: 1995,
            is_active: true,
        },
    ];

    let table = Table::new(&data)
        .with(Style::psql())
        .with(Modify::new(Rows::first()).with(Alignment::center()))
        .with(Modify::new(Rows::new(1..)).with(Alignment::left()))
        .with(Modify::new(Columns::single(1)).with(Format::new(|s| s.blue().to_string())))
        .with(
            Modify::new(Columns::single(0).and(Columns::new(2..)))
                .with(Format::new(|s| s.red().to_string())),
        );

    println!("{}", table);
}
