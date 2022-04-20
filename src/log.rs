//! The actual implementation of the logger core

use crate::util::shorten;
use console::{pad_str, Alignment, Style, Term};
use std::io::{stdout, Write};
use term_size::dimensions as terminal_dimensions;

// This checks if colors can be enabled on windows.
// It also checks if the output is piped and simplify the output for better debugging
lazy_static! {
	pub static ref PAD_OUTPUT: bool = {
		// Pad output if the stdout is a tty
		Term::stdout().is_term()
	};
}

/// The maximum length of a log label
pub const LABEL_WIDTH: usize = 12;

/// The enum of possible output labels
pub enum OutputLabel<'a> {
	/// Outputs `Error` in red
	Error(&'a str),
	/// Outputs `Warning` in yellow
	Warning(&'a str),
	/// Outputs the provided label in blue
	Info(&'a str),
	/// Outputs the provided label in green
	Success(&'a str),
	// TODO: Custom accept styled object instead of string + color
	/// Outputs the provided label in the provided color
	Custom(&'a str),
	/// Outputs a blank space with no label
	None,
}

/// Print a message with the specified label
pub fn println_label(label: OutputLabel, message: String) {
	match label {
		OutputLabel::Error(_) => {
			eprintln!("{}", pretty_output(label, message));
		}
		_ => {
			println!("{}", pretty_output(label, message));
		}
	}
}

/// Print a message with a label, add a carriage return at the end and flush the stdout
pub fn print_r_label(label: OutputLabel, message: String) {
	print!("{}\r", pretty_output(label, message));

	stdout().flush().unwrap_or_else(|_| {
		println_label(
			OutputLabel::Error("Error"),
			"Could not flush stdout".to_string(),
		);
	});
}

/// Pretty a message with a given label and a given message color
pub fn pretty_output(out_label: OutputLabel, message: String) -> String {
	let label = match out_label {
		OutputLabel::Error(error) => Style::new().bold().red().apply_to(error),
		OutputLabel::Warning(warn) => Style::new().bold().yellow().apply_to(warn),
		OutputLabel::Info(info) => Style::new().bold().blue().apply_to(info),
		OutputLabel::Success(success) => Style::new().bold().green().apply_to(success),
		OutputLabel::Custom(custom) => Style::new().apply_to(custom),
		OutputLabel::None => Style::new().apply_to(""),
	};

	match (*PAD_OUTPUT, out_label) {
		// Special case for piped output, none label adds a tabulation at the start
		(false, OutputLabel::None) => format!("\t{}", message),
		(false, _) => format!("{} {}", label, message),
		(true, _) => {
			// PAD_OUTPUT is false if there is no tty connected to stdout.
			// We should be able to use unwrap() here safely.
			let (term_width, _) = terminal_dimensions().unwrap();

			let message = shorten(message, term_width - LABEL_WIDTH - 1);

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
		}
	}
}
