pub mod fns;

#[repr(C)]
pub struct AccelerateComplex<T: Copy> {
    real: T,
    imaginary: T,
}

impl<T: Copy> AccelerateComplex<T> {
    pub fn unpack(&self) -> (T, T) {
        (self.real, self.imaginary)
    }
}
