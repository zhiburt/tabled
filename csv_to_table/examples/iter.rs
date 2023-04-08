//! This example demonstrates an alternative parsing option for [`csv_to_table`] translations.
//!
//! * The [`IterTable::sniff()`] formatting function is used to control how many
//! rows will be considered in determining column widths. Since the default sniff value is 1000,
//! this is helpful for controlling large dataset display outputs.

fn main() {
    let syscalls = "\
        0,INDIR,,\"int sys_syscall(int number, ...)\"\n\
        1,STD,,\"void sys_exit(int rval)\"\n\
        2,STD,,\"int sys_fork(void)\"\n\
        3,STD,NOLOCK,\"ssize_t sys_read(int fd, void *buf, size_t nbyte)\"\n\
        4,STD,NOLOCK,\"ssize_t sys_write(int fd, const void *buf, size_t nbyte)\"";

    let table = csv_to_table::iter::from_reader(syscalls.as_bytes()).sniff(3);

    table.build(std::io::stdout()).unwrap();
}
