use std::io::IsTerminal;

use arboard::Clipboard;
use tokio::io::{stdin, AsyncReadExt};

use crate::error::Error;

/// Get text from stdin or the system clipboard.
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
/// use stdin_or_clipboard::get_text_from_stdin_or_clipboard;
///
/// #[tokio::main]
/// async fn main() {
///    let (text, clipboard) = get_text_from_stdin_or_clipboard().await.unwrap();
///    println!("{text}");
/// }
/// ```
pub async fn get_text_from_stdin_or_clipboard() -> Result<(String, Option<Clipboard>), Error> {
    if std::io::stdin().is_terminal() {
        let mut clipboard = Clipboard::new()?;
        Ok((clipboard.get_text()?.trim().to_string(), Some(clipboard)))
    } else {
        let mut text = String::new();
        stdin().read_to_string(&mut text).await?;
        Ok((text.trim().to_string(), Clipboard::new().ok()))
    }
}
