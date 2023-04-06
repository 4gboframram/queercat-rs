use crate::flag::Flag;

use crate::colorizer::Colorizer;
use std::io::{self, Read, Write};

pub struct QueerCat<'a, W: Write, C: Colorizer> {
    colorizer: C,
    writer: W,
    flag: Flag<'a>,
}

use unicode_reader::{CodePoints, Graphemes};

impl<'a, W: Write, C: Colorizer> QueerCat<'a, W, C> {
    #[must_use]
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
                self.writer.write_fmt(format_args!("{color}"))?;
                prev_color = color;
            }
            self.writer.write_all(gr.as_bytes())?;
        }

        Ok(())
    }
    
    /// # Errors
    /// Returns `Err` when writing with `self.writer` or reading `file` fails
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
        if let Ok(b) = item {
            if b != 0x1b {
                return Some(item);
            }
            loop {
                match self.0.next()? {
                    Ok(n) if n.is_ascii_alphabetic() => {
                        return self.next(); // recursively skip escapes. this will not stack overflow because of tail-call optimization
                    }
                    Ok(_) => {}
                    v => { return Some(v); }
                }
            }
        }
        Some(item)
    }
}
