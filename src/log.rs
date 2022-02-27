use crate::util::{get_term_width, shorten};
use std::{
	io::{stdout, Write},
	sync::atomic::{AtomicBool, Ordering},
};
use yansi::{Color, Style};

pub const LABEL_WIDTH: usize = 12;
pub static SIMPLIFY_OUTPUT: AtomicBool = AtomicBool::new(false);

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
pub fn println_label<S: Into<String>>(label: OutputLabel, message: S) {
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
pub fn print_label<S: Into<String>>(label: OutputLabel, message: S) {
	print!("{}", pretty_output(label, message));
}

/// Print a message with a label, add a carriage return at the end and flush the stdout
pub fn print_r_label<S: Into<String>>(label: OutputLabel, message: S) {
	print!("{}\r", pretty_output(label, message));

	stdout().flush().unwrap_or_else(|_| {
		println_label(OutputLabel::Error, "Could not flush stdout");
	});
}

/// Pretty a message with a given label and a given message colour
pub fn pretty_output<S: Into<String>>(label: OutputLabel, message: S) -> String {
	let (label, label_color) = match label {
		OutputLabel::Error => (String::from("Error"), Color::Red),
		OutputLabel::Warning => (String::from("Warn"), Color::Yellow),
		OutputLabel::Info(info) => (String::from(info), Color::Blue),
		OutputLabel::Success(success) => (String::from(success), Color::Green),
		OutputLabel::Custom(custom, custom_colour) => (String::from(custom), custom_colour),
		OutputLabel::Prompt(prompt) => (String::from(prompt), Color::Yellow),
		OutputLabel::None => (String::from(""), Color::White),
	};

	let term_width = get_term_width();
	let message = message.into();

	match SIMPLIFY_OUTPUT.load(Ordering::Acquire) {
		true => format!("{} {}", label, message),
		false => {
			let shorten_message = shorten(message, term_width - LABEL_WIDTH - 1);

			format!(
				"{}{} {}{}",
				" ".repeat(LABEL_WIDTH - label.len()),
				Style::new(label_color).bold().paint(label),
				shorten_message,
				" ".repeat(term_width - LABEL_WIDTH - shorten_message.len() - 1),
			)
		}
	}
}
