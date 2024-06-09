# A library for converting `ron` to a table.

It uses [`tabled`](https://github.com/zhiburt/tabled) as a rendering backend.

## Get started

The library supports 2 modes for a table embedded and collapsed.
It also provides with a list of options to modify the table, such as style, alignment, padding and more.

You can change an orientation of a `map` and `sequence` via `Orientation`.

You'll find to examples for the modes bellow.

## Usage

Add the library to a dependency list.

```toml
[dependencies]
ron_to_table = "0.1.0"
```

<table>
<tr>
<th> Example (embedded) </th>
<th> Result </th>
</tr>
<tr>
<td>

```rust
let data = r#"Scene(
    materials: {
        "metal": (reflectivity: 1.0),
        "plastic": (reflectivity: 0.5),
    },
    entities: [
        (name: "hero", material: "metal"),
        (name: "monster", material: "plastic"),
    ],
)"#;

let scene = ron::from_str(data).unwrap();
let table = ron_to_table::to_string(&scene);

println!("{}", table);
```

</td>
<td style="vertical-align: top;">

```text
+-------------+--------------------------------------------+
|  entities   | +----------------------------+             |
|             | | +------------+---------+   |             |
|             | | |  material  |  metal  |   |             |
|             | | +------------+---------+   |             |
|             | | |  name      |  hero   |   |             |
|             | | +------------+---------+   |             |
|             | +----------------------------+             |
|             | | +------------+-----------+ |             |
|             | | |  material  |  plastic  | |             |
|             | | +------------+-----------+ |             |
|             | | |  name      |  monster  | |             |
|             | | +------------+-----------+ |             |
|             | +----------------------------+             |
+-------------+--------------------------------------------+
|  materials  | +-----------+----------------------------+ |
|             | |  metal    | +----------------+-----+   | |
|             | |           | |  reflectivity  |  1  |   | |
|             | |           | +----------------+-----+   | |
|             | +-----------+----------------------------+ |
|             | |  plastic  | +----------------+-------+ | |
|             | |           | |  reflectivity  |  0.5  | | |
|             | |           | +----------------+-------+ | |
|             | +-----------+----------------------------+ |
+-------------+--------------------------------------------+
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
use ron_to_table::RonTable;
use tabled::settings::Style;

let data = r#"Scene(
    materials: {
        "metal": (reflectivity: 1.0),
        "plastic": (reflectivity: 0.5),
    },
    entities: [
        (name: "hero", material: "metal"),
        (name: "monster", material: "plastic"),
    ],
)"#;

let scene = ron::from_str(data).unwrap();
let table = RonTable::default()
    .collapse()
    .with(Style::extended())
    .build(&scene);

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
