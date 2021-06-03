//! The example can be run by this command
//! `cargo run --features color --example color`

#[cfg(not(feature = "color"))]
fn main() {
    panic!("To run this example activate a color feature. You can to it by a flag `--features`")
}

#[cfg(feature = "color")]
fn main() {
    use colored::Colorize;
    use tabled::{
        table, Alignment, Column, Format, Full, Head, HorizontalAlignment, Object, Row,
        Style, Tabled,
    };

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

    let table = table!(
        &data,
        Style::Psql,
        HorizontalAlignment(Head, Alignment::Center),
        HorizontalAlignment(Row(1..), Alignment::Left),
        Format(Full, |s| { s.blue().to_string() }),
        Format(Column(..1).and(Column(2..)), |s| { s.red().to_string() }),
    );

    println!("{}", table);
}
