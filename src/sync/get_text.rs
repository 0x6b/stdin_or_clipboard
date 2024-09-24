use std::io::{stdin, IsTerminal, Read};

use arboard::Clipboard;

use crate::error::Error;

/// Get text from stdin or the system clipboard. Sync version.
///
/// # Returns
///
/// The text from stdin or the system clipboard, trimmed.
///
/// # Errors
///
/// If the text cannot be retrieved from the clipboard or stdin.
///
/// # Examples
///
/// ```rust
/// use stdin_or_clipboard::sync::get_text_from_stdin_or_clipboard;
///
/// let text = get_text_from_stdin_or_clipboard().unwrap();
/// println!("{text}");
/// ```
pub fn get_text_from_stdin_or_clipboard() -> Result<String, Error> {
    if stdin().is_terminal() {
        Ok(Clipboard::new()?.get_text()?.trim().to_string())
    } else {
        let mut text = String::new();
        stdin().read_to_string(&mut text)?;
        Ok(text.trim().to_string())
    }
}
