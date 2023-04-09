//! This example demonstrates using [`HtmlTable`] to easily transform a multi-dimensional array
//! into web-friendly [html](https://developer.mozilla.org/en-US/docs/Web/HTML).
//!
//! * Note how [`HtmlTable::set_border()`] is used to customize the output markup.
//! These changes are implemented through a prepended [style](https://developer.mozilla.org/en-US/docs/Web/css)
//! section above the opening table tag.
//! * Customization options include:
//!     * Border
//!     * Alignment
//!     * Column and Row span
//!     * Margin and Padding

use table_to_html::HtmlTable;

fn main() {
    #[rustfmt::skip]
    let data = vec![
        ["0",  "INDIR", "",       "int sys_syscall(int number, ...)"],
        ["1",  "STD",   "",       "void sys_exit(int rval)"],
        ["2",  "STD",   "",       "int sys_fork(void)"],
        ["3",  "STD",   "NOLOCK", "ssize_t sys_read(int fd, void *buf, size_t nbyte)"],
        ["4",  "STD",   "NOLOCK", "ssize_t sys_write(int fd, const void *buf, size_t nbyte)"],
    ];

    let mut table = HtmlTable::new(data);
    table.set_border(3);

    println!("{table}")
}
