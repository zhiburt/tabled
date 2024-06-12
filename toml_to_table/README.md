# A library for converting `toml` to a table.

It uses [`tabled`](https://github.com/zhiburt/tabled) as a rendering backend.

## Get started

The library supports 2 modes for a table embedded and collapsed.
It also provides with a list of options to modify the table, such as style, alignment, padding and more.

You can change an orientation of a `table` and `array` via `Orientation`.

You'll find to examples for the modes bellow.

## Usage

Add the library to a dependency list.

```toml
[dependencies]
toml_to_table = "0.1.0"
```

<table>
<tr>
<th> Example (embedded) </th>
<th> Result </th>
</tr>
<tr>
<td>

```rust
let data = r#"
[materials]
metal = { reflectivity = 1.0 }
plastic = { reflectivity = 0.5 }

[[entities]]
name = "hero"
material = "metal"

[[entities]]
name = "monster"
material = "plastic"
"#;

let scene = toml::from_str(data).unwrap();

let table = toml_to_table::to_string(&scene);

println!("{}", table);
```

</td>
<td style="vertical-align: top;">

```text
+-----------+----------------------------------------+
| entities  | +--------------------------+           |
|           | | +----------+---------+   |           |
|           | | | material |  metal  |   |           |
|           | | +----------+---------+   |           |
|           | | | name     |  hero   |   |           |
|           | | +----------+---------+   |           |
|           | +--------------------------+           |
|           | | +----------+-----------+ |           |
|           | | | material |  plastic  | |           |
|           | | +----------+-----------+ |           |
|           | | | name     |  monster  | |           |
|           | | +----------+-----------+ |           |
|           | +--------------------------+           |
+-----------+----------------------------------------+
| materials | +---------+--------------------------+ |
|           | | metal   | +--------------+-----+   | |
|           | |         | | reflectivity |  1  |   | |
|           | |         | +--------------+-----+   | |
|           | +---------+--------------------------+ |
|           | | plastic | +--------------+-------+ | |
|           | |         | | reflectivity |  0.5  | | |
|           | |         | +--------------+-------+ | |
|           | +---------+--------------------------+ |
+-----------+----------------------------------------+
```

</td>
</tr>
</table>

<table>
<tr>
<th> Example (collapsed) </th>
<th> Result </th>
</tr>
<tr>
<td>

```rust
use toml_to_table::TomlTable;
use tabled::settings::Style;

let data = r#"
[materials]
metal = { reflectivity = 1.0 }
plastic = { reflectivity = 0.5 }

[[entities]]
name = "hero"
material = "metal"

[[entities]]
name = "monster"
material = "plastic"
"#;

let scene = toml::from_str(data).unwrap();
let table = TomlTable::new(&scene)
    .collapse()
    .with(Style::extended())
    .to_string();

println!("{table}");
```

</td>
<td style="vertical-align: top;">

```text
╔═══════════╦══════════╦═══════════════════╗
║ entities  ║ material ║ metal             ║
║           ╠══════════╬═══════════════════╣
║           ║ name     ║ hero              ║
║           ╠══════════╬═══════════════════╣
║           ║ material ║ plastic           ║
║           ╠══════════╬═══════════════════╣
║           ║ name     ║ monster           ║
╠═══════════╬═════════╦╩═════════════╦═════╣
║ materials ║ metal   ║ reflectivity ║ 1   ║
║           ╠═════════╬══════════════╬═════╣
║           ║ plastic ║ reflectivity ║ 0.5 ║
╚═══════════╩═════════╩══════════════╩═════╝
```
