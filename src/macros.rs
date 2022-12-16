// TODO: add more examples
//! The replacement macro that uses core logger functions.
//!
//! # Usage
//!
//! ```rust
//! # fn main() {
//! pub use label_logger::{info, log, success};
//!
//! info!(label: "Compiling", "the program");
//! log!("information without label");
//! log!("more informations without label");
//! success!(label: "Finished", "the compilation");
//! # }
//! ```
//!
//! For more see the [examples folder](https://github.com/MrNossiom/label_logger/tree/main/examples)

/// Prints a message with no or the provided label
#[macro_export]
macro_rules! log {
	(label: $lbl:expr, $($arg:tt)+) => {
		$crate::println_label($lbl, std::format!($($arg)+))
	};
	($($arg:tt)+) => {
		$crate::println_label($crate::OutputLabel::None, std::format!($($arg)*))
	};
}

// TODO: document
/// Prints a message with the error label (prints to stdout)
#[macro_export]
macro_rules! error {
	(label: $lbl:tt, $($arg:tt)+) => {
		$crate::println_label($crate::OutputLabel::Error($lbl), std::format!($($arg)+))
	};
	($($arg:tt)+) => {
		$crate::println_label($crate::OutputLabel::Error("Error"), std::format!($($arg)+))
	};
}

/// Prints a message with the warning label
#[macro_export]
macro_rules! warn {
	(label: $lbl:expr, $($arg:tt)+) => {
		$crate::println_label($crate::OutputLabel::Warning($lbl), std::format!($($arg)+))
	};
	($($arg:tt)*) => {
		$crate::println_label($crate::OutputLabel::Warning("Warn"), std::format!($($arg)*))
	};
}

/// Prints a message with the info label and the provided text
#[macro_export]
macro_rules! info {
	(label: $lbl:expr, $($arg:tt)+) => {
		$crate::println_label($crate::OutputLabel::Info($lbl), std::format!($($arg)+))
	};
	($($arg:tt)+) => {
		$crate::println_label($crate::OutputLabel::Info("Info"), std::format!($($arg)+))
	};
}

/// Prints a message with the success label and the provided text
#[macro_export]
macro_rules! success {
	(label: $lbl:expr, $($arg:tt)+) => {
		$crate::println_label($crate::OutputLabel::Success($lbl), std::format!($($arg)+))
	};
	($($arg:tt)+) => {
		$crate::println_label($crate::OutputLabel::Success("Success"), std::format!($($arg)+))
	};
}

// TODO: document
/// Formats your message with the specified output label
#[macro_export]
macro_rules! format_label {
	(label: $label:expr, $($arg:tt)+) => {
		$crate::pretty_output($label, std::format!($($arg)+))
	};
	($($arg:tt)+) => {
		$crate::pretty_output($crate::OutputLabel::None, std::format!($($arg)+))
	};
}

#[cfg(test)]
mod tests {
	use crate::OutputLabel;

	#[test]
	fn log_macro_expand() {
		log!("Hello, world!");

		log!(label: OutputLabel::Error("Error"), "Hello");
	}

	#[test]
	fn error_macro_expand() {
		error!("Hello, error!");

		error!(label: "Bip Bip", "alert, everything is broken");
	}

	#[test]
	fn warn_macro_expand() {
		warn!("Hello, warn!");

		warn!(label: "Wow", "there is a bug here!");
	}

	#[test]
	fn info_macro_expand() {
		info!("Hello, info!");

		info!(label: "Ping", "new message");
	}

	#[test]
	fn success_macro_expand() {
		success!("Hello, success!");

		success!(label: "Nice", "you succeed!");
	}

	#[test]
	fn format_label_macro_expand() {
		format_label!(label: OutputLabel::Info("Hey"), "Hello, world!");
	}
}
