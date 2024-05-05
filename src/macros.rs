//! The replacement macro that uses core logger functions.
//!
//! # Usage
//!
//! ```rust
//! use label_logger::{info, log, success};
//!
//! info!(label: "Compiling", "the program");
//! log!("information without label");
//! log!("more informations without label");
//! success!(label: "Finished", "the compilation");
//! ```
//!
//! For more see the [examples folder](https://github.com/MrNossiom/label_logger/tree/main/examples)

/// Prints a message with no or the provided label
///
/// ```
/// # use label_logger::log;
/// log!("Hello, world!");
/// ```
#[macro_export]
macro_rules! log {
	(label: $lbl:expr, $($arg:tt)+) => {
		$crate::println_label($lbl, std::format!($($arg)+))
	};
	($($arg:tt)+) => {
		$crate::println_label($crate::OutputLabel::default(), std::format!($($arg)*))
	};
}

/// Prints a message with the error label (prints to `stderr`)
///
/// ```
/// # use label_logger::error;
/// error!("An error occurred while deactivating the nuclear core");
/// // "Boom" will be displayed in red as a label
/// error!(label: "Boom", "it is too late");
/// ```
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
///
/// ```
/// # use label_logger::warn;
/// warn!("This is fine, there are only 2849 warnings, but no errors");
/// // "Tic Tac" will be displayed in yellow as a label
/// warn!(label: "Tic Tac", "run forest, run!");
/// ```
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
///
/// ```
/// # use label_logger::info;
/// info!("Cleaning up the mess that was made");
/// // "Waiting" will be displayed in cyan (light blue) as a label
/// info!(label: "Waiting", "for OAuth2 callback");
/// ```
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
///
/// ```
/// # use label_logger::success;
/// success!("temporary file successfully deleted");
/// // "Waouh" will be displayed in green as a label
/// success!(label: "Waouh", "you successfully did not went on StackOverflow for 5min");
/// ```
#[macro_export]
macro_rules! success {
	(label: $lbl:expr, $($arg:tt)+) => {
		$crate::println_label($crate::OutputLabel::Success($lbl), std::format!($($arg)+))
	};
	($($arg:tt)+) => {
		$crate::println_label($crate::OutputLabel::Success("Success"), std::format!($($arg)+))
	};
}

/// Formats your message with the specified output label
///
/// ```
/// # use label_logger::{format_label, OutputLabel};
/// let _msg = format_label!(
///     label: OutputLabel::Info("Building"),
///     "one crate at a time"
/// );
/// ```
///
/// # Example
///
/// It can be useful when you need to have a correctly formatted message but you don't want to print it directly to the terminal.
///
// indicatif is only available on `indicatif` feature
/// ```ignore
/// use label_logger::{format_label, OutputLabel};
/// use indicatif::ProgressBar;
///
/// let mut bar = ProgressBar::new(100);
/// let msg = format_label!(
///    label: OutputLabel::Info("Building"),
///   "one crate at a time"
/// );
///
/// bar.set_message(msg);
/// ```
#[macro_export]
macro_rules! format_label {
	(label: $label:expr, $($arg:tt)+) => {
		$crate::pretty_output($label, std::format!($($arg)+))
	};
	($($arg:tt)+) => {
		$crate::pretty_output($crate::OutputLabel::default(), std::format!($($arg)+))
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
