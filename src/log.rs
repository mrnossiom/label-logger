use crate::util::shorten;
use console::{pad_str, Alignment, Color, Style, Term};
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
	Error,
	/// Outputs `Warning` in yellow
	Warning,
	/// Outputs the provided label in blue
	Info(&'a str),
	/// Outputs the provided label in green
	Success(&'a str),
	/// Outputs the provided label in the provided color
	Custom(&'a str, Color),
	/// Outputs the provided label in yellow
	Prompt(&'a str),
	/// Outputs a blank space with no label
	None,
}

/// Print a message with the specified label
pub fn println_label(label: OutputLabel, message: String) {
	match label {
		OutputLabel::Error => {
			eprintln!("{}", pretty_output(label, message, true));
		}
		_ => {
			println!("{}", pretty_output(label, message, true));
		}
	}
}

/// Print a message with a label, add a carriage return at the end and flush the stdout
pub fn print_r_label(label: OutputLabel, message: String) {
	print!("{}\r", pretty_output(label, message, true));

	stdout().flush().unwrap_or_else(|_| {
		println_label(OutputLabel::Error, "Could not flush stdout".to_string());
	});
}

/// Pretty a message with a given label and a given message colour
pub fn pretty_output(out_label: OutputLabel, message: String, fill_line: bool) -> String {
	let (label, label_color) = match out_label {
		OutputLabel::Error => (String::from("Error"), Color::Red),
		OutputLabel::Warning => (String::from("Warn"), Color::Yellow),
		OutputLabel::Info(info) => (String::from(info), Color::Blue),
		OutputLabel::Success(success) => (String::from(success), Color::Green),
		OutputLabel::Custom(custom, custom_colour) => (String::from(custom), custom_colour),
		OutputLabel::Prompt(prompt) => (String::from(prompt), Color::Yellow),
		OutputLabel::None => (String::from(""), Color::White),
	};

	match (*PAD_OUTPUT, out_label) {
		// Special case for piped output, none label adds a tabulation at the end
		(false, OutputLabel::None) => format!("\t{}", message),
		(false, _) => format!("{} {}", label, message),
		(true, _) => {
			// PAD_OUTPUT is false if there is no tty connected to stdout.
			// We should be able to use unwrap() here safely.
			let (term_width, _) = terminal_dimensions().unwrap();

			let label = Style::new()
				.fg(label_color)
				.bold()
				.apply_to(label)
				.to_string();

			let message = shorten(message, term_width - LABEL_WIDTH - 1);

			format!(
				"{} {}",
				pad_str(label.as_str(), LABEL_WIDTH, Alignment::Right, None),
				if fill_line {
					pad_str(
						message.as_str(),
						term_width - LABEL_WIDTH - 1,
						Alignment::Left,
						None,
					)
					.to_string()
				} else {
					message
				}
			)
		}
	}
}
