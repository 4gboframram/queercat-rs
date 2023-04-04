pub mod color;
pub mod flag;

pub mod driver;
pub use driver::*;
pub use flag::*;
use fixed::{types::extra::*, types::U0F32, FixedU32};
// represents the 0-1 range of color values and theta 
pub type ColorV = U0F32;
// big enough to hold an 8-bit integer and do precise calculations
pub type Extended = FixedU32<U24>;
