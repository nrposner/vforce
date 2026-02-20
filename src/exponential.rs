use super::*;

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
