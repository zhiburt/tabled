//! Module contains a list of helpers for work with derive.

pub mod display;

/// A derive macro to implement a [`Tabled`] trait.
///
/// The macro available only when `derive` feature in turned on (and it is by default).
///
/// ```
/// use tabled::Tabled;
///
/// #[derive(Tabled)]
/// struct SomeType {
///     field0: &'static str,
///     field1: String,
///     field2: usize,
/// }
/// ```
///
/// To be able to use the derive each field must implement `std::fmt::Display`.\
/// The following example will cause an error because of that.
///
/// ```,compile_fail
/// use tabled::Tabled;
///
/// #[derive(Tabled)]
/// struct SomeType {
///     field1: Vec<usize>,
/// }
/// ```
///
/// Bellow you'll find available options for it.
///
/// ## Rename a column name
///
/// You can use a `#[tabled(rename = "")]` attribute to override a column name.
///
/// ```
/// use tabled::Tabled;
///
/// #[derive(Tabled)]
/// struct Person {
///     #[tabled(rename = "Name")]
///     first_name: String,
///     #[tabled(rename = "Surname")]
///     last_name: String,
/// }
/// ```
///
/// ## Hide a column
///
/// You can mark fields as hidden in which case they fill be ignored and not be present on a sheet.
///
/// ```
/// use tabled::Tabled;
///
/// #[derive(Tabled)]
/// struct Person {
///    id: u8,
///    #[tabled(skip)]
///    number: String,
///    name: String,
/// }
/// ```
///
/// ## Set column order
///
/// You can change the order in which they will be displayed in table.
///
/// ```
/// use tabled::Tabled;
///
/// #[derive(Tabled)]
/// struct Person {
///    id: u8,
///    #[tabled(order = 0)]
///    number: String,
///    #[tabled(order = 1)]
///    name: String,
/// }
/// ```
///
/// ## Format fields
///
/// Using `#[derive(Tabled)]` is possible only when all fields implement a `Display` trait.\
/// However, this may be not convinient for example when a field uses the `Option` type.\
/// There's 2 common ways how to solve this:
///
/// - Implement `Tabled` trait manually for a type.
/// - Wrap `Option` to something like `DisplayedOption<T>(Option<T>)` and implement a Display trait for it.
///
/// But, it's not quite convient either.
///
/// So alternatively, we provide the next solutions.
///
/// - Use the `#[tabled(display = "func")]` - attribute to set a display function.
/// - Use the `#[tabled(format = "{}")]` - attribute to format field.
///
/// ### `#[tabled(display)]`
///
/// A poverfull helper, the set function must have a first argument as a reference to a field.\
/// It supports custom arguments as well (including `self`).
///
/// You can set it right on the whole type,\
/// In which case all fields which are matching a set type will be using the given function.
///
/// We also provide a set of commonly used function for your types.\
/// You can find them in [`tabled::derive::display`].
///
/// ```
/// use tabled::Tabled;
/// use tabled::derive::display;
///
/// #[derive(Tabled)]
/// #[tabled(display(i64, "display_i64"))]
/// pub struct Record {
///     pub id: i64,
///     #[tabled(display("display::option", "unvalidated"))]
///     pub valid: Option<bool>,
///     #[tabled(display("display_private", self))]
///     pub private: (),
/// }
///
/// fn display_private(_: &(), rec: &Record) -> String {
///     todo!()
/// }
///
/// fn display_i64(val: &i64) -> String {
///     todo!()
/// }
/// ```
///
/// ### `#[tabled(format)]`
///
/// An analogue to [`format!`], which can be used right on the field.\
///
/// ```
/// use tabled::Tabled;
///
/// #[derive(Tabled)]
/// struct Record {
///     #[tabled(skip)]
///     id: u8,
///     #[tabled(format("{}.{}-{}", self.id, self.name, 123))]
///     name: String,
/// }
/// ```
///
/// ## Format headers
///
/// Beside `#[tabled(rename = "")]` you can change a format of a column name using\
/// `#[tabled(rename_all = "UPPERCASE")]`.
///
/// ```
/// use tabled::Tabled;
///
/// #[derive(Tabled)]
/// #[tabled(rename_all = "CamelCase")]
/// struct Person {
///     id: u8,
///     number: String,
///     name: String,
///     #[tabled(rename_all = "snake_case")]
///     middle_name: String,
/// }
/// ```
///
/// ## Embeding a field
///
/// You can inline a field or a variant if it implements `Tabled` trait\
/// using `#[tabled(inline)]`.
/// You can also set a prefix for inlined elements by given it as a argument\
/// `#[tabled(inline("::::"))]`.
///
/// ```
/// use tabled::Tabled;
///
/// #[derive(Tabled)]
/// struct Person {
///     id: u8,
///     name: String,
///     #[tabled(inline)]
///     ed: Education,
/// }
///
/// #[derive(Tabled)]
/// struct Education {
///     uni: String,
///     graduated: bool,
/// }
/// ```
///
/// And it works for enums as well.
///
/// ```
/// use tabled::Tabled;
///
/// #[derive(Tabled)]
/// enum Vehicle {
///     #[tabled(inline("Auto::"))]
///     Auto {
///         model: String,
///         engine: String,
///     },
///     #[tabled(inline)]
///     Bikecycle(
///         String,
///         #[tabled(inline)] Bike,
///     ),
/// }
///
/// #[derive(Tabled)]
/// struct Bike {
///     brand: &'static str,
///     price: f32,
/// }
/// ```
///
/// [`tabled::derive::display`]: crate::tabled::derive::display
pub use tabled_derive::Tabled;
