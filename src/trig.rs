use super::*;
use crate::accelerate::AccelerateComplex;

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

// ── Special: sincos and cosisin ──

/// Computes the sine and cosine of each element simultaneously, writing the results
/// to `sin_out` and `cos_out` respectively.
pub fn sincos_array<AF: AccelerateFloat>(
    sin_out: &mut [AF], cos_out: &mut [AF], input: &[AF]
) -> Result<(), AccelerateError> {
    check_lengths_2(input.len(), sin_out.len(), cos_out.len())?;
    for ((sin_chunk, cos_chunk), in_chunk) in sin_out.chunks_mut(CHUNK)
        .zip(cos_out.chunks_mut(CHUNK))
        .zip(input.chunks(CHUNK))
    {
        let count = in_chunk.len() as i32;
        unsafe { AF::accelerate_sincos(sin_chunk.as_mut_ptr(), cos_chunk.as_mut_ptr(), in_chunk.as_ptr(), &count); }
    }
    Ok(())
}
/// Computes the sine and cosine of each element simultaneously, writing the `sin` results into
/// `input` and the `cos` results into `cos_out`
pub fn sincos_array_in_place_sin<AF: AccelerateFloat>(
    cos_out: &mut [AF], input: &mut [AF]
) -> Result<(), AccelerateError> {
    check_lengths_1(input.len(), cos_out.len())?;
    for (in_chunk, cos_chunk) in input.chunks_mut(CHUNK).zip(cos_out.chunks_mut(CHUNK)) {
        let count = in_chunk.len() as i32;
        unsafe { AF::accelerate_sincos(in_chunk.as_mut_ptr(), cos_chunk.as_mut_ptr(), in_chunk.as_ptr(), &count); }
    }
    Ok(())
}

/// Computes the sine and cosine of each element simultaneously, writing the `cos` results into
/// `input` and the `sin` results into `sin_out`
pub fn sincos_array_in_place_cos<AF: AccelerateFloat>(
    sin_out: &mut [AF], input: &mut [AF]
) -> Result<(), AccelerateError> {
    check_lengths_1(input.len(), sin_out.len())?;
    for (in_chunk, sin_chunk) in input.chunks_mut(CHUNK).zip(sin_out.chunks_mut(CHUNK)) {
        let count = in_chunk.len() as i32;
        unsafe { AF::accelerate_sincos(sin_chunk.as_mut_ptr(), in_chunk.as_mut_ptr(), in_chunk.as_ptr(), &count); }
    }
    Ok(())
}

/// Computes the complex number on the unit circle corresponding to the angle given by each element of a vector.
///
/// Does not have an in-place variant, as the output array is necessarily twice the size of the input array.
pub fn cosisin_array<AF: AccelerateFloat>(
    out: &mut [AccelerateComplex<AF>], input: &[AF]
) -> Result<(), AccelerateError> {
    check_lengths_1(out.len(), input.len())?;
    for (out_chunk, in_chunk) in out.chunks_mut(CHUNK).zip(input.chunks(CHUNK)) {
        let count = in_chunk.len() as i32;
        unsafe { AF::accelerate_cosisin(out_chunk.as_mut_ptr(), in_chunk.as_ptr(), &count); }
    }
    Ok(())
}
