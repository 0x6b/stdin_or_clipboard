use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    /// Failed to get text from the clipboard.
    #[error("failed to get text from clipboard")]
    Clipboard(#[from] arboard::Error),
    /// Failed to read from stdin.
    #[error("failed to read from stdin")]
    Io(#[from] std::io::Error),
}
