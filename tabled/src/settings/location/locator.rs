use super::{by_value::ByValue, ByColumnName, ByCondition, ByContent};

/// An abstract factory for locations, to be used to find different things on the table.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Locator;

impl Locator {
    /// Constructs a new location searcher for a cells with a given content.
    ///
    /// # Example
    ///
    /// ```
    /// use tabled::{
    ///     settings::location::Locator,
    ///     assert::assert_table,
    ///     Table, Tabled,
    /// };
    ///
    /// #[derive(Tabled)]
    /// struct Reading {
    ///     link: &'static str,
    ///     comment: &'static str,
    /// }
    ///
    /// let data = [
    ///     Reading { link: "https://www.gnu.org/software/grub/manual/multiboot/multiboot.html", comment: "todo" },
    ///     Reading { link: "https://wiki.debian.org/initramfs", comment: "todo" },
    ///     Reading { link: "http://jdebp.uk/FGA/efi-boot-process.html", comment: "..." },
    ///     Reading { link: "https://wiki.debian.org/UEFI", comment: "" },
    /// ];
    ///
    /// let mut table = Table::new(data);
    /// table.modify(Locator::content("todo"), "todo-soon");
    ///
    /// assert_table!(
    ///     table,
    ///     "+-------------------------------------------------------------------+-----------+"
    ///     "| link                                                              | comment   |"
    ///     "+-------------------------------------------------------------------+-----------+"
    ///     "| https://www.gnu.org/software/grub/manual/multiboot/multiboot.html | todo-soon |"
    ///     "+-------------------------------------------------------------------+-----------+"
    ///     "| https://wiki.debian.org/initramfs                                 | todo-soon |"
    ///     "+-------------------------------------------------------------------+-----------+"
    ///     "| http://jdebp.uk/FGA/efi-boot-process.html                         | ...       |"
    ///     "+-------------------------------------------------------------------+-----------+"
    ///     "| https://wiki.debian.org/UEFI                                      |           |"
    ///     "+-------------------------------------------------------------------+-----------+"
    /// );
    /// ```
    pub fn content<S>(text: S) -> ByContent<S>
    where
        S: AsRef<str>,
    {
        ByContent::new(text)
    }

    /// Constructs a new location searcher for a column by its header.
    ///
    /// # Example
    ///
    /// ```
    /// use tabled::{
    ///     settings::{location::Locator, Width},
    ///     assert::assert_table,
    ///     Table, Tabled,
    /// };
    ///
    /// #[derive(Tabled)]
    /// struct Reading {
    ///     link: &'static str,
    ///     comment: &'static str,
    /// }
    ///
    /// let data = [
    ///     Reading { link: "https://www.gnu.org/software/grub/manual/multiboot/multiboot.html", comment: "todo" },
    ///     Reading { link: "https://wiki.debian.org/initramfs", comment: "todo" },
    ///     Reading { link: "http://jdebp.uk/FGA/efi-boot-process.html", comment: "..." },
    ///     Reading { link: "https://wiki.debian.org/UEFI", comment: "" },
    /// ];
    ///
    /// let mut table = Table::new(data);
    /// table.modify(Locator::column("link"), Width::truncate(10));
    ///
    /// assert_table!(
    ///     table,
    ///     "+------------+---------+"
    ///     "| link       | comment |"
    ///     "+------------+---------+"
    ///     "| https://ww | todo    |"
    ///     "+------------+---------+"
    ///     "| https://wi | todo    |"
    ///     "+------------+---------+"
    ///     "| http://jde | ...     |"
    ///     "+------------+---------+"
    ///     "| https://wi |         |"
    ///     "+------------+---------+"
    /// );
    /// ```
    pub fn column<S>(text: S) -> ByColumnName<S>
    where
        S: AsRef<str>,
    {
        ByColumnName::new(text)
    }

    /// Constructs a new location searcher with a specified condition closure.
    ///
    /// Return `true` if it shall be included in output.
    /// Otherwise return `false`.
    ///  
    /// # Example
    ///
    /// ```
    /// use tabled::{
    ///     settings::{location::Locator, Width},
    ///     assert::assert_table,
    ///     Table, Tabled,
    /// };
    ///
    /// #[derive(Tabled)]
    /// struct Reading {
    ///     link: &'static str,
    ///     comment: &'static str,
    /// }
    ///
    /// let data = [
    ///     Reading { link: "https://www.gnu.org/software/grub/manual/multiboot/multiboot.html", comment: "todo" },
    ///     Reading { link: "https://wiki.debian.org/initramfs", comment: "todo" },
    ///     Reading { link: "http://jdebp.uk/FGA/efi-boot-process.html", comment: "..." },
    ///     Reading { link: "https://wiki.debian.org/UEFI", comment: "" },
    /// ];
    ///
    /// let mut table = Table::new(data);
    /// table.modify(Locator::by(|text| text.len() > 33), Width::truncate(33));
    ///
    /// assert_table!(
    ///     table,
    ///     "+-----------------------------------+---------+"
    ///     "| link                              | comment |"
    ///     "+-----------------------------------+---------+"
    ///     "| https://www.gnu.org/software/grub | todo    |"
    ///     "+-----------------------------------+---------+"
    ///     "| https://wiki.debian.org/initramfs | todo    |"
    ///     "+-----------------------------------+---------+"
    ///     "| http://jdebp.uk/FGA/efi-boot-proc | ...     |"
    ///     "+-----------------------------------+---------+"
    ///     "| https://wiki.debian.org/UEFI      |         |"
    ///     "+-----------------------------------+---------+"
    /// );
    /// ```
    pub fn by<F>(condition: F) -> ByCondition<F>
    where
        F: Fn(&str) -> bool,
    {
        ByCondition::new(condition)
    }

    /// Constructs a new location searcher which finds only all values equal to the top.
    ///
    /// Return `true` if it shall be counted.
    /// Otherwise return `false`.
    ///
    /// # Example
    ///
    /// ```
    /// use tabled::{
    ///     settings::{location::Locator, Width, Format},
    ///     assert::assert_table,
    ///     Table, Tabled,
    /// };
    ///
    /// #[derive(Tabled)]
    /// struct Reading {
    ///     link: &'static str,
    ///     comment: &'static str,
    /// }
    ///
    /// let data = [
    ///     Reading { link: "https://www.gnu.org/software/grub/manual/multiboot/multiboot.html", comment: "todo" },
    ///     Reading { link: "https://wiki.debian.org/initramfs", comment: "todo" },
    ///     Reading { link: "http://jdebp.uk/FGA/efi-boot-process.html", comment: "..." },
    ///     Reading { link: "https://wiki.debian.org/UEFI", comment: "" },
    /// ];
    ///
    /// let mut table = Table::new(data);
    /// table.modify(
    ///     Locator::value(Locator::column("link"), |link1, link2| link1.len() > link2.len()),
    ///     Format::content(|s| format!("[ {s} ]")),
    /// );
    ///
    /// assert_table!(
    ///     table,
    ///     "+-----------------------------------------------------------------------+---------+"
    ///     "| link                                                                  | comment |"
    ///     "+-----------------------------------------------------------------------+---------+"
    ///     "| [ https://www.gnu.org/software/grub/manual/multiboot/multiboot.html ] | todo    |"
    ///     "+-----------------------------------------------------------------------+---------+"
    ///     "| https://wiki.debian.org/initramfs                                     | todo    |"
    ///     "+-----------------------------------------------------------------------+---------+"
    ///     "| http://jdebp.uk/FGA/efi-boot-process.html                             | ...     |"
    ///     "+-----------------------------------------------------------------------+---------+"
    ///     "| https://wiki.debian.org/UEFI                                          |         |"
    ///     "+-----------------------------------------------------------------------+---------+"
    /// );
    /// ```
    pub fn value<O, F>(search: O, condition: F) -> ByValue<O, F>
    where
        F: Fn(&str, &str) -> bool,
    {
        ByValue::new(search, condition)
    }
}
