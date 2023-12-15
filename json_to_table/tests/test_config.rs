use json_to_table::json_to_table;
use serde_json::json;
use tabled::settings::{Alignment, Padding, Style};

use testing_table::test_table;

#[cfg(feature = "ansi")]
use tabled::{grid::ansi::ANSIBuf, grid::config::ColoredConfig};

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

#[cfg(feature = "ansi")]
test_table!(
    color_test,
    {
        struct ColorizeBorders;

        impl<R, D> tabled::settings::TableOption<R, ColoredConfig, D> for ColorizeBorders {
            fn change(self, _: &mut R, cfg: &mut ColoredConfig, _: &mut D) {
                cfg.set_border_color_default(ANSIBuf::new("\u{1b}[34m", "\u{1b}[39m"));
            }
        }

        let data = json!({
            "key1": 123,
            "234": ["123", "234", "456"],
            "key22": {
                "k1": 1,
                "k2": 2,
            }
        });

        let mut table = json_to_table(&data);
        table.collapse();
        table.with(Padding::zero());
        table.with(Style::modern());
        table.with(ColorizeBorders);

        table.to_string()
    },
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
