use std::io::IsTerminal;

use arboard::Clipboard;
use tokio::io::{stdin, AsyncReadExt};

use crate::error::Error;

/// Get text from stdin or the system clipboard.
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
/// use stdin_or_clipboard::get_text_from_stdin_or_clipboard;
///
/// #[tokio::main]
/// async fn main() {
///    let text = get_text_from_stdin_or_clipboard().await.unwrap();
///    println!("{text}");
/// }
/// ```
pub async fn get_text_from_stdin_or_clipboard() -> Result<String, Error> {
    if std::io::stdin().is_terminal() {
        Ok(Clipboard::new()?.get_text()?.trim().to_string())
    } else {
        let mut text = String::new();
        stdin().read_to_string(&mut text).await?;
        Ok(text.trim().to_string())
    }
}
