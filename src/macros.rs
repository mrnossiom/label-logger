pub macro println($label:expr, $($arg:tt)*) {
	label_logger::println_label($label, format!($($arg)*))
}

pub macro print($label:expr, $($arg:tt)*) {
	label_logger::print_label($label, format!($($arg)*))
}

pub macro print_r($label:ty, $($arg:tt)*) {
	label_logger::print_r_label($label, format!($($arg)*))
}
