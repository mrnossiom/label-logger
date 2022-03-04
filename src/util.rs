/// Shortens a message by omitting the middle part and replacing it with '...'
///
/// If the given message is shorter than the available width, the
/// original message will be returned
pub fn shorten(message: String, max_width: usize) -> String {
	let len = message.len();

	if len <= max_width {
		return message;
	}

	// Break the message at half of the available width
	// Better for readability than at the end
	let break_index = max_width / 2;

	return [
		message.chars().take(break_index).collect(),
		"...".to_owned(),
		message
			.chars()
			.skip(len - max_width + break_index + 3)
			.collect(),
	]
	.join("");
}

// TODO: make this static
/// Get the current terminal width
///
/// If not found default to 80
pub fn get_term_width() -> usize {
	if let Some((width, _)) = term_size::dimensions() {
		width
	} else {
		80
	}
}
