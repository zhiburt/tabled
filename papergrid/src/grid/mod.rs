//! Module contains a list of backends for pritty print tables.

pub mod compact;

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub mod spanned;

// todo: maybe we could have something like
// trait Printer {
//     print_row();
//     print_horizonal();
//     print_vertical();
// }
//
// instead of the 2 grid implementations
//
// so we could have something like BordetTextPrinter<P> which would change the impl of original Printer.
//
// todo: The interface is not explicit enough.
