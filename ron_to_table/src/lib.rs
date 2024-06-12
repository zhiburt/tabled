//! The library creates pretty table out of a [`ron`] object.
//!
//! The are 2 types of tables you can create.
//!     1. Embedded (default)
//!     2. Collapsed
//!
//! # Examples
//!
//! Embedded table.
//!
//! ```
//! let data = r#"GameConfig(
//!     window_size: (800, 600),
//!     window_title: "PAC-MAN",
//!     fullscreen: false,
//!     
//!     mouse_sensitivity: 1.4,
//!     key_bindings: {
//!         "up": Up,
//!         "down": Down,
//!         "left": Left,
//!         "right": Right,
//!     },
//!     
//!     difficulty_options: (
//!         start_difficulty: Easy,
//!         adaptive: false,
//!     ),
//! )"#;
//! let scene = ron::from_str(data).unwrap();
//!
//! assert_eq!(
//!     ron_to_table::to_string(&scene),
//!     "+----------------------+----------------------------------+\n\
//!      |  difficulty_options  | +--------------------+---------+ |\n\
//!      |                      | |  adaptive          |  false  | |\n\
//!      |                      | +--------------------+---------+ |\n\
//!      |                      | |  start_difficulty  |         | |\n\
//!      |                      | +--------------------+---------+ |\n\
//!      +----------------------+----------------------------------+\n\
//!      |  fullscreen          |  false                           |\n\
//!      +----------------------+----------------------------------+\n\
//!      |  key_bindings        | +---------+--+                   |\n\
//!      |                      | |  down   |  |                   |\n\
//!      |                      | +---------+--+                   |\n\
//!      |                      | |  left   |  |                   |\n\
//!      |                      | +---------+--+                   |\n\
//!      |                      | |  right  |  |                   |\n\
//!      |                      | +---------+--+                   |\n\
//!      |                      | |  up     |  |                   |\n\
//!      |                      | +---------+--+                   |\n\
//!      +----------------------+----------------------------------+\n\
//!      |  mouse_sensitivity   |  1.4                             |\n\
//!      +----------------------+----------------------------------+\n\
//!      |  window_size         | +-------+                        |\n\
//!      |                      | |  800  |                        |\n\
//!      |                      | +-------+                        |\n\
//!      |                      | |  600  |                        |\n\
//!      |                      | +-------+                        |\n\
//!      +----------------------+----------------------------------+\n\
//!      |  window_title        |  PAC-MAN                         |\n\
//!      +----------------------+----------------------------------+"
//! );
//! ```
//!
//! Collapsed table.
//!
//! ```
//! let data = r#"GameConfig(
//!     window_size: (800, 600),
//!     window_title: "PAC-MAN",
//!     fullscreen: false,
//!     
//!     mouse_sensitivity: 1.4,
//!     key_bindings: {
//!         "up": Up,
//!         "down": Down,
//!         "left": Left,
//!         "right": Right,
//!     },
//!     
//!     difficulty_options: (
//!         start_difficulty: Easy,
//!         adaptive: false,
//!     ),
//! )"#;
//! let scene = ron::from_str(data).unwrap();
//!
//! use ron_to_table::RonTable;
//! use tabled::settings::Style;
//! let table = RonTable::default().collapse().with(Style::modern()).build(&scene);
//!
//! assert_eq!(
//!     table,
//!     "┌────────────────────┬──────────────────┬───────┐\n\
//!      │ difficulty_options │ adaptive         │ false │\n\
//!      │                    ├──────────────────┼───────┤\n\
//!      │                    │ start_difficulty │       │\n\
//!      ├────────────────────┼──────────────────┴───────┤\n\
//!      │ fullscreen         │ false                    │\n\
//!      ├────────────────────┼───────┬──────────────────┤\n\
//!      │ key_bindings       │ down  │                  │\n\
//!      │                    ├───────┼──────────────────┤\n\
//!      │                    │ left  │                  │\n\
//!      │                    ├───────┼──────────────────┤\n\
//!      │                    │ right │                  │\n\
//!      │                    ├───────┼──────────────────┤\n\
//!      │                    │ up    │                  │\n\
//!      ├────────────────────┼───────┴──────────────────┤\n\
//!      │ mouse_sensitivity  │ 1.4                      │\n\
//!      ├────────────────────┼──────────────────────────┤\n\
//!      │ window_size        │ 800                      │\n\
//!      │                    ├──────────────────────────┤\n\
//!      │                    │ 600                      │\n\
//!      ├────────────────────┼──────────────────────────┤\n\
//!      │ window_title       │ PAC-MAN                  │\n\
//!      └────────────────────┴──────────────────────────┘",
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

use ron::Value;

pub use orientation::Orientation;
pub use table::RonTable;

mod orientation;
mod table;

/// The function converts a given [`Value`] to a pretty table.
///
/// ```
/// let data = r#"Scene(
///     materials: {
///         "metal": (reflectivity: 1.0),
///         "plastic": (reflectivity: 0.5),
///     },
///     entities: [
///         (name: "hero", material: "metal"),
///         (name: "monster", material: "plastic"),
///     ],
/// )"#;
/// let scene = ron::from_str(data).unwrap();
///
/// assert_eq!(
///     ron_to_table::to_string(&scene),
///     concat!(
///         "+-------------+--------------------------------------------+\n",
///         "|  entities   | +----------------------------+             |\n",
///         "|             | | +------------+---------+   |             |\n",
///         "|             | | |  material  |  metal  |   |             |\n",
///         "|             | | +------------+---------+   |             |\n",
///         "|             | | |  name      |  hero   |   |             |\n",
///         "|             | | +------------+---------+   |             |\n",
///         "|             | +----------------------------+             |\n",
///         "|             | | +------------+-----------+ |             |\n",
///         "|             | | |  material  |  plastic  | |             |\n",
///         "|             | | +------------+-----------+ |             |\n",
///         "|             | | |  name      |  monster  | |             |\n",
///         "|             | | +------------+-----------+ |             |\n",
///         "|             | +----------------------------+             |\n",
///         "+-------------+--------------------------------------------+\n",
///         "|  materials  | +-----------+----------------------------+ |\n",
///         "|             | |  metal    | +----------------+-----+   | |\n",
///         "|             | |           | |  reflectivity  |  1  |   | |\n",
///         "|             | |           | +----------------+-----+   | |\n",
///         "|             | +-----------+----------------------------+ |\n",
///         "|             | |  plastic  | +----------------+-------+ | |\n",
///         "|             | |           | |  reflectivity  |  0.5  | | |\n",
///         "|             | |           | +----------------+-------+ | |\n",
///         "|             | +-----------+----------------------------+ |\n",
///         "+-------------+--------------------------------------------+",
///     )
/// );
/// ```
pub fn to_string(value: &Value) -> String {
    RonTable::default().build(value)
}
