# `table_to_html`

Provides a interface to build HTML `<table>`.

# Get started

```rust
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
```

![<table>](https://user-images.githubusercontent.com/20165848/219793600-9e685a3f-0e27-4c37-bebb-d9bdef198b33.png)
