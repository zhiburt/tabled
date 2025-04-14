use tabled::{
    settings::{themes::Layout, Alignment, Style},
    Table, Tabled,
};

#[derive(Tabled)]
struct Concept<'a>(
    #[tabled(rename = "name")] &'a str,
    #[tabled(rename = "desc")] &'a str,
);

fn main() {
    let data = vec![
        Concept("vnode", "A structure representing a filesystem entity like a file, directory, device node, etc at VFS abstraction level"),
        Concept("struct bufobj", "struct bufobj represents a set of buffers belonging to the same abstract object"),
        Concept("struct buf", "struct buf represents a single buffer. In addition to holding identity of the buffer"),
        Concept("bread(9)", "Filesystem use bread (breadn, breada, etc) functions to access underlying data through a buffer cache layer"),
        Concept("VOP_BMAP", "VOP_BMAP translates a given logical block number within a given vnode to a physical (512-byte) block number within underlying media"),
        Concept("VOP_STRATEGY", "VOP_STRATEGY method fulfills an I/O request described by given struct buf object"),
    ];

    let mut table = Table::new(data);
    table.with(Style::ascii());
    table.with(Layout::new(Alignment::top(), true));

    println!("{table}");
}
