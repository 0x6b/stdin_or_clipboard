use stdin_or_clipboard::sync::get_text_from_stdin_or_clipboard;

fn main() {
    let (text, _clipboard) = get_text_from_stdin_or_clipboard().unwrap();
    println!("{text}");
}
