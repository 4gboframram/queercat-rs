use crate::flag::Flag;

use crate::colorizer::*;
use std::io::{self, Read, Write};

pub struct QueerCat<'a, W: Write, C: Colorizer> {
    colorizer: C,
    writer: W,
    flag: Flag<'a>,
}

use unicode_reader::{CodePoints, Graphemes};

impl<'a, W: Write, C: Colorizer> QueerCat<'a, W, C> {
    pub const fn new(colorizer: C, writer: W, flag: Flag<'a>) -> Self {
        Self {
            colorizer,
            writer,
            flag,
        }
    }

    fn cat_impl<R: Read>(&mut self, file: R) -> Result<(), io::Error> {
        let iter = EscapeSkipper::new(file.bytes());

        let iter = Graphemes::from(CodePoints::from(iter));
        let mut prev_color: C::Color = Default::default();
        for gr in iter {
            let gr = gr?;
            let state = self.colorizer.update_state(&gr);
            let color = C::calculate_color(state, &self.flag);
            if color != prev_color {
                self.writer.write_fmt(format_args!("{}", color))?;
                prev_color = color;
            }
            self.writer.write(gr.as_bytes())?;
        }

        Ok(())
    }

    pub fn cat<R: Read>(&mut self, file: R) -> Result<(), io::Error> {
        let res = self.cat_impl(file);
        self.writer
            .write_fmt(format_args!("{}", C::Resetter::default()))?;
        res
    }
}

use std::iter::Iterator;
struct EscapeSkipper<I>(I);
impl<I: Iterator<Item = Result<u8, io::Error>>> EscapeSkipper<I> {
    const fn new(item: I) -> Self {
        Self(item)
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

        if i == b'\x1b' {
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
