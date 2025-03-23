use json_to_table::json_to_table;

use serde_json::json;
use tabled::assert::test_table;
use tabled::settings::Style;

test_table!(
    string_test,
    json_to_table(&json!("Some text string")).with(Style::modern()),
    "┌──────────────────┐"
    "│ Some text string │"
    "└──────────────────┘"
);

test_table!(
    string_multiline_test_0,
    json_to_table(&json!("Some text string\ntext on a new line\nmore text\nand a new line")).with(Style::modern()),
    "┌────────────────────┐"
    "│ Some text string   │"
    "│ text on a new line │"
    "│ more text          │"
    "│ and a new line     │"
    "└────────────────────┘"
);

test_table!(
    string_multiline_test_1,
    json_to_table(&json!("Some text string\ntext on a new line\nmore text\nand a new line\n")).with(Style::modern()),
    "┌────────────────────┐"
    "│ Some text string   │"
    "│ text on a new line │"
    "│ more text          │"
    "│ and a new line     │"
    "│                    │"
    "└────────────────────┘"
);

test_table!(
    number_test_0,
    json_to_table(&json!(123.2)).with(Style::modern()),
    "┌───────┐"
    "│ 123.2 │"
    "└───────┘"
);

test_table!(
    number_test_1,
    json_to_table(&json!(123)).with(Style::modern()),
    "┌─────┐"
    "│ 123 │"
    "└─────┘"
);

test_table!(
    null_test,
    json_to_table(&json!(null)).with(Style::modern()),
    ""
);

test_table!(
    list_test_0,
    json_to_table(&json!(["Hello", "World", "123"])).with(Style::modern()),
    "┌─────────┐"
    "│  Hello  │"
    "├─────────┤"
    "│  World  │"
    "├─────────┤"
    "│  123    │"
    "└─────────┘"
);

test_table!(
    list_test_1,
    json_to_table(&json!([{"key": "Hello"}, {"1": "2", "2": "3", "4": "5"}, 123.222229])).with(Style::modern()),
    "┌───────────────────┐"
    "│ ┌─────┬─────────┐ │"
    "│ │ key │  Hello  │ │"
    "│ └─────┴─────────┘ │"
    "├───────────────────┤"
    "│ ┌───┬─────┐       │"
    "│ │ 1 │  2  │       │"
    "│ ├───┼─────┤       │"
    "│ │ 2 │  3  │       │"
    "│ ├───┼─────┤       │"
    "│ │ 4 │  5  │       │"
    "│ └───┴─────┘       │"
    "├───────────────────┤"
    "│  123.222229       │"
    "└───────────────────┘"
);

test_table!(
    object_test_0,
    json_to_table(&json!({"message": "Hello World", "code": "123"})).with(Style::modern()),
    "┌─────────┬───────────────┐"
    "│ code    │  123          │"
    "├─────────┼───────────────┤"
    "│ message │  Hello World  │"
    "└─────────┴───────────────┘"
);

test_table!(
    object_test_1,
    json_to_table(&json!({"message": {"real": "Hello World", "cypher": "2132132"}, "code": ["123", "213"]})).with(Style::modern()),
    "┌─────────┬────────────────────────────┐"
    "│ code    │ ┌───────┐                  │"
    "│         │ │  123  │                  │"
    "│         │ ├───────┤                  │"
    "│         │ │  213  │                  │"
    "│         │ └───────┘                  │"
    "├─────────┼────────────────────────────┤"
    "│ message │ ┌────────┬───────────────┐ │"
    "│         │ │ cypher │  2132132      │ │"
    "│         │ ├────────┼───────────────┤ │"
    "│         │ │ real   │  Hello World  │ │"
    "│         │ └────────┴───────────────┘ │"
    "└─────────┴────────────────────────────┘"
);

test_table!(
    collapsed_string_test_0,
    json_to_table(&json!("Some text string")).with(Style::modern()),
    json_to_table(&json!("Some text string"))
        .collapse()
        .with(Style::modern()),
);

test_table!(
    collapsed_string_test_1,
    json_to_table(&json!("")).with(Style::modern()),
    "┌──┐"
    "│  │"
    "└──┘"
);

test_table!(
    collapsed_string_multiline_test_0,
    json_to_table(&json!(
        "Some text string\ntext on a new line\nmore text\nand a new line"
    ))
    .collapse()
    .with(Style::modern()),
    json_to_table(&json!(
        "Some text string\ntext on a new line\nmore text\nand a new line"
    ))
    .with(Style::modern()),
);

test_table!(
    collapsed_string_multiline_test_1,
    json_to_table(&json!(
        "Some text string\ntext on a new line\nmore text\nand a new line\n"
    ))
    .collapse()
    .with(Style::modern()),
    json_to_table(&json!(
        "Some text string\ntext on a new line\nmore text\nand a new line\n"
    ))
    .with(Style::modern()),
);

test_table!(
    collapsed_number_test_0,
    json_to_table(&json!(123.2))
        .collapse()
        .with(Style::modern()),
    json_to_table(&json!(123.2)).with(Style::modern()),
);

test_table!(
    collapsed_number_test_1,
    json_to_table(&json!(123)).collapse().with(Style::modern()),
    json_to_table(&json!(123)).with(Style::modern()),
);

test_table!(
    collapsed_null_test,
    json_to_table(&json!(null)).collapse().with(Style::modern()),
    "┌──┐"
    "│  │"
    "└──┘"
);

test_table!(
    collapsed_list_test_0,
    json_to_table(&json!(["Hello", "World", "123"])).collapse().with(Style::modern()),
    "┌───────┐"
    "│ Hello │"
    "├───────┤"
    "│ World │"
    "├───────┤"
    "│ 123   │"
    "└───────┘"
);

test_table!(
    collapsed_list_test_1,
    json_to_table(&json!([{"key": "Hello"}, {"1": "2", "2": "3", "4": "5"}, 123.222229])).collapse().with(Style::modern()),
    "┌─────┬───────┐"
    "│ key │ Hello │"
    "├───┬─┴───────┤"
    "│ 1 │ 2       │"
    "├───┼─────────┤"
    "│ 2 │ 3       │"
    "├───┼─────────┤"
    "│ 4 │ 5       │"
    "├───┴─────────┤"
    "│ 123.222229  │"
    "└─────────────┘"
);

test_table!(
    collapsed_list_test_2,
    json_to_table(&json!([])).collapse().with(Style::modern()),
    "┌──┐"
    "│  │"
    "└──┘"
);

test_table!(
    collapsed_object_test_1,
    json_to_table(&json!({"message": "Hello World", "code": "123"})).collapse().with(Style::modern()),
    "┌─────────┬─────────────┐"
    "│ code    │ 123         │"
    "├─────────┼─────────────┤"
    "│ message │ Hello World │"
    "└─────────┴─────────────┘"
);

test_table!(
    collapsed_object_test_2,
    json_to_table(&json!({"message": {"real": "Hello World", "cypher": "2132132"}, "code": ["123", "213"]})).collapse().with(Style::modern()),
    "┌─────────┬──────────────────────┐"
    "│ code    │ 123                  │"
    "│         ├──────────────────────┤"
    "│         │ 213                  │"
    "├─────────┼────────┬─────────────┤"
    "│ message │ cypher │ 2132132     │"
    "│         ├────────┼─────────────┤"
    "│         │ real   │ Hello World │"
    "└─────────┴────────┴─────────────┘"
);

test_table!(
    collapsed_object_test_3,
    json_to_table(&json!({})).collapse().with(Style::modern()),
    "┌──┐"
    "│  │"
    "└──┘"
);
