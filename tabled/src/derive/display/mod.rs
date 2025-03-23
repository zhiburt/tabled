//! Module contains a list of helpers for work with display.

use core::fmt::Debug;

/// A function which is usefull in conjuntion with
/// `#[tabled(display)]` and `#[tabled(display)]`.
///
/// It can be used with a [`prim@bool`] type.
/// You must provide 2 argumnts which will be display
/// for true and false case correspondingly.
///
/// # Example
///
/// ```
/// use tabled::{Tabled, assert::assert_table, derive::display};
///
/// #[derive(Tabled)]
/// #[tabled(display(bool, "display::bool", "Got", 0))]
/// pub struct State {
///     name: &'static str,
///     working: bool,
///     closed: bool,
/// }
///
/// let data = vec![
///     State { name: "work", working: true, closed: false },
///     State { name: "stop", working: false, closed: false },
///     State { name: "closed", working: false, closed: true },
/// ];
///
/// let table = tabled::Table::new(data);
///
/// assert_table!(
///     table,
///     "+--------+---------+--------+"
///     "| name   | working | closed |"
///     "+--------+---------+--------+"
///     "| work   | Got     | 0      |"
///     "+--------+---------+--------+"
///     "| stop   | 0       | 0      |"
///     "+--------+---------+--------+"
///     "| closed | 0       | Got    |"
///     "+--------+---------+--------+"
/// );
/// ```
pub fn bool<T, F>(value: &bool, on_true: T, on_false: F) -> String
where
    T: ToString,
    F: ToString,
{
    match value {
        true => on_true.to_string(),
        false => on_false.to_string(),
    }
}

/// A function which is usefull in conjuntion with
/// `#[tabled(display)]` and `#[tabled(display)]`.
///
/// It can be used with any [`Option`] type.
/// You must provide a second argument which represents a value be printed in case of [`None`].
///
/// # Example
///
/// ```
/// use tabled::{Tabled, derive::display};
/// use tabled::assert::assert_table;
///
/// #[derive(Tabled)]
/// #[tabled(display(Option, "display::option", "Unknown"))]
/// pub struct ZKP<'a> {
///     application: &'a str,
///     state: Option<&'a str>
/// }
///
/// let data = vec![
///     ZKP { application: "Decentralized Identity", state: Some("Proved") },
///     ZKP { application: "Voting Systems", state: Some("Investigation") },
///     ZKP { application: "Privacy-Preserving Transactions", state: None },
/// ];
///
/// let table = tabled::Table::new(data);
///
/// assert_table!(
///     table,
///     "+---------------------------------+---------------+"
///     "| application                     | state         |"
///     "+---------------------------------+---------------+"
///     "| Decentralized Identity          | Proved        |"
///     "+---------------------------------+---------------+"
///     "| Voting Systems                  | Investigation |"
///     "+---------------------------------+---------------+"
///     "| Privacy-Preserving Transactions | Unknown       |"
///     "+---------------------------------+---------------+"
/// );
/// ```
pub fn option<T>(value: &Option<T>, default: &str) -> String
where
    T: ToString,
{
    match value {
        Some(val) => val.to_string(),
        None => default.to_string(),
    }
}

/// A function which is usefull in conjuntion with
/// `#[tabled(display)]` and `#[tabled(display)]`.
///
/// It can be used with any type which implements a [`Debug`].
/// So rather then [`std::fmt::Display`] usage we will be using a debug implementation.
///
/// ```
/// use tabled::{Tabled, derive::display};
/// use tabled::assert::assert_table;
///
/// #[derive(Tabled)]
/// #[tabled(display(Option, "display::debug"))]
/// pub struct ZKP<'a> {
///     application: &'a str,
///     state: Option<&'a str>
/// }
///
/// let data = vec![
///     ZKP { application: "Decentralized Identity", state: Some("Proved") },
///     ZKP { application: "Voting Systems", state: Some("Investigation") },
///     ZKP { application: "Privacy-Preserving Transactions", state: None },
/// ];
///
/// let table = tabled::Table::new(data);
///
/// assert_table!(
///     table,
///     r#"+---------------------------------+-----------------------+"#
///     r#"| application                     | state                 |"#
///     r#"+---------------------------------+-----------------------+"#
///     r#"| Decentralized Identity          | Some("Proved")        |"#
///     r#"+---------------------------------+-----------------------+"#
///     r#"| Voting Systems                  | Some("Investigation") |"#
///     r#"+---------------------------------+-----------------------+"#
///     r#"| Privacy-Preserving Transactions | None                  |"#
///     r#"+---------------------------------+-----------------------+"#
/// );
/// ```
pub fn debug<T>(value: &T) -> String
where
    T: Debug,
{
    format!("{:?}", value)
}

/// A function which is usefull in conjuntion with
/// `#[tabled(display)]` and `#[tabled(display)]`.
///
/// It just returns an empty string.
///
/// ```
/// use tabled::{Tabled, derive::display};
/// use tabled::assert::assert_table;
///
/// #[derive(Tabled)]
/// pub struct ZKP<'a> {
///     application: &'a str,
///     #[tabled(display = "display::empty")]
///     state: Option<&'a str>
/// }
///
/// let data = vec![
///     ZKP { application: "Decentralized Identity", state: Some("Proved") },
///     ZKP { application: "Voting Systems", state: Some("Investigation") },
///     ZKP { application: "Privacy-Preserving Transactions", state: None },
/// ];
///
/// let table = tabled::Table::new(data);
///
/// assert_table!(
///     table,
///     r#"+---------------------------------+-------+"#
///     r#"| application                     | state |"#
///     r#"+---------------------------------+-------+"#
///     r#"| Decentralized Identity          |       |"#
///     r#"+---------------------------------+-------+"#
///     r#"| Voting Systems                  |       |"#
///     r#"+---------------------------------+-------+"#
///     r#"| Privacy-Preserving Transactions |       |"#
///     r#"+---------------------------------+-------+"#
/// );
/// ```
pub fn empty<T>(_value: &T) -> String {
    String::new()
}
