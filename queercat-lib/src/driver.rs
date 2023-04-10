//! The main driver for queercat

use crate::colorizer::Colorizer;
use crate::flag::Flag;
use std::io::{self, BufRead, Write};

use unicode_segmentation::UnicodeSegmentation;

/// The main driver struct
pub struct QueerCat<'a, W: Write, C: Colorizer> {
    colorizer: C,
    writer: W,
    flag: Flag<'a>,
}

impl<'a, W: Write, C: Colorizer> QueerCat<'a, W, C> {
    #[must_use]
    pub const fn new(colorizer: C, writer: W, flag: Flag<'a>) -> Self {
        Self {
            colorizer,
            writer,
            flag,
        }
    }

    fn cat_impl<R: BufRead>(&mut self, mut file: R) -> Result<(), io::Error> {
        let mut alt_buf; // when there's an escape, allocate a buffer for skipped escapes
        let mut remaining: Vec<u8> = Vec::new();
        let mut prev_color: C::Color = Default::default();
        let mut process_graphemes = |s: &str| {
            for gr in s.graphemes(true) {
                let state = self.colorizer.update_state(gr);
                let color = C::calculate_color(state, &self.flag);
                if color != prev_color {
                    self.writer.write_fmt(format_args!("{color}"))?;
                    prev_color = color;
                }
                self.writer.write_all(gr.as_bytes())?;
            }
            Ok::<(), std::io::Error>(())
        };
        loop {
            let buf = file.fill_buf()?;
            let buf_len = buf.len();
            if buf.is_empty() {
                break;
            }

            // only use the escape skipper if there is an escape
            // benchmarks suggest that checking for an escape has about a 15% performance increase when there is no escape
            let buf = if memchr::memchr(0x1b, buf).is_some() {
                alt_buf = Vec::with_capacity(buf.len());
                alt_buf.extend(EscapeSkipper::new(buf.iter().copied()));
                &alt_buf
            } else {
                buf
            };

            if remaining.is_empty() {
                match std::str::from_utf8(buf) {
                    Ok(str) => process_graphemes(str)?,
                    Err(e) => {
                        let (s, tail) = buf.split_at(e.valid_up_to());
                        process_graphemes(std::str::from_utf8(s).unwrap())?;
                        remaining.extend_from_slice(tail);
                    }
                }
            } else {
                remaining.extend_from_slice(buf);
                match std::str::from_utf8(&remaining) {
                    Ok(str) => {
                        process_graphemes(str)?;
                        remaining.clear();
                    }
                    Err(e) => {
                        let (s, rem) = remaining.split_at(e.valid_up_to());
                        let str = std::str::from_utf8(s).unwrap();
                        process_graphemes(str)?;
                        // avoid borrow issue
                        let mut new_remaining = Vec::with_capacity(rem.len());
                        new_remaining.extend_from_slice(rem);
                        remaining = new_remaining;
                    }
                }
            }
            file.consume(buf_len);
        }

        Ok(())
    }
    /// Colorizes input from `file` and writes it to the `writer`.
    /// # Errors
    /// Returns `Err` when writing with `self.writer` or reading `file` fails
    pub fn cat<R: BufRead>(&mut self, file: R) -> Result<(), io::Error> {
        let res = self.cat_impl(file);
        self.writer
            .write_fmt(format_args!("{}", C::Resetter::default()))?;
        res
    }
}

use std::iter::Iterator;
struct EscapeSkipper<I>(I);
impl<I> EscapeSkipper<I> {
    const fn new(item: I) -> Self {
        Self(item)
    }
}

impl<I: Iterator<Item = u8>> Iterator for EscapeSkipper<I> {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        let item = self.0.next()?;
        if item != 0x1b {
            return Some(item);
        }
        loop {
            let i = self.0.next()?;

            if i.is_ascii_alphabetic() {
                return self.next(); // recursively skip escapes. this will not stack overflow because of tail-call optimization
            }
        }
    }
}
