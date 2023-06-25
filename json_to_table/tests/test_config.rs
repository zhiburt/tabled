use json_to_table::json_to_table;
use serde_json::json;
use tabled::settings::{Alignment, Padding, Style};

use testing_table::test_table;

#[cfg(feature = "color")]
use tabled::{
    grid::color::AnsiColor,
    grid::config::{ColoredConfig, SpannedConfig},
};

test_table!(
    config_from_table_test,
    json_to_table(&json!({
            "key1": 123,
            "234": ["123", "234", "456"],
            "key22": {
                "k1": 1,
                "k2": 2,
            }
        }))
        .with(Alignment::center())
        .with(Alignment::center_vertical())
        .with(Style::modern())
        .collapse(),
    "┌───────┬────────┐"
    "│       │  123   │"
    "│       ├────────┤"
    "│  234  │  234   │"
    "│       ├────────┤"
    "│       │  456   │"
    "├───────┼────────┤"
    "│ key1  │  123   │"
    "├───────┼────┬───┤"
    "│       │ k1 │ 1 │"
    "│ key22 ├────┼───┤"
    "│       │ k2 │ 2 │"
    "└───────┴────┴───┘"
);

test_table!(
    config_from_table_padding_zero_test,
    json_to_table(&json!({
            "key1": 123,
            "234": ["123", "234", "456"],
            "key22": {
                "k1": 1,
                "k2": 2,
            }
        }))
        .with(Padding::zero())
        .with(Alignment::center())
        .with(Alignment::center_vertical())
        .with(Style::modern())
        .collapse(),
    "┌─────┬────┐"
    "│     │123 │"
    "│     ├────┤"
    "│ 234 │234 │"
    "│     ├────┤"
    "│     │456 │"
    "├─────┼────┤"
    "│key1 │123 │"
    "├─────┼──┬─┤"
    "│     │k1│1│"
    "│key22├──┼─┤"
    "│     │k2│2│"
    "└─────┴──┴─┘"
);

test_table!(
    config_from_table_general_test,
    json_to_table(&json!({
            "key1": 123,
            "234": ["123", "234", "456"],
            "key22": {
                "k1": 1,
                "k2": 2,
            }
        }))
        .with(Padding::zero())
        .with(Alignment::center())
        .with(Alignment::center_vertical())
        .with(Style::modern()),
    "┌─────┬──────┐"
    "│     │┌───┐ │"
    "│     ││123│ │"
    "│     │├───┤ │"
    "│ 234 ││234│ │"
    "│     │├───┤ │"
    "│     ││456│ │"
    "│     │└───┘ │"
    "├─────┼──────┤"
    "│key1 │ 123  │"
    "├─────┼──────┤"
    "│     │┌──┬─┐│"
    "│     ││k1│1││"
    "│key22│├──┼─┤│"
    "│     ││k2│2││"
    "│     │└──┴─┘│"
    "└─────┴──────┘"
);

#[cfg(feature = "color")]
test_table!(
    color_test,
    json_to_table(&json!({
            "key1": 123,
            "234": ["123", "234", "456"],
            "key22": {
                "k1": 1,
                "k2": 2,
            }
        }))
        .with(ColoredConfig::new({
            let mut cfg = SpannedConfig::default();
            cfg.set_border_color_global(AnsiColor::new("\u{1b}[34m".into(), "\u{1b}[39m".into()));
            cfg
        }))
        .with(Style::modern())
        .collapse(),
    "\u{1b}[34m┌─────\u{1b}[39m\u{1b}[34m┬────┐\u{1b}[39m"
    "\u{1b}[34m│\u{1b}[39m234  \u{1b}[34m│\u{1b}[39m123 \u{1b}[34m│\u{1b}[39m"
    "\u{1b}[34m│\u{1b}[39m     \u{1b}[34m├────┤\u{1b}[39m"
    "\u{1b}[34m│\u{1b}[39m     \u{1b}[34m│\u{1b}[39m234 \u{1b}[34m│\u{1b}[39m"
    "\u{1b}[34m│\u{1b}[39m     \u{1b}[34m├────┤\u{1b}[39m"
    "\u{1b}[34m│\u{1b}[39m     \u{1b}[34m│\u{1b}[39m456 \u{1b}[34m│\u{1b}[39m"
    "\u{1b}[34m├─────\u{1b}[39m\u{1b}[34m┼────┤\u{1b}[39m"
    "\u{1b}[34m│\u{1b}[39mkey1 \u{1b}[34m│\u{1b}[39m123 \u{1b}[34m│\u{1b}[39m"
    "\u{1b}[34m├─────\u{1b}[39m\u{1b}[34m┼──\u{1b}[39m\u{1b}[34m┬─┤\u{1b}[39m"
    "\u{1b}[34m│\u{1b}[39mkey22\u{1b}[34m│\u{1b}[39mk1\u{1b}[34m│\u{1b}[39m1\u{1b}[34m│\u{1b}[39m"
    "\u{1b}[34m│\u{1b}[39m     \u{1b}[34m├──\u{1b}[39m\u{1b}[34m┼─┤\u{1b}[39m"
    "\u{1b}[34m│\u{1b}[39m     \u{1b}[34m│\u{1b}[39mk2\u{1b}[34m│\u{1b}[39m2\u{1b}[34m│\u{1b}[39m"
    "\u{1b}[34m└─────\u{1b}[39m\u{1b}[34m┴──\u{1b}[39m\u{1b}[34m┴─┘\u{1b}[39m"
);
