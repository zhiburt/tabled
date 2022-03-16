[![Build Status](https://github.com/zhiburt/tabled/actions/workflows/ci.yml/badge.svg?style=for-the-badge)](https://github.com/zhiburt/tabled/actions)
[![Coverage Status](https://img.shields.io/coveralls/github/zhiburt/tabled/master)](https://coveralls.io/github/zhiburt/tabled)
[![Crate](https://img.shields.io/crates/v/tabled)](https://crates.io/crates/tabled)
[![docs.rs](https://img.shields.io/docsrs/tabled?color=blue)](https://docs.rs/tabled)
[![license](https://img.shields.io/crates/l/tabled)](./LICENSE.txt)
[![dependency status](https://deps.rs/repo/github/zhiburt/tabled/status.svg)](https://deps.rs/repo/github/zhiburt/tabled)

# tabled

An easy to use library for pretty printing tables of Rust `struct`s and `enum`s.

## Table of Contents

* [Usage](#Usage)
* [Settings](#Settings)
    * [Style](#Style)
        * [ASCII](#ASCII)
        * [Psql](#Psql)
        * [Github Markdown](#Github-Markdown)
        * [Modern](#Modern)
        * [ReStructuredText](#ReStructuredText)
        * [Extended](#Extended)
        * [Dots](#Dots)
        * [Blank](#Blank)
        * [Custom](#Custom)
    * [Text in top border](#Text-in-top-border)
    * [Alignment](#Alignment)
    * [Format](#Format)
    * [Indent](#Indent)
    * [Max width](#Max-width)
    * [Rotate](#Rotate)
    * [Disable](#Disable)
    * [Extract](#Extract)
        * [Refinish](#Refinishing)
    * [Header and Footer](#Header-and-Footer)
    * [Concat](#Concat)
* [Derive](#Derive)
    * [Column name override](#Column-name-override)
    * [Hide a column](#Hide-a-column)
    * [Custom field formatting](#Custom-field-formatting)
    * [Inline](#Inline)
* [Features](#Features)
    * [Tuple combination](#Tuple-combination)
    * [Color](#Color)
    * [Object](#Object)
* [Views](#Views)
    * [Expanded Display](#Expanded-Display)
* [Notes](#Notes)
   * [ANSI escape codes](#ANSI-escape-codes) 
   * [Dynamic table](#Dynamic-table)
   * [Emoji](#Emoji)

## Usage

To print a list of structs or enums as a table your types should implement the the `Tabled` trait or derive with a `#[derive(Tabled)]` macro.

```rust
use tabled::{Tabled, Table};

#[derive(Tabled)]
struct Language {
    name: &'static str,
    designed_by: &'static str,
    invented_year: usize,
}

let languages = vec![
    Language{
        name: "C",
        designed_by: "Dennis Ritchie",
        invented_year: 1972
    },
    Language{
        name: "Rust",
        designed_by: "Graydon Hoare",
        invented_year: 2010
    },
    Language{
        name: "Go",
        designed_by: "Rob Pike",
        invented_year: 2009
    },
];

let table = Table::new(languages).to_string();

let expected = "+------+----------------+---------------+\n\
                | name |  designed_by   | invented_year |\n\
                +------+----------------+---------------+\n\
                |  C   | Dennis Ritchie |     1972      |\n\
                +------+----------------+---------------+\n\
                | Rust | Graydon Hoare  |     2010      |\n\
                +------+----------------+---------------+\n\
                |  Go  |    Rob Pike    |     2009      |\n\
                +------+----------------+---------------+\n";

assert_eq!(table, expected);
```

Most of the default types implement the trait out of the box.

```rust
use tabled::TableIteratorExt;
let some_numbers = [1, 2, 3];
let table = some_numbers.table();
```

## Settings

In this section is listened a set of settings you can apply for your table.

### Style

There are a list of ready to use styles.
Each style can be castomized.

A custom style also can be created from scratch.

A style can be used by passing it to `.with` method of `Table`.

```rust
let table = Table::new(&data).with(Style::psql());
```

Bellow rendered a list of pre configured styles.

If you think that there's some valuable style to be added,
Please open an issue.

#### ASCII

```
+------+----------------+---------------+
| name |  designed_by   | invented_year |
+------+----------------+---------------+
|  C   | Dennis Ritchie |     1972      |
+------+----------------+---------------+
| Rust | Graydon Hoare  |     2010      |
+------+----------------+---------------+
|  Go  |    Rob Pike    |     2009      |
+------+----------------+---------------+
```

#### Psql

```
 name |  designed_by   | invented_year 
------+----------------+---------------
  C   | Dennis Ritchie |     1972      
 Rust | Graydon Hoare  |     2010      
  Go  |    Rob Pike    |     2009      
```

#### Github Markdown

```
| name |  designed_by   | invented_year |
|------+----------------+---------------|
|  C   | Dennis Ritchie |     1972      |
| Rust | Graydon Hoare  |     2010      |
|  Go  |    Rob Pike    |     2009      |
```

#### Modern

```
┌──────┬────────────────┬───────────────┐
│ name │  designed_by   │ invented_year │
├──────┼────────────────┼───────────────┤
│  C   │ Dennis Ritchie │     1972      │
├──────┼────────────────┼───────────────┤
│ Rust │ Graydon Hoare  │     2010      │
├──────┼────────────────┼───────────────┤
│  Go  │    Rob Pike    │     2009      │
└──────┴────────────────┴───────────────┘
```

#### ReStructuredText

```
====== ================ ===============
 name    designed_by     invented_year 
====== ================ ===============
  C     Dennis Ritchie       1972      
 Rust   Graydon Hoare        2010      
  Go       Rob Pike          2009      
====== ================ ===============
```

#### Extended

```
╔══════╦════════════════╦═══════════════╗
║ name ║  designed_by   ║ invented_year ║
╠══════╬════════════════╬═══════════════╣
║  C   ║ Dennis Ritchie ║     1972      ║
╠══════╬════════════════╬═══════════════╣
║ Rust ║ Graydon Hoare  ║     2010      ║
╠══════╬════════════════╬═══════════════╣
║  Go  ║    Rob Pike    ║     2009      ║
╚══════╩════════════════╩═══════════════╝
```

#### Dots

```
.........................................
: name :  designed_by   : invented_year :
:......:................:...............:
:  C   : Dennis Ritchie :     1972      :
: Rust : Graydon Hoare  :     2010      :
:  Go  :    Rob Pike    :     2009      :
:......:................:...............:
```

#### Blank

```
 name    designed_by     invented_year 
  C     Dennis Ritchie       1972      
  Rust   Graydon Hoare       2010      
  Go       Rob Pike          2009      
```

#### Custom

You can modify existing styles to fit your needs.

```rust
let style = Style::modern().header_off().horizontal_off();
```

The style will look like the following.

```rust
┌──────┬────────────────┬───────────────┐
│ name │  designed_by   │ invented_year │
│  C   │ Dennis Ritchie │     1972      │
│ Rust │ Graydon Hoare  │     2010      │
│  Go  │    Rob Pike    │     2009      │
└──────┴────────────────┴───────────────┘
```

You can find more methods which are available in the [documentation](https://docs.rs/tabled/latest/tabled/style/struct.CustomStyle.html)

### Alignment

You can set a horizontal and vertical alignment for a `Header`, `Column`, `Row` or `Full` set of cells.

```rust
Table::new(&data)
    .with(Modify::new(Full)
        .with(Alignment::left())
        .with(Alignment::top())
    );
```

### Format

The `Format` function provides an interface for a modification of cells.

```rust
Table::new(&data)
    .with(Style::psql()),
    .with(Modify::new(Column(..)).with(Format(|s| format!("<< {} >>", s))))
    .with(Modify::new(Row(..1)).with(Format(|s| format!("Head {}", s))));
```

It's also possible to use functions with signature `Fn(&str) -> String` as a formatter.

```rust
Table::new(&data)
    .with(Style::psql()),
    .with(Modify::new(Column(..)).with(|s: &str| format!("<< {} >>", s)))
    .with(Modify::new(Row(..1)).with(str::to_lowercase));
```

IMPORTANT: you may need to specify type in your lambda otherwise compiler may be disagreed to work :)

There's 2 more Format modifiers. You can find more imformation about theire usage in the documentation.

- `FormatFrom` - Uses `Vec` elements as new content.
- `FormatWithIndex` - Like `Format` but with `row` and `column` index in lambda.

### Indent

The `Indent` type provides an interface for a left, right, top and bottom indent of cells.

```rust
Table::new(&data).with(Modify::new(Row(1..)).with(Indent::new(1, 1, 0, 2)));
```

### Max width

Using `MaxWidth` type its possible to set a max width of an object.
While tinkering content we don't forget about its color.

```rust
// Truncating content to 10 chars in all rows except a header.
Table::new(&data).with(Modify::new(Row(1..)).with(MaxWidth::truncating(10, "...")));

// Wrapping content by new lines after 10 chars in a header row.
Table::new(&data).with(Modify::new(Row(..1)).with(MaxWidth::wrapping(10)));
```

### Rotate

You can rotate table using `Rotate`.

Imagine you have a table already. And the output may look like this.

```text
┌────┬──────────────┬───────────────────────────┐
│ id │ destribution │ link                      │
├────┼──────────────┼───────────────────────────┤
│ 0  │ Fedora       │ https://getfedora.org/    │
├────┼──────────────┼───────────────────────────┤
│ 2  │ OpenSUSE     │ https://www.opensuse.org/ │
├────┼──────────────┼───────────────────────────┤
│ 3  │ Endeavouros  │ https://endeavouros.com/  │
└────┴──────────────┴───────────────────────────┘
```

Now we will add `table.with(Rotate::Left)` and the output will be;

```text
┌──────────────┬────────────────────────┬───────────────────────────┬──────────────────────────┐
│     link     │ https://getfedora.org/ │ https://www.opensuse.org/ │ https://endeavouros.com/ │
├──────────────┼────────────────────────┼───────────────────────────┼──────────────────────────┤
│ destribution │         Fedora         │         OpenSUSE          │       Endeavouros        │
├──────────────┼────────────────────────┼───────────────────────────┼──────────────────────────┤
│      id      │           0            │             2             │            3             │
└──────────────┴────────────────────────┴───────────────────────────┴──────────────────────────┘
```

### Disable

You can remove certain rows or columns from the table.

```rust
Table::new(&data)
    .with(Disable::Row(..1))
    .with(Disable::Column(3..4));
```

### Extract

You can `Extract` segments of a table to focus on a reduced number of rows and columns.

```rust
let rows = 1..3;
let columns = 1..;
Table::new(&data)
    .with(Extract::new(rows, columns));
```

```text
+-------+-------------+-----------+
|  i32  |    &str     |   bool    |
+-------+-------------+-----------+         +-------------+-----------+
| : 0 : | : Grodno :  | : true :  |         | : Grodno :  | : true :  |
+-------+-------------+-----------+    =    +-------------+-----------+
| : 1 : |  : Minsk :  | : true :  |         |  : Minsk :  | : true :  |
+-------+-------------+-----------+         +-------------+-----------+
| : 2 : | : Hamburg : | : false : |
+-------+-------------+-----------+
| : 3 : |  : Brest :  | : true :  |
+-------+-------------+-----------+
```

#### Refinishing

For styles with unique corner and edge textures it is possible to reapply a table style once a `Table` extract has been created.

```rust
let rows = 1..3;
let columns = 1..;
Table::new(&data)
    .with(Extract::new(rows, columns))
    .with(Style::modern());
```

```text
Raw extract
┼───────────────────────────┼──────────────────┼──────────────┤
│ The Dark Side of the Moon │ 01 March 1973    │ Unparalleled │
┼───────────────────────────┼──────────────────┼──────────────┤
│ Rumours                   │ 04 February 1977 │ Outstanding  │
┼───────────────────────────┼──────────────────┼──────────────┤

Refinished extract
┌───────────────────────────┬──────────────────┬───────────────┐
│ The Dark Side of the Moon │ 01 March 1973    │ Unparalleled  │
├───────────────────────────┼──────────────────┼───────────────┤
│ Rumours                   │ 04 February 1977 │  Outstanding  │
└───────────────────────────┴──────────────────┴───────────────┘
```


### Header and Footer

You can add a `Header` and `Footer` to display some information.
By the way you can even add such line by using `Panel`

```rust
Table::new(&data)
    .with(Header("Tabled Name"))
    .with(Footer(format!("{} elements", data.len())))
```

A look will differ from a style you choose.
But it's how it may look like.

```text
┌────────────────────────────────────────────────────────────┐
│                       Tabled Name                          │
├────────────────────────────────────────────────────────────┤
                            ...
├───────┼──────────────┼─────────┼───────────────────────────┤
│                        3 elements                          │
└────────────────────────────────────────────────────────────┘
```

### Concat

You can concatanate 2 tables using `Concat`.
It will stick 2 tables together either vertically or horizontally.

```rust
let t1: Table = ...;
let t2: Table = ...;

let t3: Table = t1.with(Concat::vertical(t2));
```

## Derive

To be able to use a `Tabled` macros each field must implement `std::fmt::Display`
otherwise it will not work.

The following example will cause a error.

```rust,compile_fail
use tabled::Tabled;
#[derive(Tabled)]
struct SomeType {
    field1: SomeOtherType,
}

struct SomeOtherType;
```

### Column name override

You can use a `#[header("")]` attribute to override a column name.

```rust
#[derive(Tabled)]
struct Person {
    #[header("Name")]
    first_name: &'static str,
    #[header("Surname")]
    last_name: &'static str,
}
```

### Hide a column

You can mark filds as hidden in which case they fill be ignored and not be present on a sheet.

A similar affect could be achived by the means of a `Disable` setting.

```rust
struct Person {
   #[header(hidden = true)]
   id: u8,
   #[header("field 2", hidden)]
   number: &'static str,
   name: &'static str,
}
```

### Custom field formatting

`#[derive(Tabled)]` is possible only when all fields implement a `Display` trait.

However, this may be often not the case for example when a field uses the `Option` type.

There's 2 common ways how to solve this:

* Implement `Tabled` trait manually for a type.
* Wrap `Option` to something like `DisplayedOption<T>(Option<T>)` and implement a Display trait for it.

Or to use an attribute `#[field(display_with = "func")]` for the field. To use it you must provide a function name in a `display_with` parameter.
   
```rust
fn display_option(o: &Option<bool>) -> String {
    match o {
        Some(s) => format!("is valid thing = {}", s), 
        None => format!("is not valid"),
    }
}

#[derive(Tabled)]
pub struct MyRecord {
    pub id: i64,
    #[field(display_with="display_option")]
    pub valid: Option<bool>
}
```

### Inline
   
It's possible to inline internal data if it implements `Tabled` trait.
Use `#[header(inline)]` or `#[header(inline("prefix>>"))]`.
The string argument is a prefix which will be used for all inlined elements.

```rust
 #[derive(Tabled)]
 struct Person {
     id: u8,
     name: &'static str,
     #[header(inline)]
     ed: Education,
 }
 
#[derive(Tabled)]
struct Education {
    uni: &'static str,
    graduated: bool,
}
```

And it works for enums as well.

```rust
#[derive(Tabled)]
enum Vehicle {
    #[header(inline("Auto::"))]
    Auto {
        model: &'static str,
        engine: &'static str,
    },
    #[header(inline)]
    Bikecycle(#[header("name")] &'static str, #[header(inline)] Bike),
}
        
#[derive(Tabled)]
struct Bike {
    brand: &'static str,
    price: f32,
}
```

## Features

### Color

The library doesn't bind you in usage of any color library but to be able to work corectly with color input you should provide a `--features color`.

```rust
Table::new(&data)
    .with(Style::psql())
    .with(Modify::new(Column(..1)).with(Format(|s| s.red().to_string())))
    .with(Modify::new(Column(1..2)).with(Format(|s| s.blue().to_string())))
    .with(Modify::new(Column(2..)).with(Format(|s| s.green().to_string())));
```

![carbon-2](https://user-images.githubusercontent.com/20165848/120526301-b95efc80-c3e1-11eb-8779-0ec48894463b.png)
   
### Tuple combination

You also can combine objets which implements `Tabled` by means of tuples, you will get a combined columns of them.

```rust
use tabled::{Tabled, Table, Style};

#[derive(Tabled)]
enum Domain {
    Security,
    Embeded,
    Frontend,
    Unknown,
}

#[derive(Tabled)]
struct Developer(#[header("name")] &'static str);

let data = vec![
    (Developer("Terri Kshlerin"), Domain::Embeded),
    (Developer("Catalina Dicki"), Domain::Security),
    (Developer("Jennie Schmeler"), Domain::Frontend),
    (Developer("Maxim Zhiburt"), Domain::Unknown),
];

let table = Table::new(data).with(Style::psql()).to_string();

assert_eq!(
    table,
    concat!(
        "      name       | Security | Embeded | Frontend | Unknown \n",
        "-----------------+----------+---------+----------+---------\n",
        " Terri Kshlerin  |          |    +    |          |         \n",
        " Catalina Dicki  |    +     |         |          |         \n",
        " Jennie Schmeler |          |         |    +     |         \n",
        "  Maxim Zhiburt  |          |         |          |    +    \n"
    )
);
```

### Object

You can peak your target for settings using `and` and `not` methods for an object.

```rust
Full.not(Row(..1)) // peak all cells except header
Head.and(Column(..1)).not(Cell(0, 0)) // peak a header and first column except a (0, 0) cell
```

## Views

`Tabled` supports not only Table view!

### Expanded display

You can use `ExpanedDisplay` if your data structure has a lot of fields.

Here's an example.

```rust
use tabled::{display::ExpandedDisplay, Tabled};

#[derive(Tabled)]
struct Distribution {
    name: &'static str,
    is_active: bool,
    is_cool: bool,
}

fn main() {
    let data = [
        Distribution {
            name: "Manjaro",
            is_cool: true,
            is_active: true,
        },
        Distribution {
            name: "Debian",
            is_cool: true,
            is_active: true,
        },
        Distribution {
            name: "Debian",
            is_cool: true,
            is_active: true,
        },
    ];

    let table = ExpandedDisplay::new(&data);

    println!("{}", table);
}
```

You'll see the following.

```text
-[ RECORD 0 ]------
name      | Manjaro
is_active | true
is_cool   | true
-[ RECORD 1 ]------
name      | Debian
is_active | true
is_cool   | true
-[ RECORD 2 ]------
name      | Debian
is_active | true
is_cool   | true
```

## Notes

### ANSI escape codes

By default `tabled` doesn't handle [ANSI escape codes](https://en.wikipedia.org/wiki/ANSI_escape_code).
By default such things as hyperlinks, blinking and others things which can be achived via ANSI codes might not work correctly.

`tabled` support it by setting a `color` feature.

```toml
tabled = { version = "*", features = ["color"] }
```

### Dynamic table

It might be hard to build a table using `Tabled` trait, if you have a data set which structure is determined at runtime.
In such situation you can use a `Builder`.

```rust
use tabled::{builder::Builder, Style};

fn main() {
    let table = Builder::default()
        .set_header(["Index", "Language"])
        .add_row(["1", "English"])
        .add_row(["2", "Deutsch"])
        .build()
        .with(Style::psql());

    println!("{}", table);
}
```

### Emoji
   
The library support emojies out of the box but be aware that some of the terminals and editors may not render them as you would expect.
   
Let's add emojies to an example from a [Usage](##Usage) section.

```rust
 let languages = vec![
     Language {
         name: "C 💕",
         designed_by: "Dennis Ritchie",
         invented_year: 1972,
     },
     Language {
         name: "Rust 👍",
         designed_by: "Graydon Hoare",
         invented_year: 2010,
     },
     Language {
         name: "Go 🧋",
         designed_by: "Rob Pike",
         invented_year: 2009,
     },
 ];
 ```

 The resultant table will look like the following.
 
 As you can see Github triks a bit a return table, but `GNOME terminal` and `Alacritty` terminal handles it correctly.
   
 ```rust
+---------+----------------+---------------+
|  name   |  designed_by   | invented_year |
+---------+----------------+---------------+
|  C 💕   | Dennis Ritchie |     1972      |
+---------+----------------+---------------+
| Rust 👍 | Graydon Hoare  |     2010      |
+---------+----------------+---------------+
|  Go 🧋  |    Rob Pike    |     2009      |
+---------+----------------+---------------+
```
