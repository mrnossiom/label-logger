#![warn(clippy::missing_docs_in_private_items)]
#![doc(html_logo_url = "https://raw.githubusercontent.com/MrNossiom/label-logger/main/logo.png")]
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
