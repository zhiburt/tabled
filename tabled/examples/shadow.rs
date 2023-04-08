//! This example can be run with the following command:
//!
//! `echo -e -n 'Some text\nIn the box' | cargo run --example shadow`
//!
//! This example demonstrates using the [`Shadow`] [`TableOption`] to create
//! a striking frame around a [`Table`] display.
//!
//! * [`Shadow`] supports several configurations:
//!     * Thickness
//!     * Offset
//!     * Direction
//!     * Color
//!     * Fill character
//!
//! * 🎉 Inspired by <https://en.wikipedia.org/wiki/Box-drawing_character>

use std::{io::Read, iter::FromIterator};

use tabled::{
    builder::Builder,
    grid::util::string,
    row,
    settings::{
        object::Cell,
        style::{BorderChar, Offset, RawStyle, Style},
        Height, Modify, Padding, Shadow, Width,
    },
    Table,
};

fn main() {
    let message = read_message();
    print_table(message);
}

fn print_table(message: String) {
    let main_table = create_main_table(&message);
    let main_table_width = main_table.total_width();
    let small_table_row = create_small_table_list(main_table_width);
    println!("{small_table_row}");
    println!("{main_table}");
}

fn read_message() -> String {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    buf
}

fn create_small_table_list(width_available: usize) -> String {
    let mut tables = [
        create_small_table(Style::modern().into()),
        create_small_table(Style::extended().into()),
        create_small_table(
            Style::modern()
                .left('║')
                .right('║')
                .intersection_left('╟')
                .intersection_right('╢')
                .corner_top_right('╖')
                .corner_top_left('╓')
                .corner_bottom_right('╜')
                .corner_bottom_left('╙')
                .into(),
        ),
        create_small_table(
            Style::modern()
                .top('═')
                .bottom('═')
                .corner_top_right('╕')
                .corner_top_left('╒')
                .corner_bottom_right('╛')
                .corner_bottom_left('╘')
                .horizontal('═')
                .intersection_left('╞')
                .intersection_right('╡')
                .intersection_top('╤')
                .intersection_bottom('╧')
                .intersection('╪')
                .into(),
        ),
    ];
    const TOTAL_TABLE_WIDTH: usize = 19;

    if width_available > TOTAL_TABLE_WIDTH {
        let mut rest = width_available - TOTAL_TABLE_WIDTH;
        while rest > 0 {
            for table in &mut tables {
                let current_width = table.total_width();
                table.with(Width::increase(current_width + 1));
                rest -= 1;

                if rest == 0 {
                    break;
                }
            }
        }
    }

    let small_table_row = row![tables[0], tables[1], tables[2], tables[3]]
        .with(Style::blank())
        .with(Padding::zero())
        .to_string();
    small_table_row
}

fn create_small_table(style: RawStyle) -> Table {
    let mut table = Builder::from_iter(vec![vec![" ", ""], vec![" ", ""]]).build();
    table
        .with(style)
        .with(Padding::zero())
        .with(Height::list([1, 0]));

    table
}

fn create_main_table(message: &str) -> Table {
    let (count_lines, message_width) = string::string_dimension(message);
    let count_additional_separators = if count_lines > 2 { count_lines - 2 } else { 0 };

    let left_table = format!(
        "  ╔═══╗ \n  ╚═╦═╝ \n{}═╤══╩══╤\n ├──┬──┤\n └──┴──┘",
        (0..count_additional_separators)
            .map(|_| "    ║   \n")
            .collect::<String>()
    );

    let message = if count_lines < 2 {
        let mut i = count_lines;
        let mut buf = message.to_string();
        while i < 2 {
            buf.push('\n');
            i += 1;
        }

        buf
    } else {
        message.to_owned()
    };
    let count_lines = count_lines.max(2);

    let message = format!("{}\n{}", message, "═".repeat(message_width));

    let mut table = row![left_table, message];
    table
        .with(Padding::zero())
        .with(Style::modern().remove_vertical())
        .with(
            Modify::new(Cell::new(0, 0))
                .with(BorderChar::vertical('╞', Offset::Begin(count_lines))),
        )
        .with(
            Modify::new(Cell::new(0, 2))
                .with(BorderChar::vertical('╡', Offset::Begin(count_lines))),
        )
        .with(Shadow::new(2));

    table
}
