// TODO: add logo
// #![doc(html_favicon_url = "https://example.com/favicon.ico")]
// #![doc(html_logo_url = "https://example.com/logo.jpg")]

//! A Cargo-like logging library.
//!
//! # Usage
//!
//! ```
//! #[macro_use]
//! extern crate label_logger;
//!
//! use label_logger::OutputLabel;
//!
//! fn main() {
//!     // Log what you want.
//!     println!(OutputLabel::Success("Hello"), "world");
//!     println!(OutputLabel::Error, "Bye, {}!", "program");
//! }
//! ```

#[macro_use]
extern crate lazy_static;

mod log;
mod macros;

// Re-exports
pub use crate::log::{pretty_output, print_r_label, println_label, OutputLabel};
pub use crate::macros::*;
pub use console;
