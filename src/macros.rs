// TODO: add more examples
/// The replacement macro that uses core logger functions.
///
/// # Usage
///
/// ```
/// // -snip-
///
/// // In your function :
/// use label_logger::OutputLabel;
/// // -snip-
/// println!(OutputLabel::Info("Compiling"), "the program")
/// println!(_, "information without label")
/// println!(_, "more informations without label")
/// // -snip-
/// println!(OutputLabel::Success("Finished"), "the compilation")
/// ```
///
/// For more see the [examples folder](https://github.com/MrNossiom/label-logger/tree/main/examples)
/// **Note**: this macro replace the builtin println macro
pub macro println {
	(_, $($arg:tt)*) => {
			label_logger::println_label(label_logger::OutputLabel::None, format!($($arg)*))
	},
	($label:expr, $($arg:tt)*) => {
		label_logger::println_label($label, format!($($arg)*))
	}
}

// TODO: document
/// **Note**: this replace the builtin eprintln macro
pub macro eprintln($($arg:tt)*) {
	label_logger::println_label(label_logger::OutputLabel::Error, format!($($arg)*))
}

// TODO: add usage example
/// Print the given message with a carriage return at the end.
/// Useful for mid-process logging.
pub macro print_r($label:expr, $($arg:tt)*) {
	label_logger::print_r_label($label, format!($($arg)*))
}
