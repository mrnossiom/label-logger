// TODO: add more examples
/// The replacement macro that uses core logger functions.
///
/// # Usage
///
/// ```rust
/// # #[macro_use] extern crate label_logger;
/// # fn main() {
/// // In your function :
/// use label_logger::OutputLabel;
/// // -snip-
/// println!(OutputLabel::Info("Compiling"), "the program");
/// println!(_, "information without label");
/// println!(_, "more informations without label");
/// // -snip-
/// println!(OutputLabel::Success("Finished"), "the compilation");
/// # }
/// ```
///
/// For more see the [examples folder](https://github.com/MrNossiom/label-logger/tree/main/examples)
/// **Note**: this macro replace the builtin println macro
#[macro_export]
macro_rules! println {
	(_, $($arg:tt)*) => {
			$crate::println_label($crate::OutputLabel::None, format!($($arg)*))
	};
	($label:expr, $($arg:tt)*) => {
		$crate::println_label($label, format!($($arg)*))
	};
}

// TODO: document
/// **Note**: this replace the builtin eprintln macro
#[macro_export]
macro_rules! eprintln {
	($($arg:tt)*) => {
		$crate::println_label($crate::OutputLabel::Error, format!($($arg)*))
	};
}

/// Prints a message with the warning label
#[macro_export]
macro_rules! warn {
	($($arg:tt)*) => {
		$crate::println_label($crate::OutputLabel::Warning, format!($($arg)*))
	};
}

/// Prints a message with the info label and the provided text
#[macro_export]
macro_rules! info {
	($info_label:expr, $($arg:tt)*) => {
		$crate::println_label($crate::OutputLabel::Info($info_label), format!($($arg)*))
	};
}

/// Prints a message with the success label and the provided text
#[macro_export]
macro_rules! success {
	($success_label:expr, $($arg:tt)*) => {
		$crate::println_label($crate::OutputLabel::Success($success_label), format!($($arg)*))
	};
}

// TODO: add usage example
/// Print the given message with a carriage return at the end
/// Useful for mid-process logging
#[macro_export]
macro_rules! print_r {
	($label:expr, $($arg:tt)*) => {
		$crate::print_r_label($label, format!($($arg)*))
	};
}

// TODO: document
/// Formats your message with the specified output label
#[macro_export]
macro_rules! format_label {
	(_, $($arg:tt)*) => {
		$crate::pretty_output($crate::OutputLabel::None, format!($($arg)*))
	};
	($label:expr, $($arg:tt)*) => {
		$crate::pretty_output($label, format!($($arg)*))
	};
}
