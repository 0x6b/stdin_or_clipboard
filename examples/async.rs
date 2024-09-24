use stdin_or_clipboard::get_text_from_stdin_or_clipboard;

#[tokio::main]
async fn main() {
    let (text, _clipboard) = get_text_from_stdin_or_clipboard().await.unwrap();
    println!("{text}");
}
