# 1.0.0

- No changes since 0.1.4

# 0.1.4

- Add `Theme::new()` to controlling the default color specification.
- Optimize color codes emitted for empty themes.

# 0.1.2

- Forward directly to `serde_json` if colors are not supported by the writer.

# 0.1.1

- `Theme::default()` is slightly tweaked to look better in more terminals.

# 0.1.0

Initial version.

Includes:

- Support for writing JSON to a `WriteColor` stream.
- Limited customization with the `Theme` struct.
