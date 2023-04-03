pub mod color;
pub mod flag;

#[cfg(debug_assertions)]
#[macro_export]
macro_rules! assume {
    ($cond:expr, $($msg:literal)?) => {
        if !$cond {
            std::unreachable!($($msg)?)
        }
    }
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! assume {
    ($cond:expr, $($msg:literal)?) => {
        if !$cond {
            unsafe { std::hint::unreachable_unchecked() }
        }
    };
}

#[macro_export]
macro_rules! assume_normal {
    ($e:expr) => {{
        let e = $e;
        $crate::assume!(
            e.is_finite() & !e.is_subnormal(),
            "only normal floats should occur"
        );
        e
    }};
}

pub mod driver;
pub use driver::*;
pub use flag::*;
pub use queercat_proc::*;
// pub mod graphemes;
