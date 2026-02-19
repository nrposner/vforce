#![no_std]
mod accelerate;

use accelerate::fns::*;

#[cfg(target_os = "macos")]
/// Equivalent to base = base.powf(exponent)
/// can only be used on arrays up to 4_294_967_295 elements long
pub fn pow_array_in_place(bases: &mut [f64], exponents: &[f64]) -> Result<(), &'static str> {
    let count = bases.len() as i32;
    
    if count == exponents.len() as i32 {
        unsafe {
            vvpow(bases.as_mut_ptr(), exponents.as_ptr(), bases.as_ptr(), &count);
        }
        Ok(())
    } else {
        Err("Base and Exponent are not of equal length")
    }
}

/// Equivalent to y /= x
pub fn div_array_in_place(numerator: &mut [f64], denominator: &[f64]) -> Result<(), &'static str> {
    let count = numerator.len() as i32;
    
    if count == denominator.len() as i32 {
        unsafe {
            vvdiv(numerator.as_mut_ptr(), numerator.as_ptr(), denominator.as_ptr(), &count);
        }
        Ok(())
    } else {
        Err("Numerator and Denominator are not of equal length")
    }
}

// do we need a fallback??
// #[cfg(not(target_os = "macos"))]
// fn pow_array(bases: &mut [f64], exponents: &[f64]) {
//     for (b, e) in bases.iter_mut().zip(exponents) {
//         *b = b.powf(*e);
//     }
// }

