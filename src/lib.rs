mod error;
#[cfg(feature = "async")]
mod get_text;

#[cfg(feature = "sync")]
pub mod sync;

pub use error::Error;
#[cfg(feature = "async")]
pub use get_text::get_text_from_stdin_or_clipboard;
