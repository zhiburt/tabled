use json_to_table::json_to_table;
use serde_json::json;
use tabled::settings::{Alignment, Padding, Style};

#[cfg(feature = "color")]
use tabled::{grid::color::AnsiColor, grid::config::SpannedConfig};

#[test]
fn config_from_table_test() {
    let value = json!(
        {
            "key1": 123,
            "234": ["123", "234", "456"],
            "key22": {
                "k1": 1,
                "k2": 2,
            }
        }
    );

    let table = json_to_table(&value)
        .with(Alignment::center())
        .with(Alignment::center_vertical())
        .with(Style::modern())
        .collapse()
        .to_string();

    assert_eq!(
        table,
        concat!(
            "┌───────┬────────┐\n",
            "│       │  123   │\n",
            "│       ├────────┤\n",
            "│  234  │  234   │\n",
            "│       ├────────┤\n",
            "│       │  456   │\n",
            "├───────┼────────┤\n",
            "│ key1  │  123   │\n",
            "├───────┼────┬───┤\n",
            "│       │ k1 │ 1 │\n",
            "│ key22 ├────┼───┤\n",
            "│       │ k2 │ 2 │\n",
            "└───────┴────┴───┘",
        )
    );
}

#[test]
fn config_from_table_padding_zero_test() {
    let value = json!(
        {
            "key1": 123,
            "234": ["123", "234", "456"],
            "key22": {
                "k1": 1,
                "k2": 2,
            }
        }
    );

    let table = json_to_table(&value)
        .with(Padding::zero())
        .with(Alignment::center())
        .with(Alignment::center_vertical())
        .with(Style::modern())
        .collapse()
        .to_string();

    assert_eq!(
        table,
        concat!(
            "┌─────┬────┐\n",
            "│     │123 │\n",
            "│     ├────┤\n",
            "│ 234 │234 │\n",
            "│     ├────┤\n",
            "│     │456 │\n",
            "├─────┼────┤\n",
            "│key1 │123 │\n",
            "├─────┼──┬─┤\n",
            "│     │k1│1│\n",
            "│key22├──┼─┤\n",
            "│     │k2│2│\n",
            "└─────┴──┴─┘",
        )
    );
}

#[test]
fn config_from_table_general_test() {
    let value = json!(
        {
            "key1": 123,
            "234": ["123", "234", "456"],
            "key22": {
                "k1": 1,
                "k2": 2,
            }
        }
    );

    let table = json_to_table(&value)
        .with(Padding::zero())
        .with(Alignment::center())
        .with(Alignment::center_vertical())
        .with(Style::modern())
        .to_string();

    assert_eq!(
        table,
        concat!(
            "┌─────┬──────┐\n",
            "│     │┌───┐ │\n",
            "│     ││123│ │\n",
            "│     │├───┤ │\n",
            "│ 234 ││234│ │\n",
            "│     │├───┤ │\n",
            "│     ││456│ │\n",
            "│     │└───┘ │\n",
            "├─────┼──────┤\n",
            "│key1 │ 123  │\n",
            "├─────┼──────┤\n",
            "│     │┌──┬─┐│\n",
            "│     ││k1│1││\n",
            "│key22│├──┼─┤│\n",
            "│     ││k2│2││\n",
            "│     │└──┴─┘│\n",
            "└─────┴──────┘",
        )
    );
}

#[cfg(feature = "color")]
#[test]
fn color_test() {
    use tabled::grid::config::ColoredConfig;

    let value = json!(
        {
            "key1": 123,
            "234": ["123", "234", "456"],
            "key22": {
                "k1": 1,
                "k2": 2,
            }
        }
    );

    let mut cfg = SpannedConfig::default();
    cfg.set_border_color_global(AnsiColor::new("\u{1b}[34m".into(), "\u{1b}[39m".into()));
    let cfg = ColoredConfig::new(cfg);

    let table = json_to_table(&value)
        .with(cfg)
        .with(Style::modern())
        .collapse()
        .to_string();

    assert_eq!(
        table,
        concat!(
            "\u{1b}[34m┌─────\u{1b}[39m\u{1b}[34m┬────┐\u{1b}[39m\n",
            "\u{1b}[34m│\u{1b}[39m234  \u{1b}[34m│\u{1b}[39m123 \u{1b}[34m│\u{1b}[39m\n",
            "\u{1b}[34m│\u{1b}[39m     \u{1b}[34m├────┤\u{1b}[39m\n",
            "\u{1b}[34m│\u{1b}[39m     \u{1b}[34m│\u{1b}[39m234 \u{1b}[34m│\u{1b}[39m\n",
            "\u{1b}[34m│\u{1b}[39m     \u{1b}[34m├────┤\u{1b}[39m\n",
            "\u{1b}[34m│\u{1b}[39m     \u{1b}[34m│\u{1b}[39m456 \u{1b}[34m│\u{1b}[39m\n",
            "\u{1b}[34m├─────\u{1b}[39m\u{1b}[34m┼────┤\u{1b}[39m\n",
            "\u{1b}[34m│\u{1b}[39mkey1 \u{1b}[34m│\u{1b}[39m123 \u{1b}[34m│\u{1b}[39m\n",
            "\u{1b}[34m├─────\u{1b}[39m\u{1b}[34m┼──\u{1b}[39m\u{1b}[34m┬─┤\u{1b}[39m\n",
            "\u{1b}[34m│\u{1b}[39mkey22\u{1b}[34m│\u{1b}[39mk1\u{1b}[34m│\u{1b}[39m1\u{1b}[34m│\u{1b}[39m\n",
            "\u{1b}[34m│\u{1b}[39m     \u{1b}[34m├──\u{1b}[39m\u{1b}[34m┼─┤\u{1b}[39m\n",
            "\u{1b}[34m│\u{1b}[39m     \u{1b}[34m│\u{1b}[39mk2\u{1b}[34m│\u{1b}[39m2\u{1b}[34m│\u{1b}[39m\n",
            "\u{1b}[34m└─────\u{1b}[39m\u{1b}[34m┴──\u{1b}[39m\u{1b}[34m┴─┘\u{1b}[39m",
        )
    );
}
