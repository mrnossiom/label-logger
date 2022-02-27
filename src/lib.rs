//! A Cargo-like logging library.
//!
//! # Usage
//!
//! ```rust
//! #[macro_use]
//! extern crate label_logger;
//! use label_logger::{init_logger, OutputLabel};
//!
//! fn main() {
//! 	// This step is not necessary but it is recommended.
//! 	// It simplifies the output when necessary and ensures color support.
//! 	init_logger();
//!
//! 	// Log what you want.
//! 	println!(OutputLabel::Info, "Hello, {}!", "world");
//! 	println!(OutputLabel::Error, "Bye, {}!", "program");
//! }

#![feature(decl_macro)]

mod log;
mod macros;
mod util;

use crate::log::SIMPLIFY_OUTPUT;
use atty::Stream;
use std::sync::atomic::Ordering;

// Re-exports
pub use crate::log::{pretty_output, print_label, print_r_label, println_label, OutputLabel};
pub use crate::macros::*;
pub use yansi;

/// This checks if colors can be enabled on windows.
/// It also checks if the output is piped and simplify the output for better debugging
pub fn init_logger() {
	// Enable coloring on Windows if possible
	#[cfg(windows)]
	{
		use yansi::Paint;

		if !Paint::enable_windows_ascii() {
			Paint::disable();
		}
	}

	// If the output is piped don't trim output
	SIMPLIFY_OUTPUT.store(
		atty::is(Stream::Stdin) && atty::isnt(Stream::Stdout),
		Ordering::Relaxed,
	);
}
