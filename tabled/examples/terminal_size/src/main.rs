//! The example shows how we could spread a table to the size of a terminal.

use tabled::{
    settings::{Height, Style, Width},
    Table, Tabled,
};

#[derive(Tabled)]
struct Release {
    version: &'static str,
    published_date: &'static str,
    is_active: bool,
    major_feature: &'static str,
}

impl Release {
    const fn new(
        version: &'static str,
        published_date: &'static str,
        is_active: bool,
        major_feature: &'static str,
    ) -> Self {
        Self {
            version,
            published_date,
            is_active,
            major_feature,
        }
    }
}

const DATA: [Release; 3] = [
    Release::new("0.2.1", "2021-06-23", true, "#[header(inline)] attribute"),
    Release::new("0.2.0", "2021-06-19", false, "API changes"),
    Release::new("0.1.4", "2021-06-07", false, "display_with attribute"),
];

fn main() {
    let (use_width, use_height) = parse_args();

    let (terminal_size::Width(width), terminal_size::Height(height)) =
        terminal_size::terminal_size().unwrap();

    let mut table = Table::new(DATA);
    table.with(Style::extended());

    if use_width {
        table
            .with(Width::wrap(width as usize))
            .with(Width::increase(width as usize));
    }

    if use_height {
        table
            .with(Height::increase(height as usize))
            .with(Height::limit(height as usize));
    }

    println!("{table}");
}

fn parse_args() -> (bool, bool) {
    let mut args = std::env::args().skip(1);
    let a1 = args.next();
    let a2 = args.next();

    match (a1, a2) {
        (None, None) => (true, true),
        (Some(param), None) => match param.as_str() {
            "--width" => (true, false),
            "--height" => (false, true),
            _ => panic!("unexpected argument {param:?}, expected '--width' or '--height'",),
        },
        (Some(param1), Some(param2)) => {
            if param1 != "--height" || param1 != "--width" {
                panic!("unexpected argument {param1:?}, expected '--width' or '--height'",)
            }

            let use_height = param1 == "--height" || param2 == "--height";
            let use_width = param1 == "--width" || param2 == "--width";

            (use_height, use_width)
        }
        (None, Some(_)) => unreachable!(),
    }
}
