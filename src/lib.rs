#![doc(
	html_logo_url = "https://raw.githubusercontent.com/mrnossiom/label-logger/main/assets/logo-squared.png"
)]
#![doc = include_str!("../README.md")]

#[cfg(feature = "dialoguer")]
pub mod dialoguer;
#[cfg(feature = "indicatif")]
pub mod indicatif;
mod log;
mod macros;
mod util;

// Re-exports
#[cfg(feature = "dialoguer")]
pub use crate::dialoguer::LabelTheme;
#[cfg(feature = "indicatif")]
pub use crate::indicatif::label_theme;
pub use crate::log::{OutputLabel, pretty_output, println_label};
pub use console;
