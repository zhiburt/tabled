//! # The table
//!
//! ```
#![doc = static_table::static_table!([
    ["a", "b", "result"],
    ["1", '2', '3'],
    ["2", '2', '4']
])]
//! ```

/// Add joins 2 integers together to get a sum.
/// 
/// ```
#[doc = static_table::static_table!([
    ["a", "b", "result"],
    ["1", '2', '3'],
    ["2", '2', '4']
])]
/// ```
pub fn add(left: usize, right: usize) -> usize {
    left + right
}
