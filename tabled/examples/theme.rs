use tabled::{
    settings::{
        themes::{Layout, Theme},
        Alignment, Reverse, Style,
    },
    Table, Tabled,
};

#[derive(Tabled)]
struct Concept {
    name: String,
    desc: String,
}

impl Concept {
    fn new(name: &str, desc: &str) -> Self {
        Self {
            name: name.to_string(),
            desc: desc.to_string(),
        }
    }
}

fn main() {
    let data = vec![
        Concept::new("vnode", "A structure representing a filesystem entity like a file, directory, device node, etc at VFS abstraction level"),
        Concept::new("Physical block number", "In the context of FreeBSD filesystems layer we use this term when referring to fixed-size 512-byte blocks"),
        Concept::new("Logical block number", "Typical filesystems have a notion of filesystem blocks which may be constituted of multiple physical (512-byte) or media blocks"),
        Concept::new("Device vnode", "A filesystem is backed by some media that actually contains the data"),
        Concept::new("Buffer cache", "Buffer cache is a layer between filesystems and I/O code that performs actual media access via peripheral drivers"),
        Concept::new("struct bufobj", "struct bufobj represents a set of buffers belonging to the same abstract object"),
        Concept::new("struct buf", "struct buf represents a single buffer. In addition to holding identity of the buffer"),
        Concept::new("bread(9)", "Filesystem use bread (breadn, breada, etc) functions to access underlying data through a buffer cache layer"),
        Concept::new("VOP_BMAP", "VOP_BMAP translates a given logical block number within a given vnode to a physical (512-byte) block number within underlying media"),
        Concept::new("VOP_STRATEGY", "VOP_STRATEGY method fulfills an I/O request described by given struct buf object"),
    ];

    let mut table = Table::new(data);
    table.with(Theme::from_style(Style::ascii()));
    table.with(Reverse::rows(1, 0));
    table.with(Reverse::columns(0, 0));
    table.with(Layout::new(Alignment::top(), true));

    println!("{table}");
}
