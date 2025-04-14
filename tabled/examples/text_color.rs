use tabled::{
    settings::{
        location::Locator,
        object::{Columns, Object, Rows},
        Color, Style,
    },
    Table, Tabled,
};

#[derive(Tabled)]
struct BSD {
    distribution: String,
    year_of_first_release: usize,
    is_active: bool,
}

fn main() {
    #[rustfmt::skip]
    let data = vec![
        BSD { distribution: String::from("BSD"), year_of_first_release: 1978, is_active: false },
        BSD { distribution: String::from("SunOS"), year_of_first_release: 1982, is_active: false },
        BSD { distribution: String::from("NetBSD"), year_of_first_release: 1993, is_active: true },
        BSD { distribution: String::from("FreeBSD"), year_of_first_release: 1993, is_active: true },
        BSD { distribution: String::from("OpenBSD"), year_of_first_release: 1995, is_active: true },
    ];

    let mut table = Table::new(data);
    table
        .with(Style::psql())
        .modify(Rows::first(), Color::BG_BLUE)
        .modify(Locator::content("false"), Color::BG_RED)
        .modify(
            Locator::value(Columns::single(1).not(Rows::first()), |new, old| new > old),
            Color::BG_GREEN,
        )
        .modify(
            Locator::value(Columns::single(1).not(Rows::first()), |new, old| new < old),
            Color::BG_GREEN,
        );

    println!("{table}");
}
