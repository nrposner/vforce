pub mod fns;

#[repr(C)]
pub struct ComplexDouble {
    real: f64,
    imaginary: f64,
}
#[repr(C)]
pub struct ComplexFloat {
    real: f32,
    imaginary: f32,
}
