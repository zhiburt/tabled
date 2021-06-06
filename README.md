[![Build Status](https://img.shields.io/travis/com/zhiburt/tabled/master?style=for-the-badge)](https://travis-ci.com/zhiburt/tabled)
[![Coverage Status](https://img.shields.io/coveralls/github/zhiburt/tabled/master?style=for-the-badge)](https://coveralls.io/repos/github/zhiburt/tabled/badge.svg?branch=branch)
[![Crate](https://img.shields.io/crates/v/tabled?style=for-the-badge)](https://crates.io/crates/tabled)
[![docs.rs](https://img.shields.io/docsrs/tabled?style=for-the-badge)](https://docs.rs/tabled/0.1.1/tabled/)
[![license](https://img.shields.io/crates/l/tabled?style=for-the-badge)](./LICENSE.txt)
[![dependency status](https://deps.rs/repo/github/zhiburt/tabled/status.svg?style=for-the-badge)](https://deps.rs/repo/github/zhiburt/tabled)

# tabled

An easy to use library for pretty print tables of Rust `struct`s and `enum`s.

# Agenda

* [Usage](#Usage)
    * [Derive information](#Derive-information)
* [Style](#Style)
    * [Styles](#Styles)
        * [Default](#Default)
        * [Psql](#Psql)
        * [GithubMarkdown](#GithubMarkdown)
        * [Pseudo](#Pseudo)
        * [PseudoClean](#PseudoClean)
        * [Noborder](#Noborder)
    * [Custom Style](#Custom-Style)
    * [Alignment](#Alignment)
    * [Format](#Format)
    * [Disable](#Disable)
    * [Color](#Color)
* [Features](#Features)
    * [Column name override](#Column-name-override)
    * [Hide a column](#Hide-a-column)
    * [Custom field formatting](#Custom-field-formatting)
    * [Tuple combination](#Tuple-combination)
    * [Object](#Object)
* [Notes](#Notes)
   * [Emoji](#Emoji)

# Usage

To print a list of structs or enums as a table.
Implement `Tabled` trait for your struct/enum with or annotate it with a `#[derive(Tabled)]` macro. Then call a `table` macro.

```rust
use tabled::{Tabled, table};

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

let table = table!(&languages);
let expected = "+------+----------------+---------------+\n\
                | name |  designed_by   | invented_year |\n\
                +------+----------------+---------------+\n\
                |  C   | Dennis Ritchie |     1972      |\n\
                +------+----------------+---------------+\n\
                | Rust | Graydon Hoare  |     2010      |\n\
                +------+----------------+---------------+\n\
                |  Go  |    Rob Pike    |     2009      |\n\
                +------+----------------+---------------+\n";

assert_eq!(expected, table);
```

## Derive information

To be able to use a `Tabled` macro each field should implement `std::fmt::Display`
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

Most of the default types implements the trait out of the box.

```rust
use tabled::table;
let some_numbers = [1, 2, 3];
let table = table!(&some_numbers);
```

# Style

## Styles

A list of ready to use styles. A shocases for the data in the [Usage](#Usage) section.
Styles can be chosen by passing a `Style` argument like this to `table!` macro.

```rust
let table = table!(&data, Style::psql());
```

### Default

```,
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

### Psql

```
 name |  designed_by   | invented_year 
------+----------------+---------------
  C   | Dennis Ritchie |     1972      
 Rust | Graydon Hoare  |     2010      
  Go  |    Rob Pike    |     2009      
```

### GithubMarkdown

```
| name |  designed_by   | invented_year |
|------+----------------+---------------|
|  C   | Dennis Ritchie |     1972      |
| Rust | Graydon Hoare  |     2010      |
|  Go  |    Rob Pike    |     2009      |
```

### Pseudo

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

### PseudoClean

```
┌──────┬────────────────┬───────────────┐
│ name │  designed_by   │ invented_year │
├──────┼────────────────┼───────────────┤
│  C   │ Dennis Ritchie │     1972      │
│ Rust │ Graydon Hoare  │     2010      │
│  Go  │    Rob Pike    │     2009      │
└──────┴────────────────┴───────────────┘
```

### Noborder

```
 name    designed_by     invented_year 
  C     Dennis Ritchie       1972      
  Rust   Graydon Hoare       2010      
  Go       Rob Pike          2009      
```

## Custom Style

You can modify existing styles to fits your needs.

```rust

table!(
   &data,
   tabled::Style::noborder()
      .frame_bottom(Some(Line::short('*', ' '')))
      .split(Some(Line::short(' ', ' ')))
      .inner(' ')
)
```

## Alignment

You can set a alignemt for a Header, Column, Row or All Cells.

```rust
table!(
    &data,
    Style::psql(),
    HorizontalAlignment(Full, Alignment::Left)
);
```

## Format

Format function provides an interface for a modification of cells.

```rust
let table = table!(
    &data,
    Style::psql(),
    Format(Column(..), |s| { format!("<< {} >>", s) }),
    Format(Row(..1), |s| { format!("Head {}", s) }),
);
```

## Disable

You can remove a certain rows or column from the table.

```rust
table!(&data, Disable::Row(..1), Disable::Column(3..4));
```

## Color

The library doesn't bind you in usage of any color library but to be able to work corectly with color input you should provide a `--features color`.
The folowing change on the script in the usage and it's result

```rust
let table = table!(
    &data,
    Style::psql(),
    Format(Column(..1), |s| { s.red().to_string() }),
    Format(Column(1..2), |s| { s.blue().to_string() }),
    Format(Column(2..), |s| { s.green().to_string() }),
);
```

![carbon-2](https://user-images.githubusercontent.com/20165848/120526301-b95efc80-c3e1-11eb-8779-0ec48894463b.png)

# Features

## Column name override

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

## Hide a column

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

## Custom field formatting

`#[derive(Tabled)]` is possible only when all fields implement a `Display` trait.
But it may be often not the case for example `Option` type.

There's 2 common ways how to solve it:

* Implement Tabled trait manually for a type.
* Wrap `Option` to something like DisplayedOption<T>(Option<T>) and implement a Display trait for it.

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

## Tuple combination

You also can combine objets which implements `Tabled` by means of tuples, you will get a combined columns of them.

```rust
use tabled::{Tabled, table, Style};

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
    
let table = table!(data, Style::psql());

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

## Object

You can peak your target for settings using `and` and `not` methods for an object.

```rust
Full.not(Row(..1)) // peak all cells except header
Head.and(Column(..1)).not(Cell(0, 0)) // peak a header and first column except a (0, 0) cell
```

## Notes

### Emoji
   
The library support emojies out of the box but be aware that some of the terminals and editors may not render them as you would expect.
   
Let's add emojies to an example from a [Usage](#Usage) section.

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
