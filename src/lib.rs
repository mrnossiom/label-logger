#![warn(clippy::missing_docs_in_private_items)]
// TODO: add logo
// #![doc(html_favicon_url = "https://example.com/favicon.ico")]
// #![doc(html_logo_url = "https://example.com/logo.jpg")]
#![doc = include_str!("../README.md")]

#[macro_use]
extern crate lazy_static;

mod log;
#[macro_use]
mod macros;
#[cfg(feature = "dialoguer")]
mod theme;
mod util;

// Re-exports
pub use crate::log::{pretty_output, println_label, OutputLabel};
#[cfg(feature = "dialoguer")]
pub use crate::theme::LabelTheme;
pub use console;
#[cfg(feature = "dialoguer")]
pub use dialoguer;
