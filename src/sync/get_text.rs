use std::io::{stdin, IsTerminal, Read};

use arboard::Clipboard;

use crate::error::Error;

/// Get text from stdin or the system clipboard. Sync version.
///
/// # Returns
///
/// Tuple containing the text from stdin or the system clipboard, trimmed, and the clipboard if the
/// text was retrieved from the clipboard.
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
/// let (text, clipboard) = get_text_from_stdin_or_clipboard().unwrap();
/// println!("{text}");
/// ```
pub fn get_text_from_stdin_or_clipboard() -> Result<(String, Option<Clipboard>), Error> {
    if stdin().is_terminal() {
        let mut clipboard = Clipboard::new()?;
        Ok((clipboard.get_text()?.trim().to_string(), Some(clipboard)))
    } else {
        let mut text = String::new();
        stdin().read_to_string(&mut text)?;
        Ok((text.trim().to_string(), Clipboard::new().ok()))
    }
}
