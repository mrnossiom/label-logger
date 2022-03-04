//! A Cargo-like logging library.
//!
//! # Usage
//!
//! ```rust
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

#![feature(decl_macro)]

#[macro_use]
extern crate lazy_static;

mod log;
mod macros;
mod util;

// Re-exports
pub use crate::log::{pretty_output, print_r_label, println_label, OutputLabel};
pub use crate::macros::*;
pub use yansi;
