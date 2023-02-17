# A library for converting `csv` to a table.

It uses [`tabled`](https://github.com/zhiburt/tabled) as a rendering backend.

## Usage

There's 2 approaches the library provides.

- In memory apporach; where we load CSV into memory and then construct a table.
- Sniffing a csv; so the used memory will be limited.
- Setting your constrains so no memory will be used. 

Notice that 
<table>
<tr>
<th> Example of in memory approach </th>
<th> Result </th>
</tr>
<tr>
<td>

```rust
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
```

</td>
<td style="vertical-align: top;">

```text
+---+-------+--------+----------------------------------------------------------+
| 0 | INDIR |        | int sys_syscall(int number, ...)                         |
+---+-------+--------+----------------------------------------------------------+
| 1 | STD   |        | void sys_exit(int rval)                                  |
+---+-------+--------+----------------------------------------------------------+
| 2 | STD   |        | int sys_fork(void)                                       |
+---+-------+--------+----------------------------------------------------------+
| 3 | STD   | NOLOCK | ssize_t sys_read(int fd, void *buf, size_t nbyte)        |
+---+-------+--------+----------------------------------------------------------+
| 4 | STD   | NOLOCK | ssize_t sys_write(int fd, const void *buf, size_t nbyte) |
+---+-------+--------+----------------------------------------------------------+
```

</td>
</tr>
</table>

<table>
<tr>
<th> Example of sniffing approach </th>
<th> Result </th>
</tr>
<tr>
<td>

```rust
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
```

</td>
<td style="vertical-align: top;">

```text
+---+-------+--+----------------------------------+
| 0 | INDIR |  | int sys_syscall(int number, ...) |
+---+-------+--+----------------------------------+
| 1 | STD   |  | void sys_exit(int rval)          |
+---+-------+--+----------------------------------+
| 2 | STD   |  | int sys_fork(void)               |
+---+-------+--+----------------------------------+
| 3 | STD   |  | ssize_t sys_read(int fd, void *b |
+---+-------+--+----------------------------------+
| 4 | STD   |  | ssize_t sys_write(int fd, const  |
+---+-------+--+----------------------------------+
```

<h5> Notice that the last 2 rows are truncated. <h5>

</td>
</tr>
</table>
