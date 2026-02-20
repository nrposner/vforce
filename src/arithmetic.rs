//! Groups together arithmetic and basic numerical functions, including powers
use super::*;

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
