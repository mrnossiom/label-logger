//! A label theme for indicatif progress bars

use crate::{OutputLabel, pretty_output};
use indicatif::ProgressStyle;

/// Builds and return the theme for a progress bar
///
/// # Panics
/// If the template carefully built inside our crate is wrong
#[must_use]
pub fn label_theme(label: OutputLabel) -> ProgressStyle {
	let template = "[{bar:20}] {pos}/{len} {elapsed} {wide_msg}";
	let template = pretty_output(label, template);

	ProgressStyle::with_template(&template)
		.expect("our carefully built template is apparently wrong")
		.progress_chars("=> ")
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::OutputLabel;

	#[test]
	fn label_theme_template_is_not_broken() {
		let _theme = label_theme(OutputLabel::Success("Not broken"));
	}
}
