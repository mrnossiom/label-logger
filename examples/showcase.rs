#[macro_use]
extern crate label_logger;

use label_logger::OutputLabel;
use std::{thread::sleep, time::Duration};

const WAIT_DURATION: Duration = Duration::from_millis(150);

fn main() {
	println!(OutputLabel::Success("Compiling"), "a wonderful program");

	for index in 1..=10 {
		match index {
			2 => println!(OutputLabel::Info("Info"), "the compilation label"),
			5 => println!(
				OutputLabel::Warning,
				"something was a bit weird in the chunk 5"
			),
			7 => println!(OutputLabel::Error, "could not compile the 7th chunk"),
			_ => {}
		}

		print_r!(OutputLabel::None, "Building part {}", index);
		sleep(WAIT_DURATION);
	}

	println!(
		OutputLabel::Success("Finished"),
		"the compilation with 1 warning and 1 error"
	);
}
