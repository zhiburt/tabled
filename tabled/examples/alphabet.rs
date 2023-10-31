//! This example demonstrates instantiating a [`Table`] from an [`IntoIterator`] compliant object.
//!
//! * Note how [`Range`] [expression syntax](https://doc.rust-lang.org/reference/expressions/range-expr.html)
//! is used to idiomatically represent the English alphabet.

fn main() {
    use tabled::{
        settings::{
            location::Locator,
            object::{Columns, Object},
            Alignment, Modify, Padding,
        },
        Table, Tabled,
    };

    #[derive(Tabled)]
    struct Reading {
        link: &'static str,
        comment: &'static str,
    }

    let data = [
        Reading {
            link: "https://www.gnu.org/software/grub/manual/multiboot/multiboot.html",
            comment: "todo",
        },
        Reading {
            link: "https://wiki.debian.org/initramfs",
            comment: "todo",
        },
        Reading {
            link: "http://jdebp.uk/FGA/efi-boot-process.html",
            comment: "todo,2",
        },
        Reading {
            link: "https://wiki.debian.org/UEFI",
            comment: "todo,2",
        },
    ];

    let mut table = Table::new(data);
    table.with(Padding::zero());
    table.with(Modify::new(Locator::column("link")).with(Alignment::right()));
    table.with(Modify::new(Locator::content("todo")).with("todo,1"));
    table.with(
        Modify::new(Columns::single(1).intersect(Locator::by(|text| text.contains("todo"))))
            .with(Padding::new(4, 0, 0, 0)),
    );

    let output = table.to_string();

    println!("{output}");
}
