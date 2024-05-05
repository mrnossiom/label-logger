//! The actual implementation of the logger core

use crate::util::shorten;
use console::{pad_str, style, Alignment, StyledObject};
use once_cell::sync::OnceCell;
use std::{
	fmt::Display,
	io::{stdout, IsTerminal},
};
use term_size::dimensions as terminal_dimensions;

/// Checks if the output is piped and simplify the output for better debugging
pub static PAD_OUTPUT: OnceCell<bool> = OnceCell::new();

/// The maximum length of a log label
pub const LABEL_WIDTH: usize = 12;

/// The enum of possible output labels
#[derive(Default)]
pub enum OutputLabel<'a> {
	/// Outputs `Error` in red
	Error(&'a str),
	/// Outputs `Warning` in yellow
	Warning(&'a str),
	/// Outputs the provided label in blue
	Info(&'a str),
	/// Outputs the provided label in green
	Success(&'a str),
	/// Outputs the provided label in the provided color
	Custom(StyledObject<&'a str>),
	/// Outputs a blank space with no label
	#[default]
	None,
}

/// Print a message with the specified label
pub fn println_label<M: AsRef<str> + Display>(label: OutputLabel, message: M) {
	match label {
		OutputLabel::Error(_) => {
			eprintln!("{}", pretty_output(label, message));
		}
		_ => {
			println!("{}", pretty_output(label, message));
		}
	}
}

/// Pretty a message with a given label and a given message color
///
/// # Panics
/// We panic if we can't determine the width of the stdout `TTY`.
/// But it is only used in a part where we check that we are connected a real `TTY`
pub fn pretty_output<M: AsRef<str> + Display>(out_label: OutputLabel, message: M) -> String {
	let (label, label_is_empty) = match out_label {
		OutputLabel::Error(error) => (style(error).bold().red(), false),
		OutputLabel::Warning(warn) => (style(warn).bold().yellow(), false),
		OutputLabel::Info(info) => (style(info).bold().cyan(), false),
		OutputLabel::Success(success) => (style(success).bold().green(), false),
		OutputLabel::Custom(custom) => (custom, false),
		OutputLabel::None => (style(""), true),
	};

	// Pad output if the stdout is a tty
	if *PAD_OUTPUT.get_or_init(|| stdout().is_terminal()) {
		let (term_width, _) = terminal_dimensions().unwrap_or((80, 80));

		let message = shorten(message.to_string(), term_width - LABEL_WIDTH - 1);

		format!(
			"{} {}",
			pad_str(
				label.to_string().as_str(),
				LABEL_WIDTH,
				Alignment::Right,
				None
			),
			message
		)
	} else {
		// Special case for piped output, none label adds a tabulation at the start
		if label_is_empty {
			format!("\t{message}")
		} else {
			format!("{label} {message}")
		}
	}
}
