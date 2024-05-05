use label_logger::{error, format_label, info, success, warn};
use std::{
	io::{stdout, Write},
	thread::sleep,
	time::Duration,
};

const WAIT_DURATION: Duration = Duration::from_millis(150);

fn main() {
	success!(label: "Compiling", "a wonderful program");

	for index in 1..=10 {
		match index {
			2 => info!("some line of code is great"),
			5 => warn!("something is a bit weird in chunk 5"),
			7 => error!("could not compile the 7th chunk"),
			_ => {}
		}

		print!("{}\r", format_label!("Building part {}", index));
		stdout().flush().unwrap();

		sleep(WAIT_DURATION);
	}

	success!(label: "Passed", "the compilation with 1 warning and 1 error");
}
