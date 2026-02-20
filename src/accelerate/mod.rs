pub mod fns;

#[repr(C)]
/// Represents complex numbers using either single or double precision floats for the real and
/// imaginary components
pub struct AccelerateComplex<T: Copy> {
    real: T,
    imaginary: T,
}

impl<T: Copy> AccelerateComplex<T> {
    /// Unpacks the complex number into its real and imaginary components, in that order, both either single or
    /// double precision floats
    pub fn unpack(&self) -> (T, T) {
        (self.real, self.imaginary)
    }
}
