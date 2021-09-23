#![deny(missing_debug_implementations)]
#![deny(missing_docs)]
#![doc(html_root_url = "https://docs.rs/termcolor-json/0.1.3")]

//! A library for writing colored [JSON](https://crates.io/crates/serde_json) output to a [termcolor](https://crates.io/crates/termcolor) terminal.
//!
//! ```rust
//! # use termcolor::{ColorChoice, StandardStream};
//! # fn run() -> serde_json::Result<()> {
//! let stdout = StandardStream::stdout(ColorChoice::Auto);
//!
//! termcolor_json::to_writer(
//!     &mut stdout.lock(),
//!     &serde_json::json!({
//!         "string": "value",
//!         "number": 123,
//!         "bool": true,
//!         "null": null,
//!     }),
//! )?;
//! # Ok(())
//! # }
//! # fn main() { run().unwrap() }
//! ```

use std::{
    cell::RefCell,
    io::{self, Write},
    mem,
};

use serde::Serialize;
use serde_json::ser::{CharEscape, CompactFormatter, Formatter, PrettyFormatter, Serializer};
use termcolor::{Color, ColorSpec, WriteColor};

/// Controls the console formatter used for different JSON tokens.
///
/// A reasonable default theme is provided by [`Theme::default`].
#[derive(Clone, Debug)]
pub struct Theme {
    reset: ColorSpec,
    null: ColorSpec,
    bool: ColorSpec,
    number: ColorSpec,
    string: ColorSpec,
    object_key: ColorSpec,
}

/// Serialize the given data structure as colored, pretty-printed JSON into the IO stream, using the default theme.
pub fn to_writer<W, T>(writer: W, value: &T) -> serde_json::Result<()>
where
    W: WriteColor,
    T: ?Sized + Serialize,
{
    to_writer_with_theme_and_formatter(writer, value, &Theme::default(), PrettyFormatter::new())
}

/// Serialize the given data structure as colored, compact JSON into the IO stream, using the default theme.
pub fn to_writer_compact<W, T>(writer: W, value: &T) -> serde_json::Result<()>
where
    W: WriteColor,
    T: ?Sized + Serialize,
{
    to_writer_with_theme_and_formatter(writer, value, &Theme::default(), CompactFormatter)
}

/// Serialize the given data structure as colored, pretty-printed JSON into the IO stream, using the given theme.
pub fn to_writer_with_theme<W, T>(writer: W, value: &T, theme: &Theme) -> serde_json::Result<()>
where
    W: WriteColor,
    T: ?Sized + Serialize,
{
    to_writer_with_theme_and_formatter(writer, value, theme, PrettyFormatter::new())
}

/// Serialize the given data structure as colored JSON into the IO stream, using the given theme and formatter.
///
/// The `formatter` argument is used to write text to the stream. For example, to customize the identation of pretty-printed JSON, you could
/// pass `PrettyFormatter::with_indent("\t")`.
pub fn to_writer_with_theme_and_formatter<W, T, F>(
    writer: W,
    value: &T,
    theme: &Theme,
    formatter: F,
) -> serde_json::Result<()>
where
    W: WriteColor,
    T: ?Sized + Serialize,
    F: Formatter,
{
    if !writer.supports_color() {
        let mut ser = Serializer::with_formatter(writer, formatter);
        value.serialize(&mut ser)
    } else {
        let writer = SharedWriter::new(writer);
        let formatter = ColorFormatter::new(&writer, theme, formatter);
        let mut ser = Serializer::with_formatter(&writer, formatter);
        value.serialize(&mut ser)
    }
}

struct ColorFormatter<'a, W, F> {
    formatter: F,
    writer: W,
    theme: &'a Theme,
    writing_key: bool,
    need_reset: bool,
}

impl<'a, W, F> ColorFormatter<'a, W, F> {
    fn new(writer: W, theme: &'a Theme, formatter: F) -> Self {
        ColorFormatter {
            formatter,
            writer,
            theme,
            writing_key: false,
            need_reset: false,
        }
    }
}

impl<'a, W, F> Formatter for ColorFormatter<'a, W, F>
where
    W: WriteColor,
    F: Formatter,
{
    fn write_null<U>(&mut self, _: &mut U) -> io::Result<()>
    where
        U: ?Sized + Write,
    {
        let f = &mut self.formatter;
        with_color(&mut self.writer, &self.theme.null, self.theme, |w| {
            f.write_null(w)
        })
    }

    fn write_bool<U>(&mut self, _: &mut U, value: bool) -> io::Result<()>
    where
        U: ?Sized + Write,
    {
        let f = &mut self.formatter;
        with_color(&mut self.writer, &self.theme.bool, self.theme, |w| {
            f.write_bool(w, value)
        })
    }

    fn write_i8<U>(&mut self, _: &mut U, value: i8) -> io::Result<()>
    where
        U: ?Sized + Write,
    {
        let f = &mut self.formatter;
        with_color(&mut self.writer, &self.theme.number, self.theme, |w| {
            f.write_i8(w, value)
        })
    }

    fn write_i16<U>(&mut self, _: &mut U, value: i16) -> io::Result<()>
    where
        U: ?Sized + Write,
    {
        let f = &mut self.formatter;
        with_color(&mut self.writer, &self.theme.number, self.theme, |w| {
            f.write_i16(w, value)
        })
    }

    fn write_i32<U>(&mut self, _: &mut U, value: i32) -> io::Result<()>
    where
        U: ?Sized + Write,
    {
        let f = &mut self.formatter;
        with_color(&mut self.writer, &self.theme.number, self.theme, |w| {
            f.write_i32(w, value)
        })
    }

    fn write_i64<U>(&mut self, _: &mut U, value: i64) -> io::Result<()>
    where
        U: ?Sized + Write,
    {
        let f = &mut self.formatter;
        with_color(&mut self.writer, &self.theme.number, self.theme, |w| {
            f.write_i64(w, value)
        })
    }

    fn write_u8<U>(&mut self, _: &mut U, value: u8) -> io::Result<()>
    where
        U: ?Sized + Write,
    {
        let f = &mut self.formatter;
        with_color(&mut self.writer, &self.theme.number, self.theme, |w| {
            f.write_u8(w, value)
        })
    }

    fn write_u16<U>(&mut self, _: &mut U, value: u16) -> io::Result<()>
    where
        U: ?Sized + Write,
    {
        let f = &mut self.formatter;
        with_color(&mut self.writer, &self.theme.number, self.theme, |w| {
            f.write_u16(w, value)
        })
    }

    fn write_u32<U>(&mut self, _: &mut U, value: u32) -> io::Result<()>
    where
        U: ?Sized + Write,
    {
        let f = &mut self.formatter;
        with_color(&mut self.writer, &self.theme.number, self.theme, |w| {
            f.write_u32(w, value)
        })
    }

    fn write_u64<U>(&mut self, _: &mut U, value: u64) -> io::Result<()>
    where
        U: ?Sized + Write,
    {
        let f = &mut self.formatter;
        with_color(&mut self.writer, &self.theme.number, self.theme, |w| {
            f.write_u64(w, value)
        })
    }

    fn write_f32<U>(&mut self, _: &mut U, value: f32) -> io::Result<()>
    where
        U: ?Sized + Write,
    {
        let f = &mut self.formatter;
        with_color(&mut self.writer, &self.theme.number, self.theme, |w| {
            f.write_f32(w, value)
        })
    }

    fn write_f64<U>(&mut self, _: &mut U, value: f64) -> io::Result<()>
    where
        U: ?Sized + Write,
    {
        let f = &mut self.formatter;
        with_color(&mut self.writer, &self.theme.number, self.theme, |w| {
            f.write_f64(w, value)
        })
    }

    fn write_number_str<U>(&mut self, _: &mut U, value: &str) -> io::Result<()>
    where
        U: ?Sized + Write,
    {
        let f = &mut self.formatter;
        with_color(&mut self.writer, &self.theme.number, self.theme, |w| {
            f.write_number_str(w, value)
        })
    }

    fn begin_string<U>(&mut self, _: &mut U) -> io::Result<()>
    where
        U: ?Sized + Write,
    {
        if !self.writing_key {
            self.need_reset = set_color(&mut self.writer, &self.theme.string)?;
        }
        self.formatter.begin_string(&mut self.writer)
    }

    fn end_string<U>(&mut self, _: &mut U) -> io::Result<()>
    where
        U: ?Sized + Write,
    {
        self.formatter.end_string(&mut self.writer)?;
        if !self.writing_key && mem::take(&mut self.need_reset) {
            reset(&mut self.writer, self.theme)?;
        }
        Ok(())
    }

    fn write_string_fragment<U>(&mut self, _: &mut U, fragment: &str) -> io::Result<()>
    where
        U: ?Sized + Write,
    {
        self.formatter
            .write_string_fragment(&mut self.writer, fragment)
    }

    fn write_char_escape<U>(&mut self, _: &mut U, char_escape: CharEscape) -> io::Result<()>
    where
        U: ?Sized + Write,
    {
        self.formatter
            .write_char_escape(&mut self.writer, char_escape)
    }

    fn begin_array<U>(&mut self, _: &mut U) -> io::Result<()>
    where
        U: ?Sized + Write,
    {
        self.formatter.begin_array(&mut self.writer)
    }

    fn end_array<U>(&mut self, _: &mut U) -> io::Result<()>
    where
        U: ?Sized + Write,
    {
        self.formatter.end_array(&mut self.writer)
    }

    fn begin_array_value<U>(&mut self, _: &mut U, first: bool) -> io::Result<()>
    where
        U: ?Sized + Write,
    {
        self.formatter.begin_array_value(&mut self.writer, first)
    }

    fn end_array_value<U>(&mut self, _: &mut U) -> io::Result<()>
    where
        U: ?Sized + Write,
    {
        self.formatter.end_array_value(&mut self.writer)
    }

    fn begin_object<U>(&mut self, _: &mut U) -> io::Result<()>
    where
        U: ?Sized + Write,
    {
        self.formatter.begin_object(&mut self.writer)
    }

    fn end_object<U>(&mut self, _: &mut U) -> io::Result<()>
    where
        U: ?Sized + Write,
    {
        self.formatter.end_object(&mut self.writer)
    }

    fn begin_object_key<U>(&mut self, _: &mut U, first: bool) -> io::Result<()>
    where
        U: ?Sized + Write,
    {
        self.writing_key = true;
        self.formatter.begin_object_key(&mut self.writer, first)?;
        self.need_reset = set_color(&mut self.writer, &self.theme.object_key)?;
        Ok(())
    }

    fn end_object_key<U>(&mut self, _: &mut U) -> io::Result<()>
    where
        U: ?Sized + Write,
    {
        if mem::take(&mut self.need_reset) {
            reset(&mut self.writer, self.theme)?;
        }
        self.formatter.end_object_key(&mut self.writer)?;
        self.writing_key = false;
        Ok(())
    }

    fn begin_object_value<U>(&mut self, _: &mut U) -> io::Result<()>
    where
        U: ?Sized + Write,
    {
        self.formatter.begin_object_value(&mut self.writer)
    }

    fn end_object_value<U>(&mut self, _: &mut U) -> io::Result<()>
    where
        U: ?Sized + Write,
    {
        self.formatter.end_object_value(&mut self.writer)
    }

    fn write_raw_fragment<U>(&mut self, _: &mut U, fragment: &str) -> io::Result<()>
    where
        U: ?Sized + io::Write,
    {
        self.formatter
            .write_raw_fragment(&mut self.writer, fragment)
    }
}

fn set_color<W>(writer: &mut W, color: &ColorSpec) -> io::Result<bool>
where
    W: WriteColor,
{
    if color.is_none() {
        Ok(false)
    } else {
        writer.set_color(color)?;
        Ok(true)
    }
}

fn reset<W>(writer: &mut W, theme: &Theme) -> io::Result<()>
where
    W: WriteColor,
{
    writer.set_color(&theme.reset)
}

fn with_color<W, F>(writer: &mut W, color: &ColorSpec, theme: &Theme, write: F) -> io::Result<()>
where
    W: WriteColor,
    F: FnOnce(&mut W) -> io::Result<()>,
{
    if set_color(writer, color)? {
        write(writer)?;
        reset(writer, theme)?;
        Ok(())
    } else {
        write(writer)
    }
}

// serde_json's serializer expects to own its own `Write` implementation, but we need to keep our
// own reference to it so we can set colors on the stream.
//
// We could pass serde_json a dummy implementation like io::Sink and do all the writing ourselves
// in ColorFormatter, but this is a forwards-compability hazard if new methods are added to Formatter
// in future which write to the dummy stream by default.
//
// Instead we share ownership of the stream between serde_json and this library using a RefCell.
struct SharedWriter<W> {
    inner: RefCell<W>,
}

impl<W> SharedWriter<W> {
    fn new(writer: W) -> Self {
        SharedWriter {
            inner: RefCell::new(writer),
        }
    }
}

impl<W> Write for &'_ SharedWriter<W>
where
    W: Write,
{
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.inner.borrow_mut().write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.inner.borrow_mut().flush()
    }

    fn write_vectored(&mut self, bufs: &[std::io::IoSlice<'_>]) -> io::Result<usize> {
        self.inner.borrow_mut().write_vectored(bufs)
    }

    fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        self.inner.borrow_mut().write_all(buf)
    }
}

impl<W> WriteColor for &'_ SharedWriter<W>
where
    W: WriteColor,
{
    fn supports_color(&self) -> bool {
        self.inner.borrow().supports_color()
    }

    fn set_color(&mut self, spec: &ColorSpec) -> io::Result<()> {
        self.inner.borrow_mut().set_color(spec)
    }

    fn reset(&mut self) -> io::Result<()> {
        self.inner.borrow_mut().reset()
    }

    fn is_synchronous(&self) -> bool {
        self.inner.borrow().is_synchronous()
    }
}

impl Theme {
    /// Create a theme with no styling.
    pub fn none() -> Self {
        Theme::new(ColorSpec::new())
    }

    /// Create a theme where all text is printed using the given [`termcolor::ColorSpec`].
    pub fn new(default: ColorSpec) -> Self {
        Theme {
            reset: default.clone(),
            null: default.clone(),
            bool: default.clone(),
            number: default.clone(),
            string: default.clone(),
            object_key: default,
        }
    }

    /// Gets a reference to the color specification for the `null` token.
    pub fn null(&self) -> &ColorSpec {
        &self.null
    }

    /// Gets a mutable reference to the color specification for the `null` token.
    pub fn null_mut(&mut self) -> &mut ColorSpec {
        &mut self.null
    }

    /// Gets a reference to the color specification for `true` and `false` tokens.
    pub fn bool(&self) -> &ColorSpec {
        &self.bool
    }

    /// Gets a mutable reference to the color specification for `true` and `false` tokens.
    pub fn bool_mut(&mut self) -> &mut ColorSpec {
        &mut self.bool
    }

    /// Gets a reference to the color specification for number tokens.
    pub fn number(&self) -> &ColorSpec {
        &self.number
    }

    /// Gets a mutable reference to the color specification for number tokens.
    pub fn number_mut(&mut self) -> &mut ColorSpec {
        &mut self.number
    }

    /// Gets a mutable reference to the color specification for string tokens.
    ///
    /// Note this is not used for object keys, which are controlled by the [Theme::object_key] field.
    pub fn string(&self) -> &ColorSpec {
        &self.string
    }

    /// Gets a mutable reference to the color specification for string tokens.
    ///
    /// Note this is not used for object keys, which are controlled by the [Theme::object_key_mut] field.
    pub fn string_mut(&mut self) -> &mut ColorSpec {
        &mut self.string
    }

    /// Gets a reference to the color specification for object key tokens.
    pub fn object_key(&self) -> &ColorSpec {
        &self.object_key
    }

    /// Gets a mutable reference to the color specification for object key tokens.
    pub fn object_key_mut(&mut self) -> &mut ColorSpec {
        &mut self.object_key
    }
}

impl Default for Theme {
    /// Get a reasonable default theme.
    fn default() -> Self {
        let mut theme = Theme::none();

        theme.null_mut().set_fg(Some(Color::Cyan)).set_bold(true);
        theme.bool_mut().set_fg(Some(Color::Cyan)).set_bold(true);
        theme.number_mut().set_fg(Some(Color::Cyan));
        theme.string_mut().set_fg(Some(Color::Green));
        theme
            .object_key_mut()
            .set_fg(Some(Color::Blue))
            .set_intense(true);

        theme
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn data() -> impl Serialize {
        serde_json::json!({
            "b": true,
            "n": null,
            "m": 1,
            "s": "v",
        })
    }

    fn to_readable_string(vec: Vec<u8>) -> String {
        String::from_utf8_lossy(&vec).replace('\x1B', "^[")
    }

    #[test]
    fn no_color_passthrough() {
        use termcolor::NoColor;

        let mut buf = Vec::new();
        let writer = NoColor::new(io::Cursor::new(&mut buf));

        to_writer(writer, &data()).unwrap();

        assert_eq!(
            to_readable_string(buf).as_str(),
            "{\n  \"b\": true,\n  \"m\": 1,\n  \"n\": null,\n  \"s\": \"v\"\n}"
        );
    }

    #[test]
    fn no_color_passthrough_compact() {
        use termcolor::NoColor;

        let mut buf = Vec::new();
        let writer = NoColor::new(io::Cursor::new(&mut buf));

        to_writer_compact(writer, &data()).unwrap();

        assert_eq!(
            to_readable_string(buf).as_str(),
            "{\"b\":true,\"m\":1,\"n\":null,\"s\":\"v\"}"
        );
    }

    #[test]
    fn ansi_empty_theme_passthrough() {
        use termcolor::Ansi;

        let mut buf = Vec::new();
        let writer = Ansi::new(io::Cursor::new(&mut buf));

        to_writer_with_theme(writer, &data(), &Theme::none()).unwrap();

        assert_eq!(
            to_readable_string(buf).as_str(),
            "{\n  \"b\": true,\n  \"m\": 1,\n  \"n\": null,\n  \"s\": \"v\"\n}"
        );
    }

    #[test]
    fn ansi_empty_theme_passthrough_compact() {
        use termcolor::Ansi;

        let mut buf = Vec::new();
        let writer = Ansi::new(io::Cursor::new(&mut buf));

        to_writer_with_theme_and_formatter(writer, &data(), &Theme::none(), CompactFormatter)
            .unwrap();

        assert_eq!(
            to_readable_string(buf).as_str(),
            "{\"b\":true,\"m\":1,\"n\":null,\"s\":\"v\"}"
        );
    }

    #[test]
    fn ansi_default_theme() {
        use termcolor::Ansi;

        let mut buf = Vec::new();
        let writer = Ansi::new(io::Cursor::new(&mut buf));

        to_writer(writer, &data()).unwrap();

        assert_eq!(to_readable_string(buf).as_str(), "{\n  ^[[0m^[[38;5;12m\"b\"^[[0m: ^[[0m^[[1m^[[36mtrue^[[0m,\n  ^[[0m^[[38;5;12m\"m\"^[[0m: ^[[0m^[[36m1^[[0m,\n  ^[[0m^[[38;5;12m\"n\"^[[0m: ^[[0m^[[1m^[[36mnull^[[0m,\n  ^[[0m^[[38;5;12m\"s\"^[[0m: ^[[0m^[[32m\"v\"^[[0m\n}");
    }

    #[test]
    fn ansi_default_theme_compact() {
        use termcolor::Ansi;

        let mut buf = Vec::new();
        let writer = Ansi::new(io::Cursor::new(&mut buf));

        to_writer_compact(writer, &data()).unwrap();

        assert_eq!(to_readable_string(buf).as_str(), "{^[[0m^[[38;5;12m\"b\"^[[0m:^[[0m^[[1m^[[36mtrue^[[0m,^[[0m^[[38;5;12m\"m\"^[[0m:^[[0m^[[36m1^[[0m,^[[0m^[[38;5;12m\"n\"^[[0m:^[[0m^[[1m^[[36mnull^[[0m,^[[0m^[[38;5;12m\"s\"^[[0m:^[[0m^[[32m\"v\"^[[0m}");
    }
}
