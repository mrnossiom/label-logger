#![warn(clippy::missing_docs_in_private_items)]
#![doc(html_logo_url = "https://raw.githubusercontent.com/MrNossiom/label-logger/main/logo.png")]
#![doc = include_str!("../README.md")]

#[cfg(feature = "indicatif")]
pub mod indicatif;
mod log;
mod macros;
#[cfg(feature = "dialoguer")]
mod theme;
mod util;

// Re-exports
#[cfg(feature = "indicatif")]
pub use crate::indicatif::label_theme;
pub use crate::log::{pretty_output, println_label, OutputLabel};
pub use crate::macros::*;
#[cfg(feature = "dialoguer")]
pub use crate::theme::LabelTheme;
pub use console;
#[cfg(feature = "dialoguer")]
pub use dialoguer;
