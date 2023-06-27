//! This example demonstrates how to set column names on a top horizontal line.
//!
//! It sets a `clickhouse` like table style (first seen on).

use std::iter::repeat;

use tabled::{
    grid::config::AlignmentHorizontal,
    settings::{themes::ColumnNames, Color, Style},
    Table, Tabled,
};

#[derive(Debug, Tabled)]
struct Function {
    declaration: String,
    name: String,
    return_type: String,
}

impl Function {
    fn new(decl: &str, name: &str, ret_type: &str) -> Self {
        Self {
            declaration: decl.to_string(),
            name: name.to_string(),
            return_type: ret_type.to_string(),
        }
    }
}

fn main() {
    let data = vec![
        Function::new(
            "struct stack *stack_create(int)",
            "stack_create",
            "struct stack *",
        ),
        Function::new(
            "void stack_destroy(struct stack *)",
            "stack_destroy",
            "void",
        ),
        Function::new(
            "int stack_put(struct stack *, vm_offset_t)",
            "stack_put",
            "int",
        ),
        Function::new(
            "void stack_copy(const struct stack *, struct stack *)",
            "stack_copy",
            "void",
        ),
    ];

    let mut table = Table::new(data);

    table.with(Style::modern().remove_horizontal()).with(
        ColumnNames::default()
            .set_colors(repeat(Color::BOLD | Color::BG_BLUE | Color::FG_WHITE).take(3))
            .set_alignment(AlignmentHorizontal::Center),
    );

    println!("{table}");
}
