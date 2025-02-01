//! Module contains a list of helpers for work with display.

use core::fmt::Debug;

/// A function which is usefull in conjuntion with
/// `#[tabled(display_with)]` and `#[tabled(display_type)]`.
///
/// ```
/// use tabled::Tabled;
/// use tabled::derive::display;
/// use testing_table::assert_table;
///
/// #[derive(Tabled)]
/// #[tabled(display_type(Option, "display::option", "Unknown"))]
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
/// `#[tabled(display_with)]` and `#[tabled(display_type)]`.
///
/// ```
/// use tabled::Tabled;
/// use tabled::derive::display;
/// use testing_table::assert_table;
///
/// #[derive(Tabled)]
/// #[tabled(display_type(Option, "display::debug"))]
/// pub struct ZKP<'a> {
///     application: &'a str,
///     state: Option<&'a str>
/// }
///
/// #[derive(Debug)]
/// pub enum State {
///     Proved,
///     Investigation,
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
