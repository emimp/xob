pub fn control_backspace(text_buf: &mut Vec<char>) {
    // This is Control backspace functionality
    if !text_buf.contains(&' ') {
        text_buf.clear();
    } else {
        // Checks and removes all trailing spaces & punctuation
        let last_pos = text_buf
            .iter()
            .rposition(|&c| [' ', '.', '?', '!'].contains(&c)); // find last position of repeating char
        if let Some(last_pos) = last_pos {
            text_buf.drain(last_pos + 1..text_buf.len());
        }

        // Delete the entie word until previous space
        if let Some(last_ch) = text_buf.last() {
            let mut last_non_space_index = None;
            for (index, ch) in text_buf.iter().rev().enumerate() {
                if ch != last_ch {
                    last_non_space_index = Some(index);

                    break; // Stop at the first non-space character found from the end
                }
            }
            if let Some(last_non_space_index) = last_non_space_index {
                if last_non_space_index != 0 {
                    let to = text_buf.len() - last_non_space_index;
                    text_buf.drain(to..text_buf.len());
                };
            }
        }
    }
}