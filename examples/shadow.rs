//! The example relayes on STDIN to read the message which will be outputed.
//!
//! The table is inspired by <https://en.wikipedia.org/wiki/Box-drawing_character>
//!
//! ## Example
//!
//! `echo -e -n 'Some text\nIn the box' | cargo run --package tabled --example shadow`

use std::{io::Read, iter::FromIterator};

use tabled::{
    builder::Builder,
    object::Cell,
    row,
    shadow::Shadow,
    style::{BorderChar, Offset, RawStyle},
    Height, Modify, Padding, Style, Table, Width,
};

fn main() {
    let message = read_message();

    let main_table = create_main_table(&message);
    let main_table_width = main_table.total_width();
    let small_table_row = create_small_table_list(main_table_width);
    println!("{}", small_table_row);
    println!("{}", main_table);
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
                .left_intersection('╟')
                .right_intersection('╢')
                .top_right_corner('╖')
                .top_left_corner('╓')
                .bottom_right_corner('╜')
                .bottom_left_corner('╙')
                .into(),
        ),
        create_small_table(
            Style::modern()
                .top('═')
                .bottom('═')
                .top_right_corner('╕')
                .top_left_corner('╒')
                .bottom_right_corner('╛')
                .bottom_left_corner('╘')
                .horizontal('═')
                .left_intersection('╞')
                .right_intersection('╡')
                .inner_intersection('╪')
                .top_intersection('╤')
                .bottom_intersection('╧')
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

// ┌───────────────────┐
// │  ╔═══╗ Some Text  │▒
// │  ╚═╦═╝ in the box │▒
// ╞═╤══╩══╤═══════════╡▒
// │ ├──┬──┤           │▒
// │ └──┴──┘           │▒
// └───────────────────┘▒
// ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒

fn create_main_table(message: &str) -> Table {
    let count_lines = papergrid::util::count_lines(message);
    let message_width = papergrid::util::string_width_multiline_tab(message, 4);
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
        .with(Style::modern().off_vertical())
        .with(Modify::new(Cell(0, 0)).with(BorderChar::vertical('╞', Offset::Begin(count_lines))))
        .with(Modify::new(Cell(0, 2)).with(BorderChar::vertical('╡', Offset::Begin(count_lines))))
        .with(Shadow::new(2));

    table
}
