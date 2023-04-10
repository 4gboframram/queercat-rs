#![deny(clippy::pedantic)]
//! A library for colorizing text in the style of lgbtq+ pride flags, quickly. This crate is mostly meant as internal details for the `queercat` commandline tool, so documentation may be lacking.

//! # Examples
//! Colorizes `stdin` with the pattern of the lesbian flag using ansi escape codes and writes it to stdout.
//! ```
//! use std::io::{self, BufRead};
//! use queercat_lib::{QueerCat, lesbian, Ansi, QueerCatFrequency};
//! let writer = io::stdout().lock();
//! let mut reader = io::stdin().lock();
//! let flag = lesbian();
//! let colorizer = Ansi::new(flag.ansi_colors.len() as u32, QueerCatFrequency::Original);
//! let mut cat = QueerCat::new(colorizer, writer, flag);
//! cat.cat(reader).unwrap();
//! ```
//! Colorizes `stdin` with the pattern of the trans flag using 24-bit color and writes it to stdout.
//! ```
//! use std::io::{self, BufRead};
//! use queercat_lib::{QueerCat, transgender, Bits24, QueerCatFrequency};
//! let writer = io::stdout().lock();
//! let mut reader = io::stdin().lock();
//! let flag = transgender();
//! let colorizer = Bits24::new(QueerCatFrequency::Original);
//! let mut cat = QueerCat::new(colorizer, writer, flag);
//! cat.cat(reader).unwrap();
//! ```
pub mod color;
pub mod driver;
pub mod flag;
pub use driver::*;
pub use flag::*;
pub mod colorizer;
pub use colorizer::*;
use fixed::{types::extra::U24, types::U0F32, FixedU32};

/// Represents the 0-1 range of color values and theta
pub type ColorV = U0F32;
/// Big enough to hold an 8-bit integer and do precise enough calculations
pub type Extended = FixedU32<U24>;
