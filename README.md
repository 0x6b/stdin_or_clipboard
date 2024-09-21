# stdin_or_clipboard

Get a text from the stdin or a clipboard.

I often found myself writing code to retrieve text from the clipboard or stdin, so I created this small crate to make my life easier. However, you might want to consider using a more robust option available out here and there.

## Usage

```rust
use stdin_or_clipboard::get_text_from_stdin_or_clipboard;

#[tokio::main]
async fn main() {
    let text = get_text_from_stdin_or_clipboard().await.unwrap();
    println!("{text}");
}
```

## License

MIT. See [LICENSE](LICENSE) for details.
