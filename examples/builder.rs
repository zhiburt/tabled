//! The example can be run by this command
//! `cargo run --example builder`

fn main() {
    let table = tabled::builder::Builder::default()
        .header(["Name", "Issue"])
        .add_row(["sadbuttrueasfuck", "Complex dynamic table creation"])
        .build();

    println!("{}", table);
}
