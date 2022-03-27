use console::measure_text_width;

/// Shortens a message by omitting the middle part and replacing it with '...'
///
/// If the given message is shorter than the available width, the
/// original message will be returned
pub fn shorten(message: String, max_width: usize) -> String {
	let msg_length = measure_text_width(message.as_str());

	if msg_length <= max_width {
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
			.skip(msg_length - max_width + break_index + 3)
			.collect(),
	]
	.join("");
}
