use crate::color::*;
use crate::flag::{ColorMethod, Flag};
use std::io::{self, Read, Write};
pub enum ColorType {
    Bits24,
    Ascii,
}

#[derive(Debug, Clone, Copy)]
pub enum QueerCatFrequency {
    Original,
    Fast,
    HyperGay,
    UltraHyperGay,
    Aaaaaaaaaaaaa,
    Vertical(f32),
    Horizontal(f32),
    Custom(f32, f32),
}

pub struct State<'a, W: Write> {
    flag: Flag<'a>,
    writer: W,
    line_index: usize,
    col_index: usize,
    freq_v: f32,
    freq_h: f32,
    offset: f32,
}

impl<'a, W: Write> State<'a, W> {
    pub fn with_freq(self, freq: QueerCatFrequency) -> Self {
        use QueerCatFrequency::*;
        let (freq_v, freq_h) = match freq {
            Original => (0.1, 0.23),
            Fast => (0.4, 0.4),
            HyperGay => (0.9, 0.9),
            UltraHyperGay => (1.4, 1.4),
            Aaaaaaaaaaaaa => (3.0, 3.0),
            Vertical(v) => (v, 0.0),
            Horizontal(h) => (0.0, h),
            Custom(v, x) => (v, x),
        };

        Self {
            freq_h,
            freq_v,
            ..self
        }
    }

    pub const fn new(flag: Flag<'a>, writer: W) -> Self {
        Self {
            flag,
            writer,
            line_index: 0,
            col_index: 0,
            freq_v: 0.1,
            freq_h: 0.23,
            offset: 0.0,
        }
    }
    pub const fn with_offset(mut self, offset: f32) -> Self {
        self.offset = offset;
        self
    }
}

pub trait Colorizer<'a, W: Write> {
    fn print_color(&mut self) -> Result<(), io::Error>;
    fn state(&mut self) -> &mut State<'a, W>;

    fn write_grapheme(&mut self, grapheme: &str) -> Result<usize, io::Error> {
        match grapheme.as_bytes() {
            [b'\n'] => {
                self.state().col_index = 0;
                self.state().line_index += 1;
            }
            _ => {
                self.state().col_index += 1;
            }
        };
        self.print_color()?;
        self.state().writer.write(grapheme.as_bytes())
    }

    fn reset_colors(&mut self) -> Result<usize, io::Error> {
        self.state().writer.write(b"\x1b[0m")
    }
}

pub struct AnsiColorizer<'a, W: Write> {
    state: State<'a, W>,
    previous_ansi_code_index: usize,
}

impl<'a, W: Write> AnsiColorizer<'a, W> {
    pub const fn new(state: State<'a, W>) -> Self {
        Self {
            state,
            previous_ansi_code_index: usize::MAX,
        }
    }
}

impl<'a, W: Write> Colorizer<'a, W> for AnsiColorizer<'a, W> {
    fn print_color(&mut self) -> Result<(), io::Error> {
        let state = &mut self.state;
        let next_ansi_code_index = (state.offset * (state.flag.ansi_colors.len() as f32)) as usize
            + ((state.col_index as f32) * state.freq_v + (state.line_index as f32) * state.freq_h)
                as usize;

        if next_ansi_code_index != self.previous_ansi_code_index {
            self.previous_ansi_code_index = next_ansi_code_index;
            let code = state.flag.ansi_colors[next_ansi_code_index % state.flag.ansi_colors.len()];
            state.writer.write_fmt(format_args!("\x1b[38;5;{}m", code))
        } else {
            Ok(())
        }
    }

    fn state(&mut self) -> &mut State<'a, W> {
        &mut self.state
    }
}

pub struct Bits24Colorizer<'a, W: Write> {
    state: State<'a, W>,
    previous_color: Color,
}

impl<'a, W: Write> Bits24Colorizer<'a, W> {
    pub const fn new(state: State<'a, W>) -> Self {
        Self {
            state,
            previous_color: Color::new(0.0, 0.0, 0.0),
        }
    }
}

impl<'a, W: Write> Colorizer<'a, W> for Bits24Colorizer<'a, W> {
    fn print_color(&mut self) -> Result<(), io::Error> {
        let state = &mut self.state;
        let theta = (state.col_index as f32) * state.freq_v * 0.2// / 5.0
            + (state.line_index as f32) * state.freq_h
            + (state.offset * core::f32::consts::PI);

        let color = match state.flag.color_method {
            ColorMethod::Rainbow => Color::rainbow(theta),

            ColorMethod::Stripes => Color::stripe(theta, &state.flag),
        };

        if color != self.previous_color {
            self.previous_color = color;

            state.writer.write_fmt(format_args!(
                "\x1b[38;2;{};{};{}m",
                (color.red() * 255.0) as u8,
                (color.green() * 255.0) as u8,
                (color.blue() * 255.0) as u8
            ))
        } else {
            Ok(())
        }
    }
    fn state(&mut self) -> &mut State<'a, W> {
        &mut self.state
    }
}
use core::marker::PhantomData;

pub struct QueerCat<'a, W: Write, C: Colorizer<'a, W>> {
    colorizer: C,
    _phantom: PhantomData<&'a W>,
}

use unicode_reader::{CodePoints, Graphemes};

impl<'a, W: Write, C: Colorizer<'a, W>> QueerCat<'a, W, C> {
    pub const fn new(colorizer: C) -> Self {
        Self {
            colorizer,
            _phantom: PhantomData,
        }
    }

    fn cat_impl<R: Read>(&mut self, file: R) -> Result<(), io::Error> {
        let iter = EscapeSkipper::new(file.bytes());

        let iter = Graphemes::from(CodePoints::from(iter));
        for gr in iter {
            self.colorizer.write_grapheme(&gr?)?;
        }
        Ok(())
    }

    pub fn cat<R: Read>(&mut self, file: R) -> Result<(), io::Error> {
        let res = self.cat_impl(file);
        self.colorizer.reset_colors()?;
        res
    }
}

use std::iter::Iterator;
pub struct EscapeSkipper<I>(I);
impl<I: Iterator<Item = Result<u8, io::Error>>> EscapeSkipper<I> {
    pub const fn new(item: I) -> Self {
        Self(item)
    }
}

#[inline(always)]
/// Brings [unlikely](core::intrinsics::unlikely) to stable rust.
pub const fn unlikely(b: bool) -> bool {
    #[allow(clippy::needless_bool)]
    if (1i32).checked_div(if b { 0 } else { 1 }).is_none() {
        true
    } else {
        false
    }
}

impl<I: Iterator<Item = Result<u8, io::Error>>> Iterator for EscapeSkipper<I> {
    type Item = Result<u8, io::Error>;
    fn next(&mut self) -> Option<Self::Item> {
        let item = self.0.next()?;
        let i = match item {
            Ok(v) => v,
            Err(_) => {
                return Some(item);
            }
        };
        if unlikely(i == b'\x1b') {
            while let Some(n) = self.0.next() {
                match n {
                    Ok(c) if c.is_ascii_alphabetic() => {
                        return self.0.next();
                    }
                    Err(_) => {
                        return Some(n);
                    }
                    _ => {}
                }
            }
            None
        } else {
            Some(Ok(i))
        }
    }
}
