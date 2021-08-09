use std::{
    cell::RefCell,
    io::{self, Write},
};

use serde::Serialize;
use serde_json::ser::{CharEscape, CompactFormatter, Formatter, PrettyFormatter, Serializer};
use termcolor::{Color, ColorSpec, WriteColor};

pub struct Theme {
    pub null: ColorSpec,
    pub bool: ColorSpec,
    pub number: ColorSpec,
    pub string: ColorSpec,
    pub object_key: ColorSpec,
}

pub fn to_writer<W, T>(writer: W, value: &T) -> serde_json::Result<()>
where
    W: WriteColor,
    T: ?Sized + Serialize,
{
    to_writer_with_theme_and_formatter(writer, value, &Theme::default(), PrettyFormatter::new())
}

pub fn to_writer_compact<W, T>(writer: W, value: &T) -> serde_json::Result<()>
where
    W: WriteColor,
    T: ?Sized + Serialize,
{
    to_writer_with_theme_and_formatter(writer, value, &Theme::default(), CompactFormatter)
}

pub fn to_writer_with_theme<W, T>(writer: W, value: &T, theme: &Theme) -> serde_json::Result<()>
where
    W: WriteColor,
    T: ?Sized + Serialize,
{
    to_writer_with_theme_and_formatter(writer, value, theme, PrettyFormatter::new())
}

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
    let writer = Writer::new(writer);
    let formatter = ColorFormatter::new(&writer, theme, formatter);
    let mut ser = Serializer::with_formatter(&writer, formatter);
    value.serialize(&mut ser)?;
    Ok(())
}

struct ColorFormatter<'a, W, F> {
    formatter: F,
    writer: W,
    theme: &'a Theme,
    writing_key: bool,
}

impl<'a, W, F> ColorFormatter<'a, W, F> {
    fn new(writer: W, theme: &'a Theme, formatter: F) -> Self {
        ColorFormatter {
            formatter,
            writer,
            theme,
            writing_key: false,
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
        self.writer.set_color(&self.theme.null)?;
        self.formatter.write_null(&mut self.writer)?;
        self.writer.reset()?;
        Ok(())
    }

    fn write_bool<U>(&mut self, _: &mut U, value: bool) -> io::Result<()>
    where
        U: ?Sized + Write,
    {
        self.writer.set_color(&self.theme.bool)?;
        self.formatter.write_bool(&mut self.writer, value)?;
        self.writer.reset()?;
        Ok(())
    }

    fn write_i8<U>(&mut self, _: &mut U, value: i8) -> io::Result<()>
    where
        U: ?Sized + Write,
    {
        self.writer.set_color(&self.theme.number)?;
        self.formatter.write_i8(&mut self.writer, value)?;
        self.writer.reset()?;
        Ok(())
    }

    fn write_i16<U>(&mut self, _: &mut U, value: i16) -> io::Result<()>
    where
        U: ?Sized + Write,
    {
        self.writer.set_color(&self.theme.number)?;
        self.formatter.write_i16(&mut self.writer, value)?;
        self.writer.reset()?;
        Ok(())
    }

    fn write_i32<U>(&mut self, _: &mut U, value: i32) -> io::Result<()>
    where
        U: ?Sized + Write,
    {
        self.writer.set_color(&self.theme.number)?;
        self.formatter.write_i32(&mut self.writer, value)?;
        self.writer.reset()?;
        Ok(())
    }

    fn write_i64<U>(&mut self, _: &mut U, value: i64) -> io::Result<()>
    where
        U: ?Sized + Write,
    {
        self.writer.set_color(&self.theme.number)?;
        self.formatter.write_i64(&mut self.writer, value)?;
        self.writer.reset()?;
        Ok(())
    }

    fn write_u8<U>(&mut self, _: &mut U, value: u8) -> io::Result<()>
    where
        U: ?Sized + Write,
    {
        self.writer.set_color(&self.theme.number)?;
        self.formatter.write_u8(&mut self.writer, value)?;
        self.writer.reset()?;
        Ok(())
    }

    fn write_u16<U>(&mut self, _: &mut U, value: u16) -> io::Result<()>
    where
        U: ?Sized + Write,
    {
        self.writer.set_color(&self.theme.number)?;
        self.formatter.write_u16(&mut self.writer, value)?;
        self.writer.reset()?;
        Ok(())
    }

    fn write_u32<U>(&mut self, _: &mut U, value: u32) -> io::Result<()>
    where
        U: ?Sized + Write,
    {
        self.writer.set_color(&self.theme.number)?;
        self.formatter.write_u32(&mut self.writer, value)?;
        self.writer.reset()?;
        Ok(())
    }

    fn write_u64<U>(&mut self, _: &mut U, value: u64) -> io::Result<()>
    where
        U: ?Sized + Write,
    {
        self.writer.set_color(&self.theme.number)?;
        self.formatter.write_u64(&mut self.writer, value)?;
        self.writer.reset()?;
        Ok(())
    }

    fn write_f32<U>(&mut self, _: &mut U, value: f32) -> io::Result<()>
    where
        U: ?Sized + Write,
    {
        self.writer.set_color(&self.theme.number)?;
        self.formatter.write_f32(&mut self.writer, value)?;
        self.writer.reset()?;
        Ok(())
    }

    fn write_f64<U>(&mut self, _: &mut U, value: f64) -> io::Result<()>
    where
        U: ?Sized + Write,
    {
        self.writer.set_color(&self.theme.number)?;
        self.formatter.write_f64(&mut self.writer, value)?;
        self.writer.reset()?;
        Ok(())
    }

    fn write_number_str<U>(&mut self, _: &mut U, value: &str) -> io::Result<()>
    where
        U: ?Sized + Write,
    {
        self.writer.set_color(&self.theme.number)?;
        self.formatter.write_number_str(&mut self.writer, value)?;
        self.writer.reset()?;
        Ok(())
    }

    fn begin_string<U>(&mut self, _: &mut U) -> io::Result<()>
    where
        U: ?Sized + Write,
    {
        if !self.writing_key {
            self.writer.set_color(&self.theme.string)?;
        }
        self.formatter.begin_string(&mut self.writer)
    }

    fn end_string<U>(&mut self, _: &mut U) -> io::Result<()>
    where
        U: ?Sized + Write,
    {
        self.formatter.end_string(&mut self.writer)?;
        if !self.writing_key {
            self.writer.reset()?;
        }
        Ok(())
    }

    fn write_string_fragment<U>(&mut self, _: &mut U, fragment: &str) -> io::Result<()>
    where
        U: ?Sized + Write,
    {
        self.formatter.write_string_fragment(&mut self.writer, fragment)
    }

    fn write_char_escape<U>(&mut self, _: &mut U, char_escape: CharEscape) -> io::Result<()>
    where
        U: ?Sized + Write,
    {
        self.formatter.write_char_escape(&mut self.writer, char_escape)
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
        self.writer.set_color(&self.theme.object_key)?;
        Ok(())
    }

    fn end_object_key<U>(&mut self, _: &mut U) -> io::Result<()>
    where
        U: ?Sized + Write,
    {
        self.writer.reset()?;
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
        self.formatter.write_raw_fragment(&mut self.writer, fragment)
    }
}

struct Writer<W> {
    inner: RefCell<W>,
}

impl<W> Writer<W> {
    fn new(writer: W) -> Self {
        Writer {
            inner: RefCell::new(writer),
        }
    }
}

impl<W> Write for &'_ Writer<W>
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

impl<W> WriteColor for &'_ Writer<W>
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

impl Default for Theme {
    fn default() -> Self {
        let mut theme = Theme {
            null: ColorSpec::new(),
            bool: ColorSpec::new(),
            number: ColorSpec::new(),
            string: ColorSpec::new(),
            object_key: ColorSpec::new(),
        };

        theme.string.set_fg(Some(Color::Green));

        theme.object_key.set_fg(Some(Color::Blue)).set_bold(true);

        theme
    }
}
