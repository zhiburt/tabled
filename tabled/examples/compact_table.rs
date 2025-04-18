use tabled::{settings::style::Style, tables::CompactTable};

fn main() {
    let data = [
        ["Debian", "1.1.1.1", "true"],
        ["Arch", "127.1.1.1", "true"],
        ["Manjaro", "Arch", "true"],
        ["Manjaro", "A\nr\nc\nh", "true"],
    ];

    let table = CompactTable::from(data).with(Style::ascii());

    #[cfg(feature = "std")]
    println!("{}", table.to_string());
}
