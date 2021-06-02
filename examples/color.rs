//! The example can be run by this command
//! `cargo run --features color --example color`

#[cfg(not(feature = "color"))]
fn main() {
    panic!("To run this example activate a color feature. You can to it by a flag `--features`")
}

#[cfg(feature = "color")]
fn main() {
    use colored::Colorize;
    use tabled::{table, Tabled, ChangeRing, Column, Style, Alignment, AlignmentObject, HorizontalAlignment};

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

    let expected = concat!(
        " \u{1b}[31mid\u{1b}[0m | \u{1b}[31mdestribution\u{1b}[0m |           \u{1b}[31mlink\u{1b}[0m            \n",
        "----+--------------+---------------------------\n",
        " \u{1b}[34m0\u{1b}[0m  |    \u{1b}[34mFedora\u{1b}[0m    |  \u{1b}[34mhttps://getfedora.org/\u{1b}[0m   \n",
        " \u{1b}[31m2\u{1b}[0m  |   \u{1b}[31mOpenSUSE\u{1b}[0m   | \u{1b}[31mhttps://www.opensuse.org/\u{1b}[0m \n",
        " \u{1b}[34m3\u{1b}[0m  | \u{1b}[34mEndeavouros\u{1b}[0m  | \u{1b}[34mhttps://endeavouros.com/\u{1b}[0m  \n",
    );

    let table = table!(
        &data,
        Style::Psql,
        HorizontalAlignment::new(Alignment::Center, AlignmentObject::Header),
        HorizontalAlignment::new(Alignment::Left, AlignmentObject::Full),
        ChangeRing(
            Column(..),
            vec![
                Box::new(|s| { s.red().to_string() }),
                Box::new(|s| { s.blue().to_string() }),
            ]
        ),
    );

    println!("{}", table);
}
