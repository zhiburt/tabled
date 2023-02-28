//! The example can be run by this command
//! `cargo run --example csv_table`

fn main() {
    let syscalls = "\
        0,INDIR,,\"int sys_syscall(int number, ...)\"\n\
        1,STD,,\"void sys_exit(int rval)\"\n\
        2,STD,,\"int sys_fork(void)\"\n\
        3,STD,NOLOCK,\"ssize_t sys_read(int fd, void *buf, size_t nbyte)\"\n\
        4,STD,NOLOCK,\"ssize_t sys_write(int fd, const void *buf, size_t nbyte)\"";

    let table = csv_to_table::from_reader(syscalls.as_bytes()).unwrap();

    println!("{table}")
}
