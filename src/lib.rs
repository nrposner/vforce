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

/// Ensures that all inputs to an accelerate function must be the same numeric type: either f64 or
/// f32
pub trait AccelerateFloat: sealed::Sealed + Copy {
    // Binary operations (out, a, b, count)
    /// # Safety
    /// All inputs must point to valid arrays of floating-point numbers. All must be of the same
    /// type, either f64 or f32, and all arrays must be of length 'count'
    unsafe fn accelerate_pow(out: *mut Self, exp: *const Self, base: *const Self, count: *const i32);
    /// # Safety
    /// All inputs must point to valid arrays of floating-point numbers. All must be of the same
    /// type, either f64 or f32, and all arrays must be of length 'count'
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

    // Special: sincos (sin_out, cos_out, input, coun
    /// # Safety
    /// All inputs must point to valid arrays of floating-point numbers. All must be of the same
    /// type, either f64 or f32, and all arrays must be of length 'count'
    unsafe fn accelerate_sincos(sin_out: *mut Self, cos_out: *mut Self, input: *const Self, count: *const i32);
}

macro_rules! impl_accelerate_float {
    ($ty:ty, $pow:ident, $div:ident, $copysign:ident, $fmod:ident, $remainder:ident,
     $nextafter:ident, $atan2:ident, $ceil:ident, $floor:ident, $fabs:ident,
     $int:ident, $nint:ident, $rsqrt:ident, $sqrt:ident, $rec:ident,
     $exp:ident, $exp2:ident, $expm1:ident, $log:ident, $log1p:ident,
     $log2:ident, $log10:ident, $logb:ident, $sin:ident, $sinpi:ident,
     $cos:ident, $cospi:ident, $tan:ident, $tanpi:ident, $asin:ident,
     $acos:ident, $atan:ident, $sinh:ident, $cosh:ident, $tanh:ident,
     $asinh:ident, $acosh:ident, $atanh:ident, $sincos:ident) => {
        impl AccelerateFloat for $ty {
            unsafe fn accelerate_pow(out: *mut Self, exp: *const Self, base: *const Self, count: *const i32) 
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
        }
    };
}

impl_accelerate_float!(f64,
    vvpow, vvdiv, vvcopysign, vvfmod, vvremainder, vvnextafter, vvatan2,
    vvceil, vvfloor, vvfabs, vvint, vvnint, vvrsqrt, vvsqrt, vvrec,
    vvexp, vvexp2, vvexpm1, vvlog, vvlog1p, vvlog2, vvlog10, vvlogb,
    vvsin, vvsinpi, vvcos, vvcospi, vvtan, vvtanpi, vvasin, vvacos, vvatan,
    vvsinh, vvcosh, vvtanh, vvasinh, vvacosh, vvatanh, vvsincos
);

impl_accelerate_float!(f32,
    vvpowf, vvdivf, vvcopysignf, vvfmodf, vvremainderf, vvnextafterf, vvatan2f,
    vvceilf, vvfloorf, vvfabsf, vvintf, vvnintf, vvrsqrtf, vvsqrtf, vvrecf,
    vvexpf, vvexp2f, vvexpm1f, vvlogf, vvlog1pf, vvlog2f, vvlog10f, vvlogbf,
    vvsinf, vvsinpif, vvcosf, vvcospif, vvtanf, vvtanpif, vvasinf, vvacosf, vvatanf,
    vvsinhf, vvcoshf, vvtanhf, vvasinhf, vvacoshf, vvatanhf, vvsincosf
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
    /// If x is +/-0, the result preserves the signed zero.
    /// If |x| > 1, the result is NaN.
    acos_array,
    /// Computes the arccosine of each element, overwriting `input` with the results.
    ///
    /// If x is +/-0, the result preserves the signed zero.
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
    /// If x is +/-0, the result preserves the signed zero.
    /// If x is +/-inf, the result is +inf.
    cosh_array,
    /// Computes the hyperbolic cosine of each element, overwriting `input` with the results.
    ///
    /// If x is +/-0, the result preserves the signed zero.
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
