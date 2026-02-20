use super::*;

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
