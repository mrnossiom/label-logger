#[macro_use]
extern crate label_logger;

use label_logger::OutputLabel;
use std::{thread::sleep, time::Duration};

const WAIT_DURATION: Duration = Duration::from_millis(150);

fn main() {
	success!("Compiling", "a wonderful program");

	for index in 1..=10 {
		match index {
			2 => info!("Info", "the compilation label"),
			5 => warn!("something was a bit weird in chunk 5"),
			7 => eprintln!("could not compile the 7th chunk"),
			_ => {}
		}

		print_r!(OutputLabel::None, "Building part {}", index);
		sleep(WAIT_DURATION);
	}

	success!("Finished", "the compilation with 1 warning and 1 error");
}
