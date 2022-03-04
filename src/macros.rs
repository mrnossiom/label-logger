/// TODO
///
/// # Usage
///
/// ```rust
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
/// > Note: this macro replace the builtin println macro
pub macro println {
	(_, $($arg:tt)*) => {
			label_logger::println_label(label_logger::OutputLabel::None, format!($($arg)*))
	},
	($label:expr, $($arg:tt)*) => {
		label_logger::println_label($label, format!($($arg)*))
	}
}

/// > Note: this replace the builtin eprintln macro
/// TODO: document
pub macro eprintln($($arg:tt)*) {
	label_logger::println_label(label_logger::OutputLabel::Error, format!($($arg)*))
}

///
/// TODO: document
pub macro print_r($label:expr, $($arg:tt)*) {
	label_logger::print_r_label($label, format!($($arg)*))
}
