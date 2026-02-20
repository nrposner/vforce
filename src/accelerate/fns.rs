#![allow(clippy::doc_lazy_continuation)]
#[cfg(target_os = "macos")]
use crate::accelerate::AccelerateComplex;

#[cfg(target_os = "macos")]
unsafe extern "C" {

    // Arithmetic and Auxiliary Functions

    pub fn vvceil(out: *mut f64, input: *const f64, count: *const i32);

    pub fn vvceilf(out: *mut f32, input: *const f32, count: *const i32);

    pub fn vvfloor(out: *mut f64, input: *const f64, count: *const i32);

    pub fn vvfloorf(out: *mut f32, input: *const f32, count: *const i32);

    pub fn vvcopysign(out: *mut f64, magnitude: *const f64, sign: *const f64, count: *const i32);

    pub fn vvcopysignf(out: *mut f32, magnitude: *const f32, sign: *const f32, count: *const i32);

    /// The Accelerate framework's vvdiv function for double-precision SIMD division
    pub fn vvdiv(out: *mut f64, numerator: *const f64, denominator: *const f64, count: *const i32);

    /// The Accelerate framework's vvdivf function for single-precision SIMD division
    pub fn vvdivf(out: *mut f32, numerator: *const f32, denominator: *const f32, count: *const i32);

    /// The Accelerate framework's vvfabs function for double-precision SIMD normalization
    pub fn vvfabs(out: *mut f64, input: *const f64, count: *const i32);

    /// The Accelerate framework's vvfabs function for single-precision SIMD normalization
    pub fn vvfabsf(out: *mut f32, input: *const f32, count: *const i32);

    pub fn vvfmod(out: *mut f64, numerator: *const f64, denominator: *const f64, count: *const i32);
    pub fn vvfmodf(out: *mut f32, numerator: *const f32, denominator: *const f32, count: *const i32);

    pub fn vvremainder(out: *mut f64, numerator: *const f64, denominator: *const f64, count: *const i32);
    pub fn vvremainderf(out: *mut f32, numerator: *const f32, denominator: *const f32, count: *const i32);

    pub fn vvint(out: *mut f64, input: *const f64, count: *const i32);
    pub fn vvintf(out: *mut f32, input: *const f32, count: *const i32);

    pub fn vvnint(out: *mut f64, input: *const f64, count: *const i32);
    pub fn vvnintf(out: *mut f32, input: *const f32, count: *const i32);

    pub fn vvrsqrt(out: *mut f64, input: *const f64, count: *const i32);
    pub fn vvrsqrtf(out: *mut f32, input: *const f32, count: *const i32);

    pub fn vvsqrt(out: *mut f64, input: *const f64, count: *const i32);
    pub fn vvsqrtf(out: *mut f32, input: *const f32, count: *const i32);

    pub fn vvrec(out: *mut f64, input: *const f64, count: *const i32);
    pub fn vvrecf(out: *mut f32, input: *const f32, count: *const i32);

    pub fn vvnextafter(out: *mut f64, input: *const f64, direction: *const f64, count: *const i32);
    pub fn vvnextafterf(out: *mut f32, input: *const f32, direction: *const f32, count: *const i32);

    // Exponential and Logarithmic functions

    pub fn vvexp(out: *mut f64, input: *const f64, count: *const i32);
    pub fn vvexpf(out: *mut f32, input: *const f32, count: *const i32);

    pub fn vvexp2(out: *mut f64, input: *const f64, count: *const i32);
    pub fn vvexp2f(out: *mut f32, input: *const f32, count: *const i32);

    pub fn vvexpm1(out: *mut f64, input: *const f64, count: *const i32);
    pub fn vvexpm1f(out: *mut f32, input: *const f32, count: *const i32);

    pub fn vvlog(out: *mut f64, input: *const f64, count: *const i32);
    pub fn vvlogf(out: *mut f32, input: *const f32, count: *const i32);

    pub fn vvlog1p(out: *mut f64, input: *const f64, count: *const i32);
    pub fn vvlog1pf(out: *mut f32, input: *const f32, count: *const i32);

    pub fn vvlog2(out: *mut f64, input: *const f64, count: *const i32);
    pub fn vvlog2f(out: *mut f32, input: *const f32, count: *const i32);

    pub fn vvlog10(out: *mut f64, input: *const f64, count: *const i32);
    pub fn vvlog10f(out: *mut f32, input: *const f32, count: *const i32);

    pub fn vvlogb(out: *mut f64, input: *const f64, count: *const i32);
    pub fn vvlogbf(out: *mut f32, input: *const f32, count: *const i32);

    // Power functions

    /// The Accelerate framework's vvpow function for double-precision SIMD power
    pub fn vvpow(result: *mut f64, exponents: *const f64, bases: *const f64, count: *const i32);

    /// The Accelerate framework's vvpowf function for single-precision SIMD power
    pub fn vvpowf(result: *mut f32, exponents: *const f32, bases: *const f32, count: *const i32);

    // Trigonometric functions

    /// If x is +/-0, the result preserves the signed zero. 
    /// If x is +/-inf, the result is NaN.
    pub fn vvsin(out: *mut f64, input: *const f64, count: *const i32);

    /// If x is +/-0, the result preserves the signed zero. 
    /// If x is +/-inf, the result is NaN.
    pub fn vvsinf(out: *mut f32, input: *const f32, count: *const i32);


    pub fn vvsinpi(out: *mut f64, input: *const f64, count: *const i32);
    pub fn vvsinpif(out: *mut f32, input: *const f32, count: *const i32);


    /// If x is +/-inf, the result is NaN.
    pub fn vvcos(out: *mut f64, input: *const f64, count: *const i32);

    /// If x is +/-inf, the result is NaN.
    pub fn vvcosf(out: *mut f32, input: *const f32, count: *const i32);


    pub fn vvcospi(out: *mut f64, input: *const f64, count: *const i32);
    pub fn vvcospif(out: *mut f32, input: *const f32, count: *const i32);


    // todo: provide other complex number support? Maybe an easy way to convert our newtype into
    // another existing complex number type like the num crate's version, num::Complex<f64> etc ??
    pub fn vvcosisin(out: *mut AccelerateComplex<f64>, input: *const f64, count: *const i32);
    pub fn vvcosisinf(out: *mut AccelerateComplex<f32>, input: *const f32, count: *const i32);



    pub fn vvsincos(sin_out: *mut f64, cos_out: *mut f64, input: *const f64, count: *const i32);
    pub fn vvsincosf(sin_out: *mut f32, cos_out: *mut f32, input: *const f32, count: *const i32);


    /// If x is +/-0, the result preserves the signed zero. 
    /// If x is +/-inf, the result is NaN.
    pub fn vvtan(out: *mut f64, input: *const f64, count: *const i32);

    /// If x is +/-0, the result preserves the signed zero. 
    /// If x is +/-inf, the result is NaN.
    pub fn vvtanf(out: *mut f32, input: *const f32, count: *const i32);


    pub fn vvtanpi(out: *mut f64, input: *const f64, count: *const i32);
    pub fn vvtanpif(out: *mut f32, input: *const f32, count: *const i32);



    /// If x is +/-0, the result preserves the signed zero. 
    /// If |x| > 1, the result is NaN.
    pub fn vvasin(out: *mut f64, input: *const f64, count: *const i32);

    /// If x is +/-0, the result preserves the signed zero. 
    /// If |x| > 1, the result is NaN.
    pub fn vvasinf(out: *mut f32, input: *const f32, count: *const i32);


    /// If x is 1, the result is +0.
    /// If |x| > 1, the result is NaN.
    pub fn vvacos(out: *mut f64, input: *const f64, count: *const i32);

    /// If x is 1, the result is +0.
    /// If |x| > 1, the result is NaN.
    pub fn vvacosf(out: *mut f32, input: *const f32, count: *const i32);


    /// If x is +/-0, the result preserves the signed zero. 
    /// If x is +/-inf, the result is +/-pi/2.
    pub fn vvatan(out: *mut f64, input: *const f64, count: *const i32);

    /// If x is +/-0, the result preserves the signed zero. 
    /// If x is +/-inf, the result is +/-pi/2.
    pub fn vvatanf(out: *mut f32, input: *const f32, count: *const i32);



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
    pub fn vvatan2(out: *mut f64, y: *const f64, x: *const f64, count: *const i32);

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
    pub fn vvatan2f(out: *mut f32, y: *const f32, x: *const f32, count: *const i32);

    // Hyperbolic functions

    /// If x is +/-0, the result preserves the signed zero.
    /// If x is +/-inf, the result is +/-inf.
    pub fn vvsinh(out: *mut f64, input: *const f64, count: *const i32);

    /// If x is +/-0, the result preserves the signed zero.
    /// If x is +/-inf, the result is +/-inf.
    pub fn vvsinhf(out: *mut f32, input: *const f32, count: *const i32);

    /// If x is +/-0, the result is 1.
    /// If x is +/-inf, the result is +inf.
    pub fn vvcosh(out: *mut f64, input: *const f64, count: *const i32);

    /// If x is +/-0, the result is 1.
    /// If x is +/-inf, the result is +inf.
    pub fn vvcoshf(out: *mut f32, input: *const f32, count: *const i32);

    /// If x is +/-0, the result preserves the signed zero.
    /// If x is +/-inf, the result is +/-1.
    pub fn vvtanh(out: *mut f64, input: *const f64, count: *const i32);

    /// If x is +/-0, the result preserves the signed zero.
    /// If x is +/-inf, the result is +/-1.
    pub fn vvtanhf(out: *mut f32, input: *const f32, count: *const i32);


    pub fn vvasinh(out: *mut f64, input: *const f64, count: *const i32);
    pub fn vvasinhf(out: *mut f32, input: *const f32, count: *const i32);

    /// The calculated values are in the range [0, +inf].
    /// If x == 1, the result is +0.
    /// If x < 1, the result is NaN.
    /// If x == +inf, the result is +inf.
    pub fn vvacosh(out: *mut f64, input: *const f64, count: *const i32);
    /// The calculated values are in the range [0, +inf].
    /// If x == 1, the result is +0.
    /// If x < 1, the result is NaN.
    /// If x == +inf, the result is +inf.
    pub fn vvacoshf(out: *mut f32, input: *const f32, count: *const i32);

    /// If x is +/-0, the result preserves the signed zero.
    /// If x is +/-1, the result is +/-inf.
    /// If |x|>1, the result is NaN.
    pub fn vvatanh(out: *mut f64, input: *const f64, count: *const i32);
    /// If x is +/-0, the result preserves the signed zero.
    /// If x is +/-1, the result is +/-inf.
    /// If |x|>1, the result is NaN.
    pub fn vvatanhf(out: *mut f32, input: *const f32, count: *const i32);
}
