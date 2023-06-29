//! This example demonstrates usage of [`toml_to_table::TomlTable`]
//! with changed theme and in collapsed sight.

use tabled::settings::Style;
use toml_to_table::TomlTable;

fn main() {
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
    let mut table = TomlTable::new(&scene);
    table.collapse();
    table.with(Style::extended());

    println!("{table}");
}
