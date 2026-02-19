#![no_std]
mod accelerate;

use accelerate::fns::*;

#[derive(Debug, Clone, Copy)]
pub enum AccelerateError {
    /// Inputs and outputs are not all the same length
    LengthMismatch { expected: usize, got: usize },
    /// We get an array that cannot be indexed by i32
    Overflow
}

mod sealed {
    pub trait Sealed {}
    impl Sealed for f32 {}
    impl Sealed for f64 {}
}

pub trait AccelerateFloat: sealed::Sealed + Copy {
    /// # Safety
    /// All inputs must point to valid arrays of floating-point numbers. All must be of the same
    /// type, either f64 or f32, and all arrays must be of length 'count'
    unsafe fn accelerate_pow(out: *mut Self, exp: *const Self, base: *const Self, count: *const i32);
    /// # Safety
    /// All inputs must point to valid arrays of floating-point numbers. All must be of the same
    /// type, either f64 or f32, and all arrays must be of length 'count'
    unsafe fn accelerate_div(out: *mut Self, numerator: *const Self, denominator: *const Self, count: *const i32);

    /// # Safety
    /// All inputs must point to valid arrays of floating-point numbers. All must be of the same
    /// type, either f64 or f32, and all arrays must be of length 'count'
    unsafe fn accelerate_ceil(out: *mut Self, input: *const Self, count: *const i32);
}

impl AccelerateFloat for f64 {
    unsafe fn accelerate_pow(out: *mut Self, exp: *const Self, base: *const Self, count: *const i32) {
        unsafe { vvpow(out, exp, base, count) };
    }
    unsafe fn accelerate_div(out: *mut Self, numerator: *const Self, denominator: *const Self, count: *const i32) {
        unsafe { vvdiv(out, numerator, denominator, count) };
    }
    unsafe fn accelerate_ceil(out: *mut Self, input: *const Self, count: *const i32) {
        unsafe { vvceil(out, input, count); }
    }
}

impl AccelerateFloat for f32 {
    unsafe fn accelerate_pow(out: *mut Self, exp: *const Self, base: *const Self, count: *const i32) {
        unsafe { vvpowf(out, exp, base, count) };
    }
    unsafe fn accelerate_div(out: *mut Self, numerator: *const Self, denominator: *const Self, count: *const i32) {
        unsafe { vvdivf(out, numerator, denominator, count) };
    }
    unsafe fn accelerate_ceil(out: *mut Self, input: *const Self, count: *const i32) {
        unsafe { vvceilf(out, input, count); }
    }
}


// todo:
// check what accelerate does if we pass in count = 0

macro_rules! binary_vforce_op {
    (
    $(#[$out_attr:meta])*
    $name:ident, 
    $(#[$in_place_attr:meta])*
    $name_in_place:ident, 
    $method:ident,
    $a_name:ident, 
    $b_name:ident
    ) => {
        $(#[$out_attr])*
        pub fn $name<AF: AccelerateFloat>(
            out: &mut [AF], $a_name: &[AF], $b_name: &[AF]
        ) -> Result<(), AccelerateError> {
            let count = validate_lengths_2($a_name.len(), $b_name.len(), out.len())?;
            unsafe { AF::$method(out.as_mut_ptr(), $a_name.as_ptr(), $b_name.as_ptr(), &count); }
            Ok(())
        }
        $(#[$in_place_attr])*
        pub fn $name_in_place<AF: AccelerateFloat>(
            $a_name: &mut [AF], $b_name: &[AF]
        ) -> Result<(), AccelerateError> {
            let count = validate_lengths_1($a_name.len(), $b_name.len())?;
            unsafe { AF::$method($a_name.as_mut_ptr(), $a_name.as_ptr(), $b_name.as_ptr(), &count); }
            Ok(())
        }
    };
}

macro_rules! unary_vforce_op {
    (
    $(#[$out_attr:meta])*
    $name:ident, 
    $(#[$in_place_attr:meta])*
    $name_in_place:ident, 
    $method:ident,
    $input_name:ident
    ) => {
        $(#[$out_attr])*
        pub fn $name<AF: AccelerateFloat>(
            out: &mut [AF], $input_name: &[AF]
        ) -> Result<(), AccelerateError> {
            let count = validate_lengths_1($input_name.len(), out.len())?;
            unsafe { AF::$method(out.as_mut_ptr(), $input_name.as_ptr(), &count); }
            Ok(())
        }
        $(#[$in_place_attr])*
        pub fn $name_in_place<AF: AccelerateFloat>(
            $input_name: &mut [AF]
        ) -> Result<(), AccelerateError> {
            let count: i32 = $input_name.len()
                .try_into()
                .map_err(|_| AccelerateError::Overflow)?;
            unsafe { AF::$method($input_name.as_mut_ptr(), $input_name.as_ptr(), &count); }
            Ok(())
        }
    };
}

fn validate_lengths_1(a: usize, b: usize) -> Result<i32, AccelerateError> {
    let count: i32 = a.try_into().map_err(|_| AccelerateError::Overflow)?;
    if a != b {
        return Err(AccelerateError::LengthMismatch { expected: a, got: b });
    }
    Ok(count)
}
fn validate_lengths_2(a: usize, b: usize, c: usize) -> Result<i32, AccelerateError> {
    let count: i32 = a.try_into().map_err(|_| AccelerateError::Overflow)?;
    if a != b {
        return Err(AccelerateError::LengthMismatch { expected: a, got: b });
    }
    if a != c {
        return Err(AccelerateError::LengthMismatch { expected: a, got: c });
    }
    Ok(count)
}


binary_vforce_op!(pow_array, pow_array_in_place, accelerate_pow, bases, exponents);
binary_vforce_op!(div_array, div_array_in_place, accelerate_div, numerator, denominator);

unary_vforce_op!(ceil_array, ceil_array_in_place, accelerate_ceil, input);



//
// /// Equivalent to base = base.powf(exponent)
// /// can only be used on arrays up to 4_294_967_295 elements long
// ///
// /// Will overwrite the 'bases' array with the results of bases ^ exponents
// /// Providing a safe API over the Accelerate vvpow() function:
// pub fn pow_array_in_place<AF: AccelerateFloat>(bases: &mut [AF], exponents: &[AF]) -> Result<(), AccelerateError> {
//     let count: i32 = bases.len()
//         .try_into()
//         .map_err(|_| AccelerateError::Overflow)?;
//
//     if count != exponents.len() as i32 {
//         Err(AccelerateError::LengthMismatch { expected: bases.len(), got: exponents.len() })
//     } else {
//         unsafe {
//             AF::accelerate_pow(bases.as_mut_ptr(), exponents.as_ptr(), bases.as_ptr(), &count);
//         }
//         Ok(())
//     }
// }
//
//
// pub fn pow_array<AF: AccelerateFloat>(out: &mut [AF], bases: &[AF], exponents: &[AF]) -> Result<(), AccelerateError> {
//     let count: i32 = bases.len()
//         .try_into()
//         .map_err(|_| AccelerateError::Overflow)?;
//
//     if count != exponents.len() as i32 {
//         Err(AccelerateError::LengthMismatch { expected: bases.len(), got: exponents.len() })
//     }else if count != out.len() as i32 {
//         Err(AccelerateError::LengthMismatch { expected: bases.len(), got: out.len() })
//     } else {
//         unsafe {
//             AF::accelerate_pow(out.as_mut_ptr(), exponents.as_ptr(), bases.as_ptr(), &count);
//         }
//         Ok(())
//     }
// }
//
//
// pub fn div_array_in_place<AF: AccelerateFloat>(numerator: &mut [AF], denominator: &[AF]) -> Result<(), AccelerateError> {
//     let count: i32 = numerator.len()
//         .try_into()
//         .map_err(|_| AccelerateError::Overflow)?;
//
//     if count != denominator.len() as i32 {
//         Err(AccelerateError::LengthMismatch { expected: numerator.len(), got: denominator.len() })
//     } else {
//         unsafe {
//             AF::accelerate_div(numerator.as_mut_ptr(), numerator.as_ptr(), denominator.as_ptr(), &count);
//         }
//         Ok(())
//     }
// }
//
//
// pub fn div_array<AF: AccelerateFloat>(out: &mut [AF], numerator: &[AF], denominator: &[AF]) -> Result<(), AccelerateError> {
//     let count: i32 = numerator.len()
//         .try_into()
//         .map_err(|_| AccelerateError::Overflow)?;
//
//     if count != denominator.len() as i32 {
//         Err(AccelerateError::LengthMismatch { expected: numerator.len(), got: denominator.len() })
//     } else if count != out.len() as i32 {
//         Err(AccelerateError::LengthMismatch { expected: numerator.len(), got: out.len() })
//     } else {
//         unsafe {
//             AF::accelerate_div(out.as_mut_ptr(), numerator.as_ptr(), denominator.as_ptr(), &count);
//         }
//         Ok(())
//     }
// }
