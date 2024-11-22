/// A derive macro to implement a [`Tabled`] trait.
///
/// The macro available only when `derive` feature in turned on (and it is by default).
///
/// To be able to use the derive each field must implement `std::fmt::Display`.
/// The following example will cause an error because of that.
///
/// ```rust,compile_fail
/// use tabled::Tabled;
/// #[derive(Tabled)]
/// struct SomeType {
///     field1: SomeOtherType,
/// }
///
/// struct SomeOtherType;
/// ```
///
/// Bellow you'll find available options for it.
///
/// ### Override a column name
///
/// You can use a `#[tabled(rename = "")]` attribute to override a column name.
///
/// ```rust,no_run
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
/// ### Hide a column
///
/// You can mark fields as hidden in which case they fill be ignored and not be present on a sheet.
///
/// A similar affect could be achieved by the means of a `Remove`.
///
/// ```rust,no_run
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
/// ### Set column order
///
/// You can change the order in which they will be displayed in table.
///
/// ```rust,no_run
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
/// ### Format fields
///
/// As was said already, using `#[derive(Tabled)]` is possible only when all fields implement a `Display` trait.
/// However, this may be often not the case for example when a field uses the `Option` type. There's 2 common ways how to solve this:
///
/// - Implement `Tabled` trait manually for a type.
/// - Wrap `Option` to something like `DisplayedOption<T>(Option<T>)` and implement a Display trait for it.
///
/// Alternatively, you can use the `#[tabled(display_with = "func")]` attribute for the field to specify a display function.
///
/// ```rust,no_run
/// use tabled::Tabled;
///
/// #[derive(Tabled)]
/// pub struct MyRecord {
///     pub id: i64,
///     #[tabled(display_with = "display_option")]
///     pub valid: Option<bool>
/// }
///
/// fn display_option(o: &Option<bool>) -> String {
///     match o {
///         Some(s) => format!("is valid thing = {}", s),
///         None => format!("is not valid"),
///     }
/// }
/// ```
///
/// It's also possible to change function argument to be `&self`,
/// using `#[tabled(display_with("some_function", self))]`
///
/// ```rust,no_run
/// use tabled::Tabled;
///
/// #[derive(Tabled)]
/// pub struct MyRecord {
///     pub id: i64,
///     #[tabled(display_with("Self::display_valid", self))]
///     pub valid: Option<bool>
/// }
///
/// impl MyRecord {
///     fn display_valid(&self) -> String {
///         match self.valid {
///             Some(s) => format!("is valid thing = {}", s),
///             None => format!("is not valid"),
///         }
///     }
/// }
/// ```
///
/// There's also a probably more suitable way for formatting, if your format is constant.
/// Using `#[tabled(format = "{}")]` and `#[tabled(format("{}"))]` and proving a general formatting string.
///
/// ```
/// use tabled::Tabled;
///
/// #[derive(Tabled)]
/// struct Record {
///     #[tabled(skip)]
///     id: u8,
///     #[tabled(format("{}.{}", self.id, self.name))]
///     name: String,
/// }
/// ```
///
/// ### Format headers
///
/// Beside `#[tabled(rename = "")]` you can change a format of a column name using
/// `#[tabled(rename_all = "UPPERCASE")]`.
///
/// ```rust,no_run
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
/// ### Inline
///
/// It's possible to inline internal data if it implements the `Tabled` trait using `#[tabled(inline)]`.
/// You can also set a prefix which will be used for all inlined elements by `#[tabled(inline("prefix>>"))]`.
///
/// ```rust,no_run
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
/// ```rust,no_run
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
pub use tabled_derive::Tabled;
