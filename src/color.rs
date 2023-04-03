use crate::assume_normal;
use crate::flag::Flag;

fn put_in_range(theta: f32) -> f32 {
    // The original used 2 while loops, but here we're using rem_euclid and seeing how it goes
    theta.rem_euclid(TWO_PI)
}

pub(crate) const TWO_PI: f32 = 6.2831853071795864769252867665590057683943387987502116419498891846;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color {
    red: f32,
    green: f32,
    blue: f32,
}

#[inline(always)]
// 1.0 - ((uwu % 2.0) - 1.0).abs();
fn rainbow_part(f: f32, as_u8: u8) -> f32 {
    match as_u8 {
        0 => f,
        1 => 2.0 - f,
        2 => f - 2.0,
        3 => 4.0 - f,
        4 => f - 4.0,
        5 => 6.0 - f,
        _ => unreachable!(),
    }
}
impl Color {
    pub const fn new(r: f32, g: f32, b: f32) -> Self {
        Self {
            red: r,
            green: g,
            blue: b,
        }
    }
    pub const fn red(&self) -> f32 {
        self.red
    }
    pub const fn blue(&self) -> f32 {
        self.blue
    }
    pub const fn green(&self) -> f32 {
        self.green
    }

    pub fn rainbow(theta: f32) -> Self {
        // let h = put_in_range(theta);
        let theta = theta; // * 0.5;
        let f = theta % 6.0; // * 0.95492965855; // 60 degrees
        let range = f as u8; // (uwu as u32) % 6;
        let x = rainbow_part(f, range);

        match range {
            0 => Self::new(1.0, x, 0.0),
            1 => Self::new(x, 1.0, 0.0),
            2 => Self::new(0.0, 1.0, x),
            3 => Self::new(0.0, x, 1.0),
            4 => Self::new(x, 0.0, 1.0),
            5 => Self::new(1.0, 0.0, x),
            _ => unreachable!(),
        }
    }

    pub fn stripe(theta: f32, flag: &Flag<'_>) -> Self {
        let theta = assume_normal!(theta);
        let theta = put_in_range(theta);
        let stripe_size = TWO_PI / (flag.stripe_colors.len() as f32);

        let i_float = theta / stripe_size;
        let i = unsafe { i_float.to_int_unchecked::<usize>() };
        let stripe_start = i as f32 * stripe_size;
        let balance = 1.0 - (i_float - stripe_start / stripe_size);

        let colors = &flag.stripe_colors;
        let color = colors[i];
        let next_i = if (i + 1) == colors.len() { 0 } else { i + 1 };
        let next_color = colors[next_i];

        color.mix(next_color, balance, flag.factor)
    }

    pub fn mix(self, other: Color, balance: f32, factor: f32) -> Color {
        let balance = assume_normal!(balance);
        let factor = assume_normal!(factor);

        let balance = balance.powf(factor);

        let red = mix_field(self.red(), other.red(), balance);
        let green = mix_field(self.green(), other.green(), balance);
        let blue = mix_field(self.blue(), other.blue(), balance);

        Color::new(red, green, blue)
    }
}

fn mix_field(first: f32, other: f32, balance: f32) -> f32 {
    // this is just a lerp lmao

    const SMALL_BALANCE_THRESHOLD: f32 = 0.125;

    if balance < SMALL_BALANCE_THRESHOLD {
        return other;
    }

    // f * b + o * (1 - b) = f * b - o * b + o
    // = (f - o) * b + o
    // = (f - o).mul_add(b, o)

    (first - other).mul_add(balance, other)
}
