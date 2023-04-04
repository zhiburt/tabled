use json_to_table::json_to_table;
use serde_json::json;
use tabled::settings::Style;

#[test]
fn string_test() {
    let value = json!("Some text string");
    let table = json_to_table(&value).with(Style::modern()).to_string();

    assert_eq!(
        table,
        concat!(
            "┌──────────────────┐\n",
            "│ Some text string │\n",
            "└──────────────────┘",
        )
    );
}

#[test]
fn string_multiline_test() {
    let value = json!("Some text string\ntext on a new line\nmore text\nand a new line");
    let table = json_to_table(&value).with(Style::modern()).to_string();

    assert_eq!(
        table,
        concat!(
            "┌────────────────────┐\n",
            "│ Some text string   │\n",
            "│ text on a new line │\n",
            "│ more text          │\n",
            "│ and a new line     │\n",
            "└────────────────────┘",
        )
    );

    let value = json!("Some text string\ntext on a new line\nmore text\nand a new line\n");
    let table = json_to_table(&value).with(Style::modern()).to_string();

    assert_eq!(
        table,
        concat!(
            "┌────────────────────┐\n",
            "│ Some text string   │\n",
            "│ text on a new line │\n",
            "│ more text          │\n",
            "│ and a new line     │\n",
            "│                    │\n",
            "└────────────────────┘",
        )
    );
}

#[test]
fn number_test() {
    let value = json!(123.2);
    let table = json_to_table(&value).with(Style::modern()).to_string();

    assert_eq!(
        table,
        "┌───────┐\n\
         │ 123.2 │\n\
         └───────┘",
    );

    let value = json!(123);
    let table = json_to_table(&value).with(Style::modern()).to_string();

    assert_eq!(
        table,
        "┌─────┐\n\
         │ 123 │\n\
         └─────┘",
    );
}

#[test]
fn null_test() {
    let value = json!(null);
    let table = json_to_table(&value).with(Style::modern()).to_string();
    assert_eq!(table, "");
}

#[test]
fn list_test() {
    let value = json!(["Hello", "World", "123"]);
    let table = json_to_table(&value).with(Style::modern()).to_string();

    assert_eq!(
        table,
        concat!(
            "┌─────────┐\n",
            "│  Hello  │\n",
            "├─────────┤\n",
            "│  World  │\n",
            "├─────────┤\n",
            "│  123    │\n",
            "└─────────┘",
        )
    );

    let value = json!([{"key": "Hello"}, {"1": "2", "2": "3", "4": "5"}, 123.222229]);
    let table = json_to_table(&value).with(Style::modern()).to_string();

    assert_eq!(
        table,
        concat!(
            "┌───────────────────┐\n",
            "│ ┌─────┬─────────┐ │\n",
            "│ │ key │  Hello  │ │\n",
            "│ └─────┴─────────┘ │\n",
            "├───────────────────┤\n",
            "│ ┌───┬─────┐       │\n",
            "│ │ 1 │  2  │       │\n",
            "│ ├───┼─────┤       │\n",
            "│ │ 2 │  3  │       │\n",
            "│ ├───┼─────┤       │\n",
            "│ │ 4 │  5  │       │\n",
            "│ └───┴─────┘       │\n",
            "├───────────────────┤\n",
            "│  123.222229       │\n",
            "└───────────────────┘",
        )
    );
}

#[test]
fn object_test() {
    let value = json!({"message": "Hello World", "code": "123"});
    let table = json_to_table(&value).with(Style::modern()).to_string();

    assert_eq!(
        table,
        concat!(
            "┌─────────┬───────────────┐\n",
            "│ code    │  123          │\n",
            "├─────────┼───────────────┤\n",
            "│ message │  Hello World  │\n",
            "└─────────┴───────────────┘",
        )
    );

    let value =
        json!({"message": {"real": "Hello World", "cypher": "2132132"}, "code": ["123", "213"]});
    let table = json_to_table(&value).with(Style::modern()).to_string();

    assert_eq!(
        table,
        concat!(
            "┌─────────┬────────────────────────────┐\n",
            "│ code    │ ┌───────┐                  │\n",
            "│         │ │  123  │                  │\n",
            "│         │ ├───────┤                  │\n",
            "│         │ │  213  │                  │\n",
            "│         │ └───────┘                  │\n",
            "├─────────┼────────────────────────────┤\n",
            "│ message │ ┌────────┬───────────────┐ │\n",
            "│         │ │ cypher │  2132132      │ │\n",
            "│         │ ├────────┼───────────────┤ │\n",
            "│         │ │ real   │  Hello World  │ │\n",
            "│         │ └────────┴───────────────┘ │\n",
            "└─────────┴────────────────────────────┘",
        )
    );
}

mod squashed {
    use super::*;

    #[test]
    fn string_test() {
        let value = json!("Some text string");

        let table = json_to_table(&value).with(Style::modern()).to_string();
        let table_squashed = json_to_table(&value)
            .with(Style::modern())
            .collapse()
            .to_string();

        assert_eq!(table, table_squashed);

        let table = json_to_table(&json!(""))
            .with(Style::modern())
            .collapse()
            .to_string();

        assert_eq!(table, "┌──┐\n│  │\n└──┘");
    }

    #[test]
    fn string_multiline_test() {
        let value = json!("Some text string\ntext on a new line\nmore text\nand a new line");

        let table = json_to_table(&value).with(Style::modern()).to_string();
        let table_squashed = json_to_table(&value)
            .with(Style::modern())
            .collapse()
            .to_string();

        assert_eq!(table, table_squashed);

        let value = json!("Some text string\ntext on a new line\nmore text\nand a new line\n");

        let table = json_to_table(&value).with(Style::modern()).to_string();
        let table_squashed = json_to_table(&value)
            .with(Style::modern())
            .collapse()
            .to_string();

        assert_eq!(table, table_squashed);
    }

    #[test]
    fn number_test() {
        let value = json!(123.2);

        let table = json_to_table(&value).with(Style::modern()).to_string();
        let table_squashed = json_to_table(&value)
            .with(Style::modern())
            .collapse()
            .to_string();

        assert_eq!(table, table_squashed);

        let value = json!(123);

        let table = json_to_table(&value).with(Style::modern()).to_string();
        let table_squashed = json_to_table(&value)
            .with(Style::modern())
            .collapse()
            .to_string();

        assert_eq!(table, table_squashed);
    }

    #[test]
    fn null_test() {
        let value = json!(null);
        let table_squashed = json_to_table(&value)
            .with(Style::modern())
            .collapse()
            .to_string();

        assert_eq!(
            table_squashed,
            "┌──┐\n\
             │  │\n\
             └──┘"
        );
    }

    #[test]
    fn list_test() {
        let value = json!(["Hello", "World", "123"]);
        let table = json_to_table(&value)
            .with(Style::modern())
            .collapse()
            .to_string();

        assert_eq!(
            table,
            concat!(
                "┌───────┐\n",
                "│ Hello │\n",
                "├───────┤\n",
                "│ World │\n",
                "├───────┤\n",
                "│ 123   │\n",
                "└───────┘",
            )
        );

        let value = json!([{"key": "Hello"}, {"1": "2", "2": "3", "4": "5"}, 123.222229]);
        let table = json_to_table(&value)
            .with(Style::modern())
            .collapse()
            .to_string();

        assert_eq!(
            table,
            concat!(
                "┌─────┬───────┐\n",
                "│ key │ Hello │\n",
                "├───┬─┴───────┤\n",
                "│ 1 │ 2       │\n",
                "├───┼─────────┤\n",
                "│ 2 │ 3       │\n",
                "├───┼─────────┤\n",
                "│ 4 │ 5       │\n",
                "├───┴─────────┤\n",
                "│ 123.222229  │\n",
                "└─────────────┘",
            )
        );

        let value = json!([]);
        let table = json_to_table(&value)
            .with(Style::modern())
            .collapse()
            .to_string();

        assert_eq!(table, "┌──┐\n│  │\n└──┘");
    }

    #[test]
    fn object_test() {
        let value = json!({"message": "Hello World", "code": "123"});
        let table = json_to_table(&value)
            .with(Style::modern())
            .collapse()
            .to_string();

        assert_eq!(
            table,
            concat!(
                "┌─────────┬─────────────┐\n",
                "│ code    │ 123         │\n",
                "├─────────┼─────────────┤\n",
                "│ message │ Hello World │\n",
                "└─────────┴─────────────┘",
            )
        );

        let value = json!({"message": {"real": "Hello World", "cypher": "2132132"}, "code": ["123", "213"]});
        let table = json_to_table(&value)
            .with(Style::modern())
            .collapse()
            .to_string();

        assert_eq!(
            table,
            concat!(
                "┌─────────┬──────────────────────┐\n",
                "│ code    │ 123                  │\n",
                "│         ├──────────────────────┤\n",
                "│         │ 213                  │\n",
                "├─────────┼────────┬─────────────┤\n",
                "│ message │ cypher │ 2132132     │\n",
                "│         ├────────┼─────────────┤\n",
                "│         │ real   │ Hello World │\n",
                "└─────────┴────────┴─────────────┘",
            )
        );

        let value = json!({});
        let table = json_to_table(&value)
            .with(Style::modern())
            .collapse()
            .to_string();

        assert_eq!(table, "┌──┐\n│  │\n└──┘");
    }
}
