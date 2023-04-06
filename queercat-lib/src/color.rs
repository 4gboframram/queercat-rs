#![allow(clippy::must_use_candidate)]

use crate::flag::Flag;
use crate::{ColorV, Extended};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct Color {
    red: ColorV,
    green: ColorV,
    blue: ColorV,
}

const fn trunc(v: Extended) -> ColorV {
    if v.to_bits() >= Extended::ONE.to_bits() {
        ColorV::MAX
    } else {
        ColorV::const_from_fixed(v)
    }
}

impl Color {
    #[must_use]
    pub const fn new(r: ColorV, g: ColorV, b: ColorV) -> Self {
        Self {
            red: r,
            green: g,
            blue: b,
        }
    }
    fn from_ext(r: Extended, g: Extended, b: Extended) -> Self {
        Self::new(trunc(r), trunc(g), trunc(b))
    }
    #[must_use]
    pub const fn from_hex(hex: u32) -> Self {
        let r = (hex & 0x00ff_0000) << 8;
        let g = (hex & 0x0000_ff00) << 16;
        let b = (hex & 0x0000_00ff) << 24;
        let r = ColorV::from_bits(r);
        let g = ColorV::from_bits(g);
        let b = ColorV::from_bits(b);
        Self::new(r, g, b)
    }
    pub const fn red(&self) -> ColorV {
        self.red
    }
    pub const fn blue(&self) -> ColorV {
        self.blue
    }
    pub const fn green(&self) -> ColorV {
        self.green
    }

    // essentially just Hsv(theta, 1.0, 1.0) to rgb, but with some quick optimizations that make it inaccurate
    #[must_use]
    pub fn rainbow(theta: ColorV) -> Self {
        const ZERO: Extended = Extended::ZERO;
        const ONE: Extended = Extended::ONE;
        const TWO: Extended = Extended::lit("2.0");
        const FOUR: Extended = Extended::lit("4.0");
        const SIX: Extended = Extended::lit("6.0");

        let theta = Extended::from_num(theta);
        let f = theta * SIX;
        let range = f.round_to_zero().to_num::<u32>();
        // let x = rainbow_part(f, range);

        match range {
            0 => Self::from_ext(ONE, f, ZERO),
            1 => Self::from_ext(TWO - f, ONE, ZERO),
            2 => Self::from_ext(ZERO, ONE, f - TWO),
            3 => Self::from_ext(ZERO, FOUR - f, ONE),
            4 => Self::from_ext(f - FOUR, ZERO, ONE),
            5 => Self::from_ext(ONE, ZERO, SIX - f),
            _ => unreachable!(),
        }
    }

    #[must_use]
    pub fn stripe(theta: ColorV, flag: &Flag<'_>, stripe_size: ColorV) -> Self {
        let theta = Extended::from_num(theta);
        let stripe_size = Extended::from_num(stripe_size);
        let i_float = Extended::from_num(flag.stripe_colors.len()) * theta;
        let i: usize = i_float.to_num::<u32>() as usize;
        let stripe_start = Extended::from_num(i) * stripe_size;
        let balance = Extended::ONE.wrapping_sub(i_float.wrapping_sub(stripe_start / stripe_size));

        let colors = &flag.stripe_colors;
        let color = colors[i];
        let next_i = if (i + 1) == colors.len() { 0 } else { i + 1 };
        let next_color = colors[next_i];

        color.mix(next_color, balance.wrapping_to_num(), flag.factor)
    }

    #[must_use]
    pub fn mix(self, other: Color, balance: ColorV, factor: Extended) -> Color {
        let balance = ColorV::from_num(balance.to_num::<f32>().powf(factor.to_num::<f32>()));

        let red = mix_field(self.red(), other.red(), balance);
        let green = mix_field(self.green(), other.green(), balance);
        let blue = mix_field(self.blue(), other.blue(), balance);

        Color::new(red, green, blue)
    }
}

fn mix_field(first: ColorV, other: ColorV, balance: ColorV) -> ColorV {
    // this is just a lerp lmao

    const SMALL_BALANCE_THRESHOLD: ColorV = ColorV::lit("0.0625");

    if balance < SMALL_BALANCE_THRESHOLD {
        return other;
    }

    // f * b + o * (1 - b) = f * b - o * b + o
    // = (f - o) * b + o
    // = (f - o).mul_add(b, o)

    // first.wrapping_sub(other).wrapping_mul_add(balance, other)
    first * balance + other * (ColorV::MAX - balance)
}

impl std::fmt::Display for Color {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            fmt,
            "\x1b[38;2;{};{};{}m",
            (Extended::from_num(self.red()).wrapping_shl(8)).to_num::<u32>(),
            (Extended::from_num(self.green()).wrapping_shl(8)).to_num::<u32>(),
            (Extended::from_num(self.blue()).wrapping_shl(8)).to_num::<u32>()
        )
    }
}

#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Copy, PartialEq, Default, Eq)]
pub struct AnsiColor(pub u8);

impl std::fmt::Display for AnsiColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\x1b[38;5;{}m", self.0)
    }
}
