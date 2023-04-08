# A library for converting `json` to a table.

It uses [`tabled`](https://github.com/zhiburt/tabled) as a rendering backend.

## Usage

Add the library to a dependency list.

```toml
[dependencies]
json_to_table = "*"
```

The main and only function you shall use to build a table is `json_to_table`.

<table>
<tr>
<th> Example </th>
<th> Result </th>
</tr>
<tr>
<td>

```rust
use json_to_table::json_to_table;
use serde_json::json;

fn main() {
    let value = json!(
        [
            {
                "name": "Aleix Melon",
                "id": "E00245",
                "role": ["Dev", "DBA"],
                "age": 23,
                "doj": "11-12-2019",
                "married": false,
                "address": {
                    "street": "32, Laham St.",
                    "city": "Innsbruck",
                    "country": "Austria"
                    },
                "referred-by": "E0012"
            },
        ]
    );

    let table = json_to_table(&value).to_string();

    println!("{}", table)
}
```

</td>
<td style="vertical-align: top;">

```text
+-------------------------------------------------+
| +-------------+-------------------------------+ |
| | address     | +---------+-----------------+ | |
| |             | | city    |  Innsbruck      | | |
| |             | +---------+-----------------+ | |
| |             | | country |  Austria        | | |
| |             | +---------+-----------------+ | |
| |             | | street  |  32, Laham St.  | | |
| |             | +---------+-----------------+ | |
| +-------------+-------------------------------+ |
| | age         |  23                           | |
| +-------------+-------------------------------+ |
| | doj         |  11-12-2019                   | |
| +-------------+-------------------------------+ |
| | id          |  E00245                       | |
| +-------------+-------------------------------+ |
| | married     |  false                        | |
| +-------------+-------------------------------+ |
| | name        |  Aleix Melon                  | |
| +-------------+-------------------------------+ |
| | referred-by |  E0012                        | |
| +-------------+-------------------------------+ |
| | role        | +-------+                     | |
| |             | |  Dev  |                     | |
| |             | +-------+                     | |
| |             | |  DBA  |                     | |
| |             | +-------+                     | |
| +-------------+-------------------------------+ |
+-------------------------------------------------+
```

</td>
</tr>
</table>

You can also build a table in a squash mode.

<table>
<tr>
<th> Example </th>
<th> Result </th>
</tr>
<tr>
<td>

```rust
use json_to_table::json_to_table;
use serde_json::json;

fn main() {
    let value = json!(
        [
            {
                "name": "Aleix Melon",
                "id": "E00245",
                "role": ["Dev", "DBA"],
                "age": 23,
                "doj": "11-12-2019",
                "married": false,
                "address": {
                    "street": "32, Laham St.",
                    "city": "Innsbruck",
                    "country": "Austria"
                    },
                "referred-by": "E0012"
            },
        ]
    );

    let table = json_to_table(&value).collapse().to_string();

    println!("{}", table)
}
```

</td>
<td style="vertical-align: top;">

```text
+-------------+---------+---------------+
| address     | city    | Innsbruck     |
|             +---------+---------------+
|             | country | Austria       |
|             +---------+---------------+
|             | street  | 32, Laham St. |
+-------------+---------+---------------+
| age         | 23                      |
+-------------+-------------------------+
| doj         | 11-12-2019              |
+-------------+-------------------------+
| id          | E00245                  |
+-------------+-------------------------+
| married     | false                   |
+-------------+-------------------------+
| name        | Aleix Melon             |
+-------------+-------------------------+
| referred-by | E0012                   |
+-------------+-------------------------+
| role        | Dev                     |
|             +-------------------------+
|             | DBA                     |
+-------------+-------------------------+
```

</td>
</tr>
</table>

You can chose how to build an `Array` and `Object` via `Orientation`.

<table>
<tr>
<th> Example </th>
<th> Result </th>
</tr>
<tr>
<td>

```rust
use json_to_table::{json_to_table, Orientation};
use serde_json::json;

fn main() {
    let value = json!(
        [
            {
                "name": "Aleix Melon",
                "role": ["Dev", "DBA"],
                "age": 23,
                "referred-by": "E0012"
            },
            {
                "name": "Aleix Melon",
                "role": ["DBA"],
                "age": 24,
                "referred-by": "E0012"
            },
        ]
    );

    let table = json_to_table(&value)
        .set_object_mode(Orientation::Row)
        .to_string();

    println!("{}", table)
}
```

</td>
<td style="vertical-align: top;">

```text
+----------------------------------------------------+
| +------+---------------+-------------+-----------+ |
| | age  | name          | referred-by | role      | |
| +------+---------------+-------------+-----------+ |
| |  23  |  Aleix Melon  |  E0012      | +-------+ | |
| |      |               |             | |  Dev  | | |
| |      |               |             | +-------+ | |
| |      |               |             | |  DBA  | | |
| |      |               |             | +-------+ | |
| +------+---------------+-------------+-----------+ |
+----------------------------------------------------+
| +------+---------------+-------------+-----------+ |
| | age  | name          | referred-by | role      | |
| +------+---------------+-------------+-----------+ |
| |  24  |  Aleix Melon  |  E0012      | +-------+ | |
| |      |               |             | |  DBA  | | |
| |      |               |             | +-------+ | |
| +------+---------------+-------------+-----------+ |
+----------------------------------------------------+
```

</td>
</tr>
</table>
