#![allow(clippy::must_use_candidate)]

use crate::color::{AnsiColor, Color};
use crate::flag::Flag;
use crate::{ColorV, Extended};

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

impl QueerCatFrequency {
    pub fn as_freq(self) -> (Extended, Extended) {
        use QueerCatFrequency::{Aaaaaaaaaaaaa, Custom, Fast, Horizontal, HyperGay, Original, UltraHyperGay, Vertical};
        
        let (freq_v, freq_h) = match self {
            Original => (0.23, 0.1),
            Fast => (0.4, 0.4),
            HyperGay => (0.9, 0.9),
            UltraHyperGay => (1.4, 1.4),
            Aaaaaaaaaaaaa => (3.0, 3.0),
            Vertical(v) => (v, 0.0),
            Horizontal(h) => (0.0, h),
            Custom(v, x) => (v, x),
        };

        (Extended::from_num(freq_h), Extended::from_num(freq_v))
    }
}

#[derive(Default)]
pub struct TerminalResetter;
impl std::fmt::Display for TerminalResetter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\x1b[0m")
    }
}

pub trait Colorizer {
    type Color: std::fmt::Display + PartialEq<Self::Color> + Default;
    type Resetter: std::fmt::Display + Default;
    type State;
    fn calculate_color(state: Self::State, flag: &Flag<'_>) -> Self::Color;
    fn update_state(&mut self, grapheme: &str) -> Self::State;
}

pub struct Bits24 {
    theta: ColorV,
    col_theta: ColorV,
    freq_v: Extended,
    freq_h: Extended,
}

impl Bits24 {
    #[must_use]
    pub fn new(freq: QueerCatFrequency) -> Self {
        let (freq_h, freq_v) = freq.as_freq();
        Self {
            freq_h: freq_h / 15,
            freq_v: freq_v / 30,
            theta: ColorV::ZERO,
            col_theta: ColorV::ZERO,
        }
    }
    #[must_use]
    pub fn with_offset(mut self, offset: f32) -> Self {
        self.theta += ColorV::wrapping_from_num(offset / 12.0);
        self
    }
}


impl Colorizer for Bits24 {
    type State = ColorV;
    type Color = Color;
    type Resetter = TerminalResetter;

    fn calculate_color(state: Self::State, flag: &Flag<'_>) -> Self::Color {
        match flag.color_method {
            crate::ColorMethod::Rainbow => Color::rainbow(state),
            crate::ColorMethod::Stripes => {
                let stripe_size = Extended::from_num(flag.stripe_colors.len()).recip();
                let stripe_size = ColorV::wrapping_from_num(stripe_size);
                Color::stripe(state, flag, stripe_size)
            }
        }
    }
    fn update_state(&mut self, grapheme: &str) -> Self::State {
        if let [b'\n'] = grapheme.as_bytes() {
            self.theta = self
                    .theta
                    .wrapping_add(ColorV::wrapping_from_num(self.freq_v));
                self.theta = self.theta.wrapping_sub(self.col_theta);
                self.col_theta = ColorV::ZERO;
        }
        else {
            let theta = ColorV::wrapping_from_num(self.freq_h);
                self.col_theta = self.col_theta.wrapping_add(theta);
                self.theta = self.theta.wrapping_add(theta);
        }
        self.theta
    }
}

pub struct Ansi {
    line: u32,
    col: u32,
    flag_len: u32,
    offset: Extended,
    freq_v: Extended,
    freq_h: Extended,
}

impl Colorizer for Ansi {
    type Color = AnsiColor;
    type Resetter = TerminalResetter;
    type State = u32;

    fn calculate_color(ansi_index: Self::State, flag: &Flag<'_>) -> Self::Color {
        AnsiColor(flag.ansi_colors[ansi_index as usize])
    }

    fn update_state(&mut self, grapheme: &str) -> Self::State {
        if let [b'\n'] = grapheme.as_bytes() {
            self.col = 0;
            self.line += 1;
        } else {
            self.col += 1;
        }
        (((self.offset.wrapping_mul_int(self.flag_len))
            + (self.col * self.freq_v + self.line * self.freq_h))
            .to_num::<u32>())
            % self.flag_len
    }
}

impl Ansi {
    #[must_use]
    pub fn new(flag_len: u32, freq: QueerCatFrequency) -> Self {
        let (freq_h, freq_v) = freq.as_freq();
        Self {
            freq_h,
            freq_v,
            flag_len,
            line: 0,
            col: 0,
            offset: Extended::ZERO,
        }
    }
#[must_use]
    pub fn with_offset(self, offset: f32) -> Self {
        let offset = Extended::from_num(offset);
        Self { offset, ..self }
    }
}
