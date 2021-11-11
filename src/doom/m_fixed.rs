//! Original file: m_fixed.c / m_fixed.h
//! 
//! Description: Fixed point arithmetic

// Fixed point, 32-bit as 16.16.
pub const FRAC_BITS: i32 = 16;
pub const FRAC_UNIT: i32 = 1 << FRAC_BITS;

pub type Fixed = i32;

// Fixme. __USE_C_FIXED__ or something.
pub fn fixed_mul(a: Fixed, b: Fixed) -> Fixed
{
    ((a as i64 * b as i64) >> FRAC_BITS) as Fixed
}

// FixedDiv, C version.
pub fn fixed_div(a: Fixed, b: Fixed) -> Fixed
{
    if (a.abs() >> 14) >= b.abs()
    {
        if (a ^ b) < 0 { return Fixed::MIN } else { return Fixed::MAX }
    }

    fixed_div2(a, b)
}

pub fn fixed_div2(a: Fixed, b: Fixed) -> Fixed
{
    let c: f64 = (a as f64 / b as f64) * FRAC_UNIT as f64;

    if !(-2147483648.0 .. 2147483648.0).contains(&c)
    {
        panic!("FixedDiv: divide by zero");
    }

    c as Fixed
}
