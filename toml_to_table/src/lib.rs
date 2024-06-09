//! The library creates a pretty table out of a [`toml::Value`].
//!
//! The are 2 types of tables you can create.
//!     1. Embedded (default)
//!     2. Collapsed
//!
//! You can configure table using [`TomlTable`].
//!
//! # Examples
//!
//! ## Embedded table.
//!
//! ```
//! let data = r#"
//! [game-config]
//! window_size = [800, 600]
//! window_title = "PAC-MAN"
//! fullscreen = false
//! mouse_sensitivity = 1.4
//!
//! [game-config.key_bindings]
//! up = "Up"
//! down = "Down"
//! left = "Left"
//! right = "Right"
//!
//! [game-config.difficulty_options]
//! start_difficulty = "Easy"
//! adaptive = false
//! "#;
//!
//! let scene = toml::from_str(data).unwrap();
//! let table = toml_to_table::to_string(&scene);
//!
//! assert_eq!(
//!     table,
//!     "+-------------+---------------------------------------------------------+\n\
//!      | game-config | +--------------------+--------------------------------+ |\n\
//!      |             | | difficulty_options | +------------------+---------+ | |\n\
//!      |             | |                    | | adaptive         |  false  | | |\n\
//!      |             | |                    | +------------------+---------+ | |\n\
//!      |             | |                    | | start_difficulty |  Easy   | | |\n\
//!      |             | |                    | +------------------+---------+ | |\n\
//!      |             | +--------------------+--------------------------------+ |\n\
//!      |             | | fullscreen         |  false                         | |\n\
//!      |             | +--------------------+--------------------------------+ |\n\
//!      |             | | key_bindings       | +-------+---------+            | |\n\
//!      |             | |                    | | down  |  Down   |            | |\n\
//!      |             | |                    | +-------+---------+            | |\n\
//!      |             | |                    | | left  |  Left   |            | |\n\
//!      |             | |                    | +-------+---------+            | |\n\
//!      |             | |                    | | right |  Right  |            | |\n\
//!      |             | |                    | +-------+---------+            | |\n\
//!      |             | |                    | | up    |  Up     |            | |\n\
//!      |             | |                    | +-------+---------+            | |\n\
//!      |             | +--------------------+--------------------------------+ |\n\
//!      |             | | mouse_sensitivity  |  1.4                           | |\n\
//!      |             | +--------------------+--------------------------------+ |\n\
//!      |             | | window_size        | +-------+                      | |\n\
//!      |             | |                    | |  800  |                      | |\n\
//!      |             | |                    | +-------+                      | |\n\
//!      |             | |                    | |  600  |                      | |\n\
//!      |             | |                    | +-------+                      | |\n\
//!      |             | +--------------------+--------------------------------+ |\n\
//!      |             | | window_title       |  PAC-MAN                       | |\n\
//!      |             | +--------------------+--------------------------------+ |\n\
//!      +-------------+---------------------------------------------------------+"
//! );
//! ```
//!
//! ## Collapsed table.
//!
//! ```
//! let data = r#"
//! [game-config]
//! window_size = [800, 600]
//! window_title = "PAC-MAN"
//! fullscreen = false
//! mouse_sensitivity = 1.4
//!
//! [game-config.key_bindings]
//! up = "Up"
//! down = "Down"
//! left = "Left"
//! right = "Right"
//!
//! [game-config.difficulty_options]
//! start_difficulty = "Easy"
//! adaptive = false
//! "#;
//!
//! let scene = toml::from_str(data).unwrap();
//! let table = toml_to_table::to_string_collapsed(&scene);
//!
//! assert_eq!(
//!     table,
//!     "+-------------+--------------------+------------------+-------+\n\
//!      | game-config | difficulty_options | adaptive         | false |\n\
//!      |             |                    +------------------+-------+\n\
//!      |             |                    | start_difficulty | Easy  |\n\
//!      |             +--------------------+------------------+-------+\n\
//!      |             | fullscreen         | false                    |\n\
//!      |             +--------------------+-------+------------------+\n\
//!      |             | key_bindings       | down  | Down             |\n\
//!      |             |                    +-------+------------------+\n\
//!      |             |                    | left  | Left             |\n\
//!      |             |                    +-------+------------------+\n\
//!      |             |                    | right | Right            |\n\
//!      |             |                    +-------+------------------+\n\
//!      |             |                    | up    | Up               |\n\
//!      |             +--------------------+-------+------------------+\n\
//!      |             | mouse_sensitivity  | 1.4                      |\n\
//!      |             +--------------------+--------------------------+\n\
//!      |             | window_size        | 800                      |\n\
//!      |             |                    +--------------------------+\n\
//!      |             |                    | 600                      |\n\
//!      |             +--------------------+--------------------------+\n\
//!      |             | window_title       | PAC-MAN                  |\n\
//!      +-------------+--------------------+--------------------------+"
//! );
//! ```
//!
//! ## [`TomlTable`] style configuration (embedded)
//!
//! ```
//! use toml_to_table::TomlTable;
//! use tabled::settings::{Padding, Style};
//!
//! let data = r#"
//! [game-config]
//! window_size = [800, 600]
//! window_title = "PAC-MAN"
//! fullscreen = false
//! mouse_sensitivity = 1.4
//!
//! [game-config.key_bindings]
//! up = "Up"
//! down = "Down"
//! left = "Left"
//! right = "Right"
//!
//! [game-config.difficulty_options]
//! start_difficulty = "Easy"
//! adaptive = false
//! "#;
//!
//! let scene = toml::from_str(data).unwrap();
//!
//! let table = TomlTable::new(&scene)
//!     .with(Padding::zero())
//!     .with(Style::modern())
//!     .to_string();
//!
//! assert_eq!(
//!     table,
//!     "┌───────────┬─────────────────────────────────────────────┐\n\
//!      │game-config│┌──────────────────┬────────────────────────┐│\n\
//!      │           ││difficulty_options│┌────────────────┬─────┐││\n\
//!      │           ││                  ││adaptive        │false│││\n\
//!      │           ││                  │├────────────────┼─────┤││\n\
//!      │           ││                  ││start_difficulty│Easy │││\n\
//!      │           ││                  │└────────────────┴─────┘││\n\
//!      │           │├──────────────────┼────────────────────────┤│\n\
//!      │           ││fullscreen        │false                   ││\n\
//!      │           │├──────────────────┼────────────────────────┤│\n\
//!      │           ││key_bindings      │┌─────┬─────┐           ││\n\
//!      │           ││                  ││down │Down │           ││\n\
//!      │           ││                  │├─────┼─────┤           ││\n\
//!      │           ││                  ││left │Left │           ││\n\
//!      │           ││                  │├─────┼─────┤           ││\n\
//!      │           ││                  ││right│Right│           ││\n\
//!      │           ││                  │├─────┼─────┤           ││\n\
//!      │           ││                  ││up   │Up   │           ││\n\
//!      │           ││                  │└─────┴─────┘           ││\n\
//!      │           │├──────────────────┼────────────────────────┤│\n\
//!      │           ││mouse_sensitivity │1.4                     ││\n\
//!      │           │├──────────────────┼────────────────────────┤│\n\
//!      │           ││window_size       │┌───┐                   ││\n\
//!      │           ││                  ││800│                   ││\n\
//!      │           ││                  │├───┤                   ││\n\
//!      │           ││                  ││600│                   ││\n\
//!      │           ││                  │└───┘                   ││\n\
//!      │           │├──────────────────┼────────────────────────┤│\n\
//!      │           ││window_title      │PAC-MAN                 ││\n\
//!      │           │└──────────────────┴────────────────────────┘│\n\
//!      └───────────┴─────────────────────────────────────────────┘"
//! );
//! ```
//!
//! ## [`TomlTable`] style configuration (collapsed)
//!
//! ```
//! use toml_to_table::TomlTable;
//! use tabled::settings::{Padding, Style};
//!
//! let data = r#"
//! [game-config]
//! window_size = [800, 600]
//! window_title = "PAC-MAN"
//! fullscreen = false
//! mouse_sensitivity = 1.4
//!
//! [game-config.key_bindings]
//! up = "Up"
//! down = "Down"
//! left = "Left"
//! right = "Right"
//!
//! [game-config.difficulty_options]
//! start_difficulty = "Easy"
//! adaptive = false
//! "#;
//!
//! let scene = toml::from_str(data).unwrap();
//!
//! let table = TomlTable::new(&scene)
//!     .with(Padding::zero())
//!     .with(Style::modern())
//!     .collapse()
//!     .to_string();
//!
//! assert_eq!(
//!     table,
//!     "┌───────────┬──────────────────┬────────────────┬─────┐\n\
//!      │game-config│difficulty_options│adaptive        │false│\n\
//!      │           │                  ├────────────────┼─────┤\n\
//!      │           │                  │start_difficulty│Easy │\n\
//!      │           ├──────────────────┼────────────────┴─────┤\n\
//!      │           │fullscreen        │false                 │\n\
//!      │           ├──────────────────┼─────┬────────────────┤\n\
//!      │           │key_bindings      │down │Down            │\n\
//!      │           │                  ├─────┼────────────────┤\n\
//!      │           │                  │left │Left            │\n\
//!      │           │                  ├─────┼────────────────┤\n\
//!      │           │                  │right│Right           │\n\
//!      │           │                  ├─────┼────────────────┤\n\
//!      │           │                  │up   │Up              │\n\
//!      │           ├──────────────────┼─────┴────────────────┤\n\
//!      │           │mouse_sensitivity │1.4                   │\n\
//!      │           ├──────────────────┼──────────────────────┤\n\
//!      │           │window_size       │800                   │\n\
//!      │           │                  ├──────────────────────┤\n\
//!      │           │                  │600                   │\n\
//!      │           ├──────────────────┼──────────────────────┤\n\
//!      │           │window_title      │PAC-MAN               │\n\
//!      └───────────┴──────────────────┴──────────────────────┘"
//! );
//! ```

#![deny(unused_must_use)]
#![warn(
    missing_docs,
    rust_2018_idioms,
    missing_debug_implementations,
    unreachable_pub
)]
#![allow(clippy::uninlined_format_args)]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/zhiburt/tabled/86ac146e532ce9f7626608d7fd05072123603a2e/assets/tabled-gear.svg"
)]

use toml::Value;

pub use table::{Orientation, TomlTable};

mod table;

/// The function converts a given [`Value`] to a pretty table,
/// recursively creating new tables if necessary.
///
/// ```
/// let file = r#"
///[materials]
///metal = { reflectivity = 1.0 }
///plastic = { reflectivity = 0.5 }
///
///[[entities]]
///name = "hero"
///material = "metal"
///
///[[entities]]
///name = "monster"
///material = "plastic"
///"#;
///
/// let scene = toml::from_str(file).unwrap();
/// let table = toml_to_table::to_string(&scene);
///
/// assert_eq!(
///     table,
///     concat!(
///         "+-----------+----------------------------------------+\n",
///         "| entities  | +--------------------------+           |\n",
///         "|           | | +----------+---------+   |           |\n",
///         "|           | | | material |  metal  |   |           |\n",
///         "|           | | +----------+---------+   |           |\n",
///         "|           | | | name     |  hero   |   |           |\n",
///         "|           | | +----------+---------+   |           |\n",
///         "|           | +--------------------------+           |\n",
///         "|           | | +----------+-----------+ |           |\n",
///         "|           | | | material |  plastic  | |           |\n",
///         "|           | | +----------+-----------+ |           |\n",
///         "|           | | | name     |  monster  | |           |\n",
///         "|           | | +----------+-----------+ |           |\n",
///         "|           | +--------------------------+           |\n",
///         "+-----------+----------------------------------------+\n",
///         "| materials | +---------+--------------------------+ |\n",
///         "|           | | metal   | +--------------+-----+   | |\n",
///         "|           | |         | | reflectivity |  1  |   | |\n",
///         "|           | |         | +--------------+-----+   | |\n",
///         "|           | +---------+--------------------------+ |\n",
///         "|           | | plastic | +--------------+-------+ | |\n",
///         "|           | |         | | reflectivity |  0.5  | | |\n",
///         "|           | |         | +--------------+-------+ | |\n",
///         "|           | +---------+--------------------------+ |\n",
///         "+-----------+----------------------------------------+",
///     )
/// );
/// ```
pub fn to_string(value: &Value) -> String {
    TomlTable::new(value).to_string()
}

/// The function converts a given [`Value`] to a pretty table,
/// recursively creating new tables if necessary.
///
/// ```
/// let file = r#"
///[materials]
///metal = { reflectivity = 1.0 }
///plastic = { reflectivity = 0.5 }
///
///[[entities]]
///name = "hero"
///material = "metal"
///
///[[entities]]
///name = "monster"
///material = "plastic"
///"#;
///
/// let scene = toml::from_str(file).unwrap();
/// let table = toml_to_table::to_string_collapsed(&scene);
///
/// assert_eq!(
///     table,
///     concat!(
///         "+-----------+----------+-------------------+\n",
///         "| entities  | material | metal             |\n",
///         "|           +----------+-------------------+\n",
///         "|           | name     | hero              |\n",
///         "|           +----------+-------------------+\n",
///         "|           | material | plastic           |\n",
///         "|           +----------+-------------------+\n",
///         "|           | name     | monster           |\n",
///         "+-----------+---------++-------------+-----+\n",
///         "| materials | metal   | reflectivity | 1   |\n",
///         "|           +---------+--------------+-----+\n",
///         "|           | plastic | reflectivity | 0.5 |\n",
///         "+-----------+---------+--------------+-----+",
///     )
/// );
/// ```
pub fn to_string_collapsed(value: &Value) -> String {
    TomlTable::new(value).collapse().to_string()
}
