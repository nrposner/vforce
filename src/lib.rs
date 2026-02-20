#![no_std]
#![cfg(target_os = "macos")]
//! Docs go here

mod accelerate;

use core::fmt::Display;
use accelerate::{AccelerateComplex, fns::*};

#[derive(Debug, Clone, Copy)]
pub enum AccelerateError {
    /// Inputs and outputs are not all the same length
    LengthMismatch { expected: usize, got: usize },
    /// We get an array that cannot be indexed by i32
    Overflow
}

impl Display for AccelerateError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Overflow => {
                write!(f, "AccelerateError::Overflow - vforce recieved an array that cannot be indexed by an i32: please use a shorter array")
            },
            Self::LengthMismatch { expected, got } => {
                write!(f, "AccelerateError::LengthMismatch - vforce recived arrays of different lengths: expected {} elements, got {} elements", expected, got)
            }
        }
    }
}

mod sealed {
    pub trait Sealed {}
    impl Sealed for f32 {}
    impl Sealed for f64 {}
}

/// Ensures that all inputs to an accelerate function must be the same numeric type: either f64 or
/// f32
pub trait AccelerateFloat: sealed::Sealed + Copy {
    // Binary operations (out, a, b, count)
    /// # Safety
    /// All inputs must point to valid arrays of floating-point numbers. All must be of the same
    /// type, either f64 or f32, and all arrays must be of length 'count'
    unsafe fn accelerate_pow(out: *mut Self, base: *const Self, exp: *const Self, count: *const i32);
    /// # Safety
    /// All inputs must point to valid arrays of floating-point numbers. All must be of the same
    /// type, either f64 or f32, and all arrays must be of length 'count'
    unsafe fn accelerate_div(out: *mut Self, numerator: *const Self, denominator: *const Self, count: *const i32);
    /// # Safety
    /// All inputs must point to valid arrays of floating-point numbers. All must be of the same
    /// type, either f64 or f32, and all arrays must be of length 'count'
    unsafe fn accelerate_copysign(out: *mut Self, magnitude: *const Self, sign: *const Self, count: *const i32);
    /// # Safety
    /// All inputs must point to valid arrays of floating-point numbers. All must be of the same
    /// type, either f64 or f32, and all arrays must be of length 'count'
    unsafe fn accelerate_fmod(out: *mut Self, numerator: *const Self, denominator: *const Self, count: *const i32);
    /// # Safety
    /// All inputs must point to valid arrays of floating-point numbers. All must be of the same
    /// type, either f64 or f32, and all arrays must be of length 'count'
    unsafe fn accelerate_remainder(out: *mut Self, numerator: *const Self, denominator: *const Self, count: *const i32);
    /// # Safety
    /// All inputs must point to valid arrays of floating-point numbers. All must be of the same
    /// type, either f64 or f32, and all arrays must be of length 'count'
    unsafe fn accelerate_nextafter(out: *mut Self, input: *const Self, direction: *const Self, count: *const i32);
    /// # Safety
    /// All inputs must point to valid arrays of floating-point numbers. All must be of the same
    /// type, either f64 or f32, and all arrays must be of length 'count'
    unsafe fn accelerate_atan2(out: *mut Self, y: *const Self, x: *const Self, count: *const i32);

    // Unary operations (out, input, count)

    /// # Safety
    /// All inputs must point to valid arrays of floating-point numbers. All must be of the same
    /// type, either f64 or f32, and all arrays must be of length 'count'
    unsafe fn accelerate_ceil(out: *mut Self, input: *const Self, count: *const i32);
    /// # Safety
    /// All inputs must point to valid arrays of floating-point numbers. All must be of the same
    /// type, either f64 or f32, and all arrays must be of length 'count'
    unsafe fn accelerate_floor(out: *mut Self, input: *const Self, count: *const i32);
    /// # Safety
    /// All inputs must point to valid arrays of floating-point numbers. All must be of the same
    /// type, either f64 or f32, and all arrays must be of length 'count'
    unsafe fn accelerate_fabs(out: *mut Self, input: *const Self, count: *const i32);
    /// # Safety
    /// All inputs must point to valid arrays of floating-point numbers. All must be of the same
    /// type, either f64 or f32, and all arrays must be of length 'count'
    unsafe fn accelerate_int(out: *mut Self, input: *const Self, count: *const i32);
    /// # Safety
    /// All inputs must point to valid arrays of floating-point numbers. All must be of the same
    /// type, either f64 or f32, and all arrays must be of length 'count'
    unsafe fn accelerate_nint(out: *mut Self, input: *const Self, count: *const i32);
    /// # Safety
    /// All inputs must point to valid arrays of floating-point numbers. All must be of the same
    /// type, either f64 or f32, and all arrays must be of length 'count'
    unsafe fn accelerate_rsqrt(out: *mut Self, input: *const Self, count: *const i32);
    /// # Safety
    /// All inputs must point to valid arrays of floating-point numbers. All must be of the same
    /// type, either f64 or f32, and all arrays must be of length 'count'
    unsafe fn accelerate_sqrt(out: *mut Self, input: *const Self, count: *const i32);
    /// # Safety
    /// All inputs must point to valid arrays of floating-point numbers. All must be of the same
    /// type, either f64 or f32, and all arrays must be of length 'count'
    unsafe fn accelerate_rec(out: *mut Self, input: *const Self, count: *const i32);
    /// # Safety
    /// All inputs must point to valid arrays of floating-point numbers. All must be of the same
    /// type, either f64 or f32, and all arrays must be of length 'count'
    unsafe fn accelerate_exp(out: *mut Self, input: *const Self, count: *const i32);
    /// # Safety
    /// All inputs must point to valid arrays of floating-point numbers. All must be of the same
    /// type, either f64 or f32, and all arrays must be of length 'count'
    unsafe fn accelerate_exp2(out: *mut Self, input: *const Self, count: *const i32);
    /// # Safety
    /// All inputs must point to valid arrays of floating-point numbers. All must be of the same
    /// type, either f64 or f32, and all arrays must be of length 'count'
    unsafe fn accelerate_expm1(out: *mut Self, input: *const Self, count: *const i32);
    /// # Safety
    /// All inputs must point to valid arrays of floating-point numbers. All must be of the same
    /// type, either f64 or f32, and all arrays must be of length 'count'
    unsafe fn accelerate_log(out: *mut Self, input: *const Self, count: *const i32);
    /// # Safety
    /// All inputs must point to valid arrays of floating-point numbers. All must be of the same
    /// type, either f64 or f32, and all arrays must be of length 'count'
    unsafe fn accelerate_log1p(out: *mut Self, input: *const Self, count: *const i32);
    /// # Safety
    /// All inputs must point to valid arrays of floating-point numbers. All must be of the same
    /// type, either f64 or f32, and all arrays must be of length 'count'
    unsafe fn accelerate_log2(out: *mut Self, input: *const Self, count: *const i32);
    /// # Safety
    /// All inputs must point to valid arrays of floating-point numbers. All must be of the same
    /// type, either f64 or f32, and all arrays must be of length 'count'
    unsafe fn accelerate_log10(out: *mut Self, input: *const Self, count: *const i32);
    /// # Safety
    /// All inputs must point to valid arrays of floating-point numbers. All must be of the same
    /// type, either f64 or f32, and all arrays must be of length 'count'
    unsafe fn accelerate_logb(out: *mut Self, input: *const Self, count: *const i32);
    /// # Safety
    /// All inputs must point to valid arrays of floating-point numbers. All must be of the same
    /// type, either f64 or f32, and all arrays must be of length 'count'
    unsafe fn accelerate_sin(out: *mut Self, input: *const Self, count: *const i32);
    /// # Safety
    /// All inputs must point to valid arrays of floating-point numbers. All must be of the same
    /// type, either f64 or f32, and all arrays must be of length 'count'
    unsafe fn accelerate_sinpi(out: *mut Self, input: *const Self, count: *const i32);
    /// # Safety
    /// All inputs must point to valid arrays of floating-point numbers. All must be of the same
    /// type, either f64 or f32, and all arrays must be of length 'count'
    unsafe fn accelerate_cos(out: *mut Self, input: *const Self, count: *const i32);
    /// # Safety
    /// All inputs must point to valid arrays of floating-point numbers. All must be of the same
    /// type, either f64 or f32, and all arrays must be of length 'count'
    unsafe fn accelerate_cospi(out: *mut Self, input: *const Self, count: *const i32);
    /// # Safety
    /// All inputs must point to valid arrays of floating-point numbers. All must be of the same
    /// type, either f64 or f32, and all arrays must be of length 'count'
    unsafe fn accelerate_tan(out: *mut Self, input: *const Self, count: *const i32);
    /// # Safety
    /// All inputs must point to valid arrays of floating-point numbers. All must be of the same
    /// type, either f64 or f32, and all arrays must be of length 'count'
    unsafe fn accelerate_tanpi(out: *mut Self, input: *const Self, count: *const i32);
    /// # Safety
    /// All inputs must point to valid arrays of floating-point numbers. All must be of the same
    /// type, either f64 or f32, and all arrays must be of length 'count'
    unsafe fn accelerate_asin(out: *mut Self, input: *const Self, count: *const i32);
    /// # Safety
    /// All inputs must point to valid arrays of floating-point numbers. All must be of the same
    /// type, either f64 or f32, and all arrays must be of length 'count'
    unsafe fn accelerate_acos(out: *mut Self, input: *const Self, count: *const i32);
    /// # Safety
    /// All inputs must point to valid arrays of floating-point numbers. All must be of the same
    /// type, either f64 or f32, and all arrays must be of length 'count'
    unsafe fn accelerate_atan(out: *mut Self, input: *const Self, count: *const i32);
    /// # Safety
    /// All inputs must point to valid arrays of floating-point numbers. All must be of the same
    /// type, either f64 or f32, and all arrays must be of length 'count'
    unsafe fn accelerate_sinh(out: *mut Self, input: *const Self, count: *const i32);
    /// # Safety
    /// All inputs must point to valid arrays of floating-point numbers. All must be of the same
    /// type, either f64 or f32, and all arrays must be of length 'count'
    unsafe fn accelerate_cosh(out: *mut Self, input: *const Self, count: *const i32);
    /// # Safety
    /// All inputs must point to valid arrays of floating-point numbers. All must be of the same
    /// type, either f64 or f32, and all arrays must be of length 'count'
    unsafe fn accelerate_tanh(out: *mut Self, input: *const Self, count: *const i32);
    /// # Safety
    /// All inputs must point to valid arrays of floating-point numbers. All must be of the same
    /// type, either f64 or f32, and all arrays must be of length 'count'
    unsafe fn accelerate_asinh(out: *mut Self, input: *const Self, count: *const i32);
    /// # Safety
    /// All inputs must point to valid arrays of floating-point numbers. All must be of the same
    /// type, either f64 or f32, and all arrays must be of length 'count'
    unsafe fn accelerate_acosh(out: *mut Self, input: *const Self, count: *const i32);
    /// # Safety
    /// All inputs must point to valid arrays of floating-point numbers. All must be of the same
    /// type, either f64 or f32, and all arrays must be of length 'count'
    unsafe fn accelerate_atanh(out: *mut Self, input: *const Self, count: *const i32);

    // Special: sincos (sin_out, cos_out, input, count)
    /// # Safety
    /// All inputs must point to valid arrays of floating-point numbers. All must be of the same
    /// type, either f64 or f32, and all arrays must be of length 'count'
    unsafe fn accelerate_sincos(sin_out: *mut Self, cos_out: *mut Self, input: *const Self, count: *const i32);

    // Special: cosisin
    /// # Safety
    /// All inputs must point to valid arrays of floating-point numbers. All must be of the same
    /// type, either f64 or f32, and all arrays must be of length 'count'
    unsafe fn accelerate_cosisin(out: *mut AccelerateComplex<Self>, input: *const Self, count: *const i32);
}

macro_rules! impl_accelerate_float {
    ($ty:ty, $pow:ident, $div:ident, $copysign:ident, $fmod:ident, $remainder:ident,
     $nextafter:ident, $atan2:ident, $ceil:ident, $floor:ident, $fabs:ident,
     $int:ident, $nint:ident, $rsqrt:ident, $sqrt:ident, $rec:ident,
     $exp:ident, $exp2:ident, $expm1:ident, $log:ident, $log1p:ident,
     $log2:ident, $log10:ident, $logb:ident, $sin:ident, $sinpi:ident,
     $cos:ident, $cospi:ident, $tan:ident, $tanpi:ident, $asin:ident,
     $acos:ident, $atan:ident, $sinh:ident, $cosh:ident, $tanh:ident,
     $asinh:ident, $acosh:ident, $atanh:ident, $sincos:ident, $cosisin:ident) => {
        impl AccelerateFloat for $ty {
            unsafe fn accelerate_pow(out: *mut Self, base: *const Self, exp: *const Self, count: *const i32)
            { unsafe { $pow(out, exp, base, count) } }
            unsafe fn accelerate_div(out: *mut Self, n: *const Self, d: *const Self, count: *const i32) 
            { unsafe { $div(out, n, d, count) } }
            unsafe fn accelerate_copysign(out: *mut Self, m: *const Self, s: *const Self, count: *const i32) 
            { unsafe { $copysign(out, m, s, count) } }
            unsafe fn accelerate_fmod(out: *mut Self, n: *const Self, d: *const Self, count: *const i32) 
            { unsafe { $fmod(out, n, d, count) } }
            unsafe fn accelerate_remainder(out: *mut Self, n: *const Self, d: *const Self, count: *const i32) 
            { unsafe { $remainder(out, n, d, count) } }
            unsafe fn accelerate_nextafter(out: *mut Self, i: *const Self, d: *const Self, count: *const i32) 
            { unsafe { $nextafter(out, i, d, count) } }
            unsafe fn accelerate_atan2(out: *mut Self, y: *const Self, x: *const Self, count: *const i32) 
            { unsafe { $atan2(out, y, x, count) } }

           unsafe fn accelerate_ceil(out: *mut Self, i: *const Self, count: *const i32) 
            { unsafe { $ceil(out, i, count) } }
            unsafe fn accelerate_floor(out: *mut Self, i: *const Self, count: *const i32) 
            { unsafe { $floor(out, i, count) } }
            unsafe fn accelerate_fabs(out: *mut Self, i: *const Self, count: *const i32) 
            { unsafe { $fabs(out, i, count) } }
            unsafe fn accelerate_int(out: *mut Self, i: *const Self, count: *const i32) 
            { unsafe { $int(out, i, count) } }
            unsafe fn accelerate_nint(out: *mut Self, i: *const Self, count: *const i32) 
            { unsafe { $nint(out, i, count) } }
            unsafe fn accelerate_rsqrt(out: *mut Self, i: *const Self, count: *const i32) 
            { unsafe { $rsqrt(out, i, count) } }
            unsafe fn accelerate_sqrt(out: *mut Self, i: *const Self, count: *const i32) 
            { unsafe { $sqrt(out, i, count) } }
            unsafe fn accelerate_rec(out: *mut Self, i: *const Self, count: *const i32) 
            { unsafe { $rec(out, i, count) } }
            unsafe fn accelerate_exp(out: *mut Self, i: *const Self, count: *const i32) 
            { unsafe { $exp(out, i, count) } }
            unsafe fn accelerate_exp2(out: *mut Self, i: *const Self, count: *const i32) 
            { unsafe { $exp2(out, i, count) } }
            unsafe fn accelerate_expm1(out: *mut Self, i: *const Self, count: *const i32) 
            { unsafe { $expm1(out, i, count) } }
            unsafe fn accelerate_log(out: *mut Self, i: *const Self, count: *const i32) 
            { unsafe { $log(out, i, count) } }
            unsafe fn accelerate_log1p(out: *mut Self, i: *const Self, count: *const i32) 
            { unsafe { $log1p(out, i, count) } }
            unsafe fn accelerate_log2(out: *mut Self, i: *const Self, count: *const i32) 
            { unsafe { $log2(out, i, count) } }
            unsafe fn accelerate_log10(out: *mut Self, i: *const Self, count: *const i32) 
            { unsafe { $log10(out, i, count) } }
            unsafe fn accelerate_logb(out: *mut Self, i: *const Self, count: *const i32) 
            { unsafe { $logb(out, i, count) } }
            unsafe fn accelerate_sin(out: *mut Self, i: *const Self, count: *const i32) 
            { unsafe { $sin(out, i, count) } }
            unsafe fn accelerate_sinpi(out: *mut Self, i: *const Self, count: *const i32) 
            { unsafe { $sinpi(out, i, count) } }
            unsafe fn accelerate_cos(out: *mut Self, i: *const Self, count: *const i32) 
            { unsafe { $cos(out, i, count) } }
            unsafe fn accelerate_cospi(out: *mut Self, i: *const Self, count: *const i32) 
            { unsafe { $cospi(out, i, count) } }
            unsafe fn accelerate_tan(out: *mut Self, i: *const Self, count: *const i32) 
            { unsafe { $tan(out, i, count) } }
            unsafe fn accelerate_tanpi(out: *mut Self, i: *const Self, count: *const i32) 
            { unsafe { $tanpi(out, i, count) } }
            unsafe fn accelerate_asin(out: *mut Self, i: *const Self, count: *const i32) 
            { unsafe { $asin(out, i, count) } }
            unsafe fn accelerate_acos(out: *mut Self, i: *const Self, count: *const i32) 
            { unsafe { $acos(out, i, count) } }
            unsafe fn accelerate_atan(out: *mut Self, i: *const Self, count: *const i32) 
            { unsafe { $atan(out, i, count) } }
            unsafe fn accelerate_sinh(out: *mut Self, i: *const Self, count: *const i32) 
            { unsafe { $sinh(out, i, count) } }
            unsafe fn accelerate_cosh(out: *mut Self, i: *const Self, count: *const i32) 
            { unsafe { $cosh(out, i, count) } }
            unsafe fn accelerate_tanh(out: *mut Self, i: *const Self, count: *const i32) 
            { unsafe { $tanh(out, i, count) } }
            unsafe fn accelerate_asinh(out: *mut Self, i: *const Self, count: *const i32) 
            { unsafe { $asinh(out, i, count) } }
            unsafe fn accelerate_acosh(out: *mut Self, i: *const Self, count: *const i32) 
            { unsafe { $acosh(out, i, count) } }
            unsafe fn accelerate_atanh(out: *mut Self, i: *const Self, count: *const i32) 
            { unsafe { $atanh(out, i, count) } }
            unsafe fn accelerate_sincos(s: *mut Self, c: *mut Self, i: *const Self, count: *const i32) 
            { unsafe { $sincos(s, c, i, count) } }
            unsafe fn accelerate_cosisin(out: *mut AccelerateComplex<Self>, i: *const Self, count: *const i32) 
            { unsafe { $cosisin(out, i, count) } }
        }
    };
}

impl_accelerate_float!(f64,
    vvpow, vvdiv, vvcopysign, vvfmod, vvremainder, vvnextafter, vvatan2,
    vvceil, vvfloor, vvfabs, vvint, vvnint, vvrsqrt, vvsqrt, vvrec,
    vvexp, vvexp2, vvexpm1, vvlog, vvlog1p, vvlog2, vvlog10, vvlogb,
    vvsin, vvsinpi, vvcos, vvcospi, vvtan, vvtanpi, vvasin, vvacos, vvatan,
    vvsinh, vvcosh, vvtanh, vvasinh, vvacosh, vvatanh, vvsincos, vvcosisin
);

impl_accelerate_float!(f32,
    vvpowf, vvdivf, vvcopysignf, vvfmodf, vvremainderf, vvnextafterf, vvatan2f,
    vvceilf, vvfloorf, vvfabsf, vvintf, vvnintf, vvrsqrtf, vvsqrtf, vvrecf,
    vvexpf, vvexp2f, vvexpm1f, vvlogf, vvlog1pf, vvlog2f, vvlog10f, vvlogbf,
    vvsinf, vvsinpif, vvcosf, vvcospif, vvtanf, vvtanpif, vvasinf, vvacosf, vvatanf,
    vvsinhf, vvcoshf, vvtanhf, vvasinhf, vvacoshf, vvatanhf, vvsincosf, vvcosisinf
);

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


// ── Power functions ──

binary_vforce_op!(
    /// Computes bases ^ exponents, writing the results to `out`.
    pow_array,
    /// Computes bases ^ exponents, overwriting `bases` with the results.
    pow_array_in_place,
    accelerate_pow, bases, exponents);

// ── Arithmetic and Auxiliary Functions ──

binary_vforce_op!(
    /// Computes numerator / denominator, writing the results to `out`.
    div_array,
    /// Computes numerator / denominator, overwriting `numerator` with the results.
    div_array_in_place,
    accelerate_div, numerator, denominator);

binary_vforce_op!(
    /// Copies the sign of each element in `sign` to the corresponding element in `magnitude`,
    /// writing the results to `out`.
    copysign_array,
    /// Copies the sign of each element in `sign` to the corresponding element in `magnitude`,
    /// overwriting `magnitude` with the results.
    copysign_array_in_place,
    accelerate_copysign, magnitude, sign);

binary_vforce_op!(
    /// Computes the floating-point remainder of numerator / denominator (C fmod),
    /// writing the results to `out`.
    fmod_array,
    /// Computes the floating-point remainder of numerator / denominator (C fmod),
    /// overwriting `numerator` with the results.
    fmod_array_in_place,
    accelerate_fmod, numerator, denominator);

binary_vforce_op!(
    /// Computes the IEEE remainder of numerator / denominator,
    /// writing the results to `out`.
    remainder_array,
    /// Computes the IEEE remainder of numerator / denominator,
    /// overwriting `numerator` with the results.
    remainder_array_in_place,
    accelerate_remainder, numerator, denominator);

binary_vforce_op!(
    /// Computes the next representable floating-point value after each element in `input`
    /// in the direction of the corresponding element in `direction`, writing the results to `out`.
    nextafter_array,
    /// Computes the next representable floating-point value after each element in `input`
    /// in the direction of the corresponding element in `direction`,
    /// overwriting `input` with the results.
    nextafter_array_in_place,
    accelerate_nextafter, input, direction);

unary_vforce_op!(
    /// Computes the ceiling of each element, writing the results to `out`.
    ceil_array,
    /// Computes the ceiling of each element, overwriting `input` with the results.
    ceil_array_in_place,
    accelerate_ceil, input);

unary_vforce_op!(
    /// Computes the floor of each element, writing the results to `out`.
    floor_array,
    /// Computes the floor of each element, overwriting `input` with the results.
    floor_array_in_place,
    accelerate_floor, input);

unary_vforce_op!(
    /// Computes the absolute value of each element, writing the results to `out`.
    fabs_array,
    /// Computes the absolute value of each element, overwriting `input` with the results.
    fabs_array_in_place,
    accelerate_fabs, input);

unary_vforce_op!(
    /// Truncates each element to an integer (rounds toward zero), writing the results to `out`.
    int_array,
    /// Truncates each element to an integer (rounds toward zero), overwriting `input` with the results.
    int_array_in_place,
    accelerate_int, input);

unary_vforce_op!(
    /// Rounds each element to the nearest integer, writing the results to `out`.
    nint_array,
    /// Rounds each element to the nearest integer, overwriting `input` with the results.
    nint_array_in_place,
    accelerate_nint, input);

unary_vforce_op!(
    /// Computes 1/sqrt(x) for each element, writing the results to `out`.
    rsqrt_array,
    /// Computes 1/sqrt(x) for each element, overwriting `input` with the results.
    rsqrt_array_in_place,
    accelerate_rsqrt, input);

unary_vforce_op!(
    /// Computes the square root of each element, writing the results to `out`.
    sqrt_array,
    /// Computes the square root of each element, overwriting `input` with the results.
    sqrt_array_in_place,
    accelerate_sqrt, input);

unary_vforce_op!(
    /// Computes 1/x for each element, writing the results to `out`.
    rec_array,
    /// Computes 1/x for each element, overwriting `input` with the results.
    rec_array_in_place,
    accelerate_rec, input);

// ── Exponential and Logarithmic Functions ──

unary_vforce_op!(
    /// Computes e^x for each element, writing the results to `out`.
    exp_array,
    /// Computes e^x for each element, overwriting `input` with the results.
    exp_array_in_place,
    accelerate_exp, input);

unary_vforce_op!(
    /// Computes 2^x for each element, writing the results to `out`.
    exp2_array,
    /// Computes 2^x for each element, overwriting `input` with the results.
    exp2_array_in_place,
    accelerate_exp2, input);

unary_vforce_op!(
    /// Computes e^x - 1 for each element, writing the results to `out`.
    expm1_array,
    /// Computes e^x - 1 for each element, overwriting `input` with the results.
    expm1_array_in_place,
    accelerate_expm1, input);

unary_vforce_op!(
    /// Computes the natural logarithm of each element, writing the results to `out`.
    log_array,
    /// Computes the natural logarithm of each element, overwriting `input` with the results.
    log_array_in_place,
    accelerate_log, input);

unary_vforce_op!(
    /// Computes ln(1 + x) for each element, writing the results to `out`.
    log1p_array,
    /// Computes ln(1 + x) for each element, overwriting `input` with the results.
    log1p_array_in_place,
    accelerate_log1p, input);

unary_vforce_op!(
    /// Computes the base-2 logarithm of each element, writing the results to `out`.
    log2_array,
    /// Computes the base-2 logarithm of each element, overwriting `input` with the results.
    log2_array_in_place,
    accelerate_log2, input);

unary_vforce_op!(
    /// Computes the base-10 logarithm of each element, writing the results to `out`.
    log10_array,
    /// Computes the base-10 logarithm of each element, overwriting `input` with the results.
    log10_array_in_place,
    accelerate_log10, input);

unary_vforce_op!(
    /// Extracts the exponent of each element as a signed integral value, writing the results to `out`.
    logb_array,
    /// Extracts the exponent of each element as a signed integral value, overwriting `input` with the results.
    logb_array_in_place,
    accelerate_logb, input);

// ── Trigonometric Functions ──

unary_vforce_op!(
    /// Computes the sine of each element (in radians), writing the results to `out`.
    ///
    /// If x is +/-0, the result preserves the signed zero.
    /// If x is +/-inf, the result is NaN.
    sin_array,
    /// Computes the sine of each element (in radians), overwriting `input` with the results.
    ///
    /// If x is +/-0, the result preserves the signed zero.
    /// If x is +/-inf, the result is NaN.
    sin_array_in_place,
    accelerate_sin, input);

unary_vforce_op!(
    /// Computes sin(x * pi) for each element, writing the results to `out`.
    sinpi_array,
    /// Computes sin(x * pi) for each element, overwriting `input` with the results.
    sinpi_array_in_place,
    accelerate_sinpi, input);

unary_vforce_op!(
    /// Computes the cosine of each element (in radians), writing the results to `out`.
    ///
    /// If x is +/-inf, the result is NaN.
    cos_array,
    /// Computes the cosine of each element (in radians), overwriting `input` with the results.
    ///
    /// If x is +/-inf, the result is NaN.
    cos_array_in_place,
    accelerate_cos, input);

unary_vforce_op!(
    /// Computes cos(x * pi) for each element, writing the results to `out`.
    cospi_array,
    /// Computes cos(x * pi) for each element, overwriting `input` with the results.
    cospi_array_in_place,
    accelerate_cospi, input);

unary_vforce_op!(
    /// Computes the tangent of each element (in radians), writing the results to `out`.
    ///
    /// If x is +/-0, the result preserves the signed zero.
    /// If x is +/-inf, the result is NaN.
    tan_array,
    /// Computes the tangent of each element (in radians), overwriting `input` with the results.
    ///
    /// If x is +/-0, the result preserves the signed zero.
    /// If x is +/-inf, the result is NaN.
    tan_array_in_place,
    accelerate_tan, input);

unary_vforce_op!(
    /// Computes tan(x * pi) for each element, writing the results to `out`.
    tanpi_array,
    /// Computes tan(x * pi) for each element, overwriting `input` with the results.
    tanpi_array_in_place,
    accelerate_tanpi, input);

unary_vforce_op!(
    /// Computes the arcsine of each element, writing the results to `out`.
    ///
    /// If x is +/-0, the result preserves the signed zero.
    /// If |x| > 1, the result is NaN.
    asin_array,
    /// Computes the arcsine of each element, overwriting `input` with the results.
    ///
    /// If x is +/-0, the result preserves the signed zero.
    /// If |x| > 1, the result is NaN.
    asin_array_in_place,
    accelerate_asin, input);

unary_vforce_op!(
    /// Computes the arccosine of each element, writing the results to `out`.
    ///
    /// If x is 1, the result is +0.
    /// If |x| > 1, the result is NaN.
    acos_array,
    /// Computes the arccosine of each element, overwriting `input` with the results.
    ///
    /// If x is 1, the result is +0.
    /// If |x| > 1, the result is NaN.
    acos_array_in_place,
    accelerate_acos, input);

unary_vforce_op!(
    /// Computes the arctangent of each element, writing the results to `out`.
    ///
    /// If x is +/-0, the result preserves the signed zero.
    /// If x is +/-inf, the result is +/-pi/2.
    atan_array,
    /// Computes the arctangent of each element, overwriting `input` with the results.
    ///
    /// If x is +/-0, the result preserves the signed zero.
    /// If x is +/-inf, the result is +/-pi/2.
    atan_array_in_place,
    accelerate_atan, input);

binary_vforce_op!(
    /// Computes the two-argument arctangent atan2(y, x), writing the results to `out`.
    ///
    /// The signs of both arguments are used to determine the quadrant of the calculated value.
    /// The following special values of x and y produce the given value of z:
    ///
    /// y      |   x  |   result
    /// +/-0   |  -0  |   +/-pi
    /// +/-0   |  +0  |   +/-0
    /// +/-0   |  <0  |   +/-pi
    /// +/-0   |  >0  |   +/-0
    ///  >0    | +/-0 |   +pi/2
    ///  <0    | +/-0 |   -pi/2
    /// +/-y   | -inf |   +/-pi
    /// +/-y   | +inf |   +/-0
    /// +/-inf |   x  |   +/-pi/2
    /// +/-inf | -inf |   +/-3pi/4
    /// +/-inf | +inf |   +/-pi/4
    atan2_array,
    /// Computes the two-argument arctangent atan2(y, x), overwriting `y` with the results.
    ///
    /// The signs of both arguments are used to determine the quadrant of the calculated value.
    /// The following special values of x and y produce the given value of z:
    ///
    /// y      |   x  |   result
    /// +/-0   |  -0  |   +/-pi
    /// +/-0   |  +0  |   +/-0
    /// +/-0   |  <0  |   +/-pi
    /// +/-0   |  >0  |   +/-0
    ///  >0    | +/-0 |   +pi/2
    ///  <0    | +/-0 |   -pi/2
    /// +/-y   | -inf |   +/-pi
    /// +/-y   | +inf |   +/-0
    /// +/-inf |   x  |   +/-pi/2
    /// +/-inf | -inf |   +/-3pi/4
    /// +/-inf | +inf |   +/-pi/4
    atan2_array_in_place,
    accelerate_atan2, y, x);

// ── Hyperbolic Functions ──

unary_vforce_op!(
    /// Computes the hyperbolic sine of each element, writing the results to `out`.
    ///
    /// If x is +/-0, the result preserves the signed zero.
    /// If x is +/-inf, the result is +/-inf.
    sinh_array,
    /// Computes the hyperbolic sine of each element, overwriting `input` with the results.
    ///
    /// If x is +/-0, the result preserves the signed zero.
    /// If x is +/-inf, the result is +/-inf.
    sinh_array_in_place,
    accelerate_sinh, input);

unary_vforce_op!(
    /// Computes the hyperbolic cosine of each element, writing the results to `out`.
    ///
    /// If x is +/-0, the result is 1.
    /// If x is +/-inf, the result is +inf.
    cosh_array,
    /// Computes the hyperbolic cosine of each element, overwriting `input` with the results.
    ///
    /// If x is +/-0, the result is 1.
    /// If x is +/-inf, the result is +inf.
    cosh_array_in_place,
    accelerate_cosh, input);

unary_vforce_op!(
    /// Computes the hyperbolic tangent of each element, writing the results to `out`.
    ///
    /// If x is +/-0, the result preserves the signed zero.
    /// If x is +/-inf, the result is +/-1.
    tanh_array,
    /// Computes the hyperbolic tangent of each element, overwriting `input` with the results.
    ///
    /// If x is +/-0, the result preserves the signed zero.
    /// If x is +/-inf, the result is +/-1.
    tanh_array_in_place,
    accelerate_tanh, input);

unary_vforce_op!(
    /// Computes the inverse hyperbolic sine of each element, writing the results to `out`.
    asinh_array,
    /// Computes the inverse hyperbolic sine of each element, overwriting `input` with the results.
    asinh_array_in_place,
    accelerate_asinh, input);

unary_vforce_op!(
    /// Computes the inverse hyperbolic cosine of each element, writing the results to `out`.
    ///
    /// The calculated values are in the range [0, +inf].
    /// If x == 1, the result is +0.
    /// If x < 1, the result is NaN.
    /// If x == +inf, the result is +inf.
    acosh_array,
    /// Computes the inverse hyperbolic cosine of each element, overwriting `input` with the results.
    ///
    /// The calculated values are in the range [0, +inf].
    /// If x == 1, the result is +0.
    /// If x < 1, the result is NaN.
    /// If x == +inf, the result is +inf.
    acosh_array_in_place,
    accelerate_acosh, input);

unary_vforce_op!(
    /// Computes the inverse hyperbolic tangent of each element, writing the results to `out`.
    ///
    /// If x is +/-0, the result preserves the signed zero.
    /// If x is +/-1, the result is +/-inf.
    /// If |x|>1, the result is NaN.
    atanh_array,
    /// Computes the inverse hyperbolic tangent of each element, overwriting `input` with the results.
    ///
    /// If x is +/-0, the result preserves the signed zero.
    /// If x is +/-1, the result is +/-inf.
    /// If |x|>1, the result is NaN.
    atanh_array_in_place,
    accelerate_atanh, input);

// ── Special: sincos and vvcosisin ──

/// Computes the sine and cosine of each element simultaneously, writing the results
/// to `sin_out` and `cos_out` respectively.
pub fn sincos_array<AF: AccelerateFloat>(
    sin_out: &mut [AF], cos_out: &mut [AF], input: &[AF]
) -> Result<(), AccelerateError> {
    let count = validate_lengths_2(input.len(), sin_out.len(), cos_out.len())?;
    unsafe { AF::accelerate_sincos(sin_out.as_mut_ptr(), cos_out.as_mut_ptr(), input.as_ptr(), &count); }
    Ok(())
}
/// Computes the sine and cosine of each element simultaneously, writing the `sin` results into
/// `input` and the `cos` results into `cos_out`
pub fn sincos_array_in_place_sin<AF: AccelerateFloat>(
    cos_out: &mut [AF], input: &mut [AF]
) -> Result<(), AccelerateError> {
    let count = validate_lengths_1(input.len(), cos_out.len())?;
    unsafe { AF::accelerate_sincos(input.as_mut_ptr(), cos_out.as_mut_ptr(), input.as_ptr(), &count); }
    Ok(())
}

/// Computes the sine and cosine of each element simultaneously, writing the `cos` results into
/// `input` and the `sin` results into `sin_out`
pub fn sincos_array_in_place_cos<AF: AccelerateFloat>(
    sin_out: &mut [AF], input: &mut [AF]
) -> Result<(), AccelerateError> {
    let count = validate_lengths_1(input.len(), sin_out.len())?;
    unsafe { AF::accelerate_sincos(sin_out.as_mut_ptr(), input.as_mut_ptr(), input.as_ptr(), &count); }
    Ok(())
}

/// Computes the complex number on the unit circle corresponding to the angle given by each element of a vector.
///
/// Does not have an in-place variant, as the output array is necessarily twice the size of the input array.
pub fn cosisin_array<AF: AccelerateFloat>(
    out: &mut [AccelerateComplex<AF>], input: &[AF]
) -> Result<(), AccelerateError> {
    let count = validate_lengths_1(out.len(), input.len())?;
    unsafe { AF::accelerate_cosisin(out.as_mut_ptr(), input.as_ptr(), &count); }
    Ok(())
}

#[cfg(test)]
extern crate alloc;

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;
    use alloc::vec::Vec;

    const INPUTS: [f64; 4] = [0.5, 1.0, 2.0, 3.5];
    const POSITIVE: [f64; 4] = [0.25, 0.5, 1.0, 4.0];
    const UNIT: [f64; 3] = [0.0, 0.25, -0.5];
    const SMALL: [f64; 3] = [0.1, -0.3, 0.9];

    fn assert_approx(actual: &[f64], expected: &[f64], tol: f64, name: &str) {
        assert_eq!(actual.len(), expected.len(), "{name}: length mismatch");
        for (i, (&a, &e)) in actual.iter().zip(expected.iter()).enumerate() {
            assert!(
                (a - e).abs() < tol,
                "{name}[{i}]: got {a}, expected {e}, diff {}",
                (a - e).abs()
            );
        }
    }

    fn assert_approx_f32(actual: &[f32], expected: &[f32], tol: f32, name: &str) {
        assert_eq!(actual.len(), expected.len(), "{name}: length mismatch");
        for (i, (&a, &e)) in actual.iter().zip(expected.iter()).enumerate() {
            assert!(
                (a - e).abs() < tol,
                "{name}[{i}]: got {a}, expected {e}, diff {}",
                (a - e).abs()
            );
        }
    }

    // Helper to test a unary out-of-place function against a scalar reference
    fn check_unary(
        vforce_fn: fn(&mut [f64], &[f64]) -> Result<(), AccelerateError>,
        scalar_fn: fn(f64) -> f64,
        inputs: &[f64],
        name: &str,
    ) {
        let mut out = vec![0.0f64; inputs.len()];
        vforce_fn(&mut out, inputs).unwrap();
        let expected: Vec<f64> = inputs.iter().map(|&x| scalar_fn(x)).collect();
        assert_approx(&out, &expected, 1e-10, name);
    }

    // Helper to test a unary in-place function against a scalar reference
    fn check_unary_in_place(
        vforce_fn: fn(&mut [f64]) -> Result<(), AccelerateError>,
        scalar_fn: fn(f64) -> f64,
        inputs: &[f64],
        name: &str,
    ) {
        let mut buf: Vec<f64> = inputs.to_vec();
        vforce_fn(&mut buf).unwrap();
        let expected: Vec<f64> = inputs.iter().map(|&x| scalar_fn(x)).collect();
        assert_approx(&buf, &expected, 1e-10, name);
    }

    // Helper to test a binary out-of-place function against a scalar reference
    #[allow(clippy::type_complexity)]
    fn check_binary(
        vforce_fn: fn(&mut [f64], &[f64], &[f64]) -> Result<(), AccelerateError>,
        scalar_fn: fn(f64, f64) -> f64,
        a: &[f64],
        b: &[f64],
        name: &str,
    ) {
        let mut out = vec![0.0f64; a.len()];
        vforce_fn(&mut out, a, b).unwrap();
        let expected: Vec<f64> = a.iter().zip(b.iter()).map(|(&x, &y)| scalar_fn(x, y)).collect();
        assert_approx(&out, &expected, 1e-10, name);
    }

    // Helper to test a binary in-place function against a scalar reference
    fn check_binary_in_place(
        vforce_fn: fn(&mut [f64], &[f64]) -> Result<(), AccelerateError>,
        scalar_fn: fn(f64, f64) -> f64,
        a: &[f64],
        b: &[f64],
        name: &str,
    ) {
        let mut buf: Vec<f64> = a.to_vec();
        vforce_fn(&mut buf, b).unwrap();
        let expected: Vec<f64> = a.iter().zip(b.iter()).map(|(&x, &y)| scalar_fn(x, y)).collect();
        assert_approx(&buf, &expected, 1e-10, name);
    }

    // ── Power functions ──

    #[test]
    fn test_pow_array() {
        let bases = [2.0, 3.0, 4.0, 5.0];
        let exponents = [3.0, 2.0, 0.5, 1.0];
        check_binary(pow_array, f64::powf, &bases, &exponents, "pow_array");
        check_binary_in_place(pow_array_in_place, f64::powf, &bases, &exponents, "pow_array_in_place");
    }

    // ── Arithmetic and Auxiliary functions ──

    #[test]
    fn test_div_array() {
        let num = [10.0, 9.0, 8.0, 7.0];
        let den = [2.0, 3.0, 4.0, 0.5];
        check_binary(div_array, |a, b| a / b, &num, &den, "div_array");
        check_binary_in_place(div_array_in_place, |a, b| a / b, &num, &den, "div_array_in_place");
    }

    #[test]
    fn test_copysign_array() {
        let mag = [1.0, -2.0, 3.0, -4.0];
        let sign = [-1.0, 1.0, -1.0, 1.0];
        check_binary(copysign_array, f64::copysign, &mag, &sign, "copysign_array");
        check_binary_in_place(copysign_array_in_place, f64::copysign, &mag, &sign, "copysign_array_in_place");
    }

    #[test]
    fn test_fmod_array() {
        let num = [5.5, 7.0, -3.5, 10.0];
        let den = [2.0, 3.0, 1.5, 3.0];
        check_binary(fmod_array, |a, b| a % b, &num, &den, "fmod_array");
        check_binary_in_place(fmod_array_in_place, |a, b| a % b, &num, &den, "fmod_array_in_place");
    }

    #[test]
    #[ignore = "stub: no core equivalent for IEEE remainder"]
    fn test_remainder_array() {
        todo!("IEEE remainder differs from fmod; needs manual verification");
    }

    #[test]
    #[ignore = "stub: no stable core equivalent for nextafter"]
    fn test_nextafter_array() {
        todo!("f64::next_up/next_down are not the same as nextafter(x, direction)");
    }

    #[test]
    fn test_ceil_array() {
        let inputs = [-1.5, 0.0, 0.3, 2.7];
        check_unary(ceil_array, f64::ceil, &inputs, "ceil_array");
        check_unary_in_place(ceil_array_in_place, f64::ceil, &inputs, "ceil_array_in_place");
    }

    #[test]
    fn test_floor_array() {
        let inputs = [-1.5, 0.0, 0.3, 2.7];
        check_unary(floor_array, f64::floor, &inputs, "floor_array");
        check_unary_in_place(floor_array_in_place, f64::floor, &inputs, "floor_array_in_place");
    }

    #[test]
    fn test_fabs_array() {
        let inputs = [-3.5, 0.0, 2.5, -0.1];
        check_unary(fabs_array, f64::abs, &inputs, "fabs_array");
        check_unary_in_place(fabs_array_in_place, f64::abs, &inputs, "fabs_array_in_place");
    }

    #[test]
    fn test_int_array() {
        let inputs = [-1.7, 0.0, 0.9, 2.3];
        check_unary(int_array, f64::trunc, &inputs, "int_array");
        check_unary_in_place(int_array_in_place, f64::trunc, &inputs, "int_array_in_place");
    }

    #[test]
    #[ignore = "stub: vvnint rounding mode for ties may differ from f64::round"]
    fn test_nint_array() {
        todo!("vvnint may round ties to even vs f64::round which rounds ties away from zero");
    }

    #[test]
    fn test_rsqrt_array() {
        check_unary(rsqrt_array, |x| 1.0 / x.sqrt(), &POSITIVE, "rsqrt_array");
        check_unary_in_place(rsqrt_array_in_place, |x| 1.0 / x.sqrt(), &POSITIVE, "rsqrt_array_in_place");
    }

    #[test]
    fn test_sqrt_array() {
        check_unary(sqrt_array, f64::sqrt, &POSITIVE, "sqrt_array");
        check_unary_in_place(sqrt_array_in_place, f64::sqrt, &POSITIVE, "sqrt_array_in_place");
    }

    #[test]
    fn test_rec_array() {
        check_unary(rec_array, |x| 1.0 / x, &INPUTS, "rec_array");
        check_unary_in_place(rec_array_in_place, |x| 1.0 / x, &INPUTS, "rec_array_in_place");
    }

    // ── Exponential and Logarithmic functions ──

    #[test]
    fn test_exp_array() {
        check_unary(exp_array, f64::exp, &INPUTS, "exp_array");
        check_unary_in_place(exp_array_in_place, f64::exp, &INPUTS, "exp_array_in_place");
    }

    #[test]
    fn test_exp2_array() {
        check_unary(exp2_array, f64::exp2, &INPUTS, "exp2_array");
        check_unary_in_place(exp2_array_in_place, f64::exp2, &INPUTS, "exp2_array_in_place");
    }

    #[test]
    fn test_expm1_array() {
        check_unary(expm1_array, |x| x.exp_m1(), &SMALL, "expm1_array");
        check_unary_in_place(expm1_array_in_place, |x| x.exp_m1(), &SMALL, "expm1_array_in_place");
    }

    #[test]
    fn test_log_array() {
        check_unary(log_array, f64::ln, &POSITIVE, "log_array");
        check_unary_in_place(log_array_in_place, f64::ln, &POSITIVE, "log_array_in_place");
    }

    #[test]
    fn test_log1p_array() {
        check_unary(log1p_array, |x| x.ln_1p(), &SMALL, "log1p_array");
        check_unary_in_place(log1p_array_in_place, |x| x.ln_1p(), &SMALL, "log1p_array_in_place");
    }

    #[test]
    fn test_log2_array() {
        check_unary(log2_array, f64::log2, &POSITIVE, "log2_array");
        check_unary_in_place(log2_array_in_place, f64::log2, &POSITIVE, "log2_array_in_place");
    }

    #[test]
    fn test_log10_array() {
        check_unary(log10_array, f64::log10, &POSITIVE, "log10_array");
        check_unary_in_place(log10_array_in_place, f64::log10, &POSITIVE, "log10_array_in_place");
    }

    #[test]
    #[ignore = "stub: no core equivalent for logb (exponent extraction)"]
    fn test_logb_array() {
        todo!("logb extracts the exponent as a float; no direct core equivalent");
    }

    // ── Trigonometric functions ──

    #[test]
    fn test_sin_array() {
        check_unary(sin_array, f64::sin, &INPUTS, "sin_array");
        check_unary_in_place(sin_array_in_place, f64::sin, &INPUTS, "sin_array_in_place");
    }

    #[test]
    #[ignore = "stub: no core equivalent for sinpi"]
    fn test_sinpi_array() {
        todo!("sinpi computes sin(x * pi); no direct core equivalent");
    }

    #[test]
    fn test_cos_array() {
        check_unary(cos_array, f64::cos, &INPUTS, "cos_array");
        check_unary_in_place(cos_array_in_place, f64::cos, &INPUTS, "cos_array_in_place");
    }

    #[test]
    #[ignore = "stub: no core equivalent for cospi"]
    fn test_cospi_array() {
        todo!("cospi computes cos(x * pi); no direct core equivalent");
    }

    #[test]
    fn test_tan_array() {
        check_unary(tan_array, f64::tan, &INPUTS, "tan_array");
        check_unary_in_place(tan_array_in_place, f64::tan, &INPUTS, "tan_array_in_place");
    }

    #[test]
    #[ignore = "stub: no core equivalent for tanpi"]
    fn test_tanpi_array() {
        todo!("tanpi computes tan(x * pi); no direct core equivalent");
    }

    #[test]
    fn test_asin_array() {
        check_unary(asin_array, f64::asin, &UNIT, "asin_array");
        check_unary_in_place(asin_array_in_place, f64::asin, &UNIT, "asin_array_in_place");
    }

    #[test]
    fn test_acos_array() {
        check_unary(acos_array, f64::acos, &UNIT, "acos_array");
        check_unary_in_place(acos_array_in_place, f64::acos, &UNIT, "acos_array_in_place");
    }

    #[test]
    fn test_atan_array() {
        check_unary(atan_array, f64::atan, &INPUTS, "atan_array");
        check_unary_in_place(atan_array_in_place, f64::atan, &INPUTS, "atan_array_in_place");
    }

    #[test]
    fn test_atan2_array() {
        let y = [1.0, -1.0, 3.0, 0.0];
        let x = [1.0, 1.0, -2.0, 5.0];
        check_binary(atan2_array, f64::atan2, &y, &x, "atan2_array");
        check_binary_in_place(atan2_array_in_place, f64::atan2, &y, &x, "atan2_array_in_place");
    }

    // ── Hyperbolic functions ──

    #[test]
    fn test_sinh_array() {
        check_unary(sinh_array, f64::sinh, &INPUTS, "sinh_array");
        check_unary_in_place(sinh_array_in_place, f64::sinh, &INPUTS, "sinh_array_in_place");
    }

    #[test]
    fn test_cosh_array() {
        let inputs = [0.0, 0.5, 1.0, 2.0];
        check_unary(cosh_array, f64::cosh, &inputs, "cosh_array");
        check_unary_in_place(cosh_array_in_place, f64::cosh, &inputs, "cosh_array_in_place");
    }

    #[test]
    fn test_tanh_array() {
        check_unary(tanh_array, f64::tanh, &INPUTS, "tanh_array");
        check_unary_in_place(tanh_array_in_place, f64::tanh, &INPUTS, "tanh_array_in_place");
    }

    #[test]
    fn test_asinh_array() {
        check_unary(asinh_array, f64::asinh, &INPUTS, "asinh_array");
        check_unary_in_place(asinh_array_in_place, f64::asinh, &INPUTS, "asinh_array_in_place");
    }

    #[test]
    fn test_acosh_array() {
        let inputs = [1.0, 1.5, 2.0, 4.0];
        check_unary(acosh_array, f64::acosh, &inputs, "acosh_array");
        check_unary_in_place(acosh_array_in_place, f64::acosh, &inputs, "acosh_array_in_place");
    }

    #[test]
    fn test_atanh_array() {
        check_unary(atanh_array, f64::atanh, &UNIT, "atanh_array");
        check_unary_in_place(atanh_array_in_place, f64::atanh, &UNIT, "atanh_array_in_place");
    }

    // ── Special: sincos ──

    #[test]
    fn test_sincos_array() {
        let mut sin_out = [0.0f64; 4];
        let mut cos_out = [0.0f64; 4];
        sincos_array(&mut sin_out, &mut cos_out, &INPUTS).unwrap();
        let expected_sin: Vec<f64> = INPUTS.iter().map(|&x| x.sin()).collect();
        let expected_cos: Vec<f64> = INPUTS.iter().map(|&x| x.cos()).collect();
        assert_approx(&sin_out, &expected_sin, 1e-10, "sincos_array (sin)");
        assert_approx(&cos_out, &expected_cos, 1e-10, "sincos_array (cos)");
    }

    // ── f32 spot check ──

    #[test]
    fn test_f32_sin_array() {
        let inputs: [f32; 4] = [0.5, 1.0, 2.0, 3.5];
        let mut out = [0.0f32; 4];
        sin_array(&mut out, &inputs).unwrap();
        let expected: Vec<f32> = inputs.iter().map(|&x| x.sin()).collect();
        assert_approx_f32(&out, &expected, 1e-6, "sin_array (f32)");
    }

    #[test]
    fn test_f32_pow_array() {
        let bases: [f32; 4] = [2.0, 3.0, 4.0, 5.0];
        let exponents: [f32; 4] = [3.0, 2.0, 0.5, 1.0];
        let mut out = [0.0f32; 4];
        pow_array(&mut out, &bases, &exponents).unwrap();
        let expected: Vec<f32> = bases.iter().zip(exponents.iter()).map(|(&b, &e)| b.powf(e)).collect();
        assert_approx_f32(&out, &expected, 1e-5, "pow_array (f32)");
    }

    // ── Error handling ──

    #[test]
    fn test_length_mismatch() {
        let mut out = [0.0f64; 3];
        let input = [1.0f64; 4];
        let result = sin_array(&mut out, &input);
        assert!(matches!(result, Err(AccelerateError::LengthMismatch { .. })));
    }

    #[test]
    fn test_binary_length_mismatch() {
        let mut out = [0.0f64; 4];
        let a = [1.0f64; 4];
        let b = [2.0f64; 3];
        let result = pow_array(&mut out, &a, &b);
        assert!(matches!(result, Err(AccelerateError::LengthMismatch { .. })));
    }
}
