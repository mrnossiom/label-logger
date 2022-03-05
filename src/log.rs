use console::{pad_str, Alignment, Term};
use std::io::{stdout, Write};
use term_size::dimensions as terminal_dimensions;
use yansi::{Color, Paint, Style};

// This checks if colors can be enabled on windows.
// It also checks if the output is piped and simplify the output for better debugging
lazy_static! {
	// TODO: rename to pad_output and change the code accordingly once moved to console crate
	pub static ref SIMPLIFY_OUTPUT: bool = {
		// Enable coloring on Windows if possible
		#[cfg(windows)]
		if !Paint::enable_windows_ascii() {
			Paint::disable();
		}

		// If the output is piped disable color and simplify output
		if !Term::stdout().is_term() {
			Paint::disable();
			return true;
		}

		false
	};
}

// The maximum length of a log label
pub const LABEL_WIDTH: usize = 12;

// TODO: document each possible label
// TODO: make the macro calls less verbose
/// The enum of possible output labels
pub enum OutputLabel<'a> {
	Error,
	Warning,
	Info(&'a str),
	Success(&'a str),
	Custom(&'a str, Color),
	Prompt(&'a str),
	None,
}

/// Print a message with the specified label
pub fn println_label(label: OutputLabel, message: String) {
	match label {
		OutputLabel::Error => {
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
		println_label(OutputLabel::Error, "Could not flush stdout".to_string());
	});
}

/// Pretty a message with a given label and a given message colour
pub fn pretty_output(label: OutputLabel, message: String) -> String {
	let (label, label_color) = match label {
		OutputLabel::Error => (String::from("Error"), Color::Red),
		OutputLabel::Warning => (String::from("Warn"), Color::Yellow),
		OutputLabel::Info(info) => (String::from(info), Color::Blue),
		OutputLabel::Success(success) => (String::from(success), Color::Green),
		OutputLabel::Custom(custom, custom_colour) => (String::from(custom), custom_colour),
		OutputLabel::Prompt(prompt) => (String::from(prompt), Color::Yellow),
		OutputLabel::None => (String::from(""), Color::White),
	};

	// TODO: document
	let term_width = terminal_dimensions().unwrap().0;

	match *SIMPLIFY_OUTPUT {
		true => format!("{} {}", label, message),
		false => {
			let label = Style::new(label_color).bold().paint(label).to_string();

			format!(
				"{} {}",
				pad_str(label.as_str(), LABEL_WIDTH, Alignment::Right, None),
				pad_str(
					message.as_str(),
					term_width - LABEL_WIDTH - 1,
					Alignment::Left,
					None,
				)
			)
		}
	}
}
