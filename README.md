Safe no_std Rust bindings for the [VForce](https://developer.apple.com/documentation/accelerate/vforce-library?language=objc) family of hardware-accelerated vectorized math functions in the [Accelerate](https://developer.apple.com/documentation/accelerate?language=objc) framework on MacOS.

Provide a safe API for VForce functions generic over single and double precision floats, with automatic chunking for very large arrays. 

```rust
use vforce::arithmetic::{pow_array, pow_array_in_place};

let mut bases: Vec<f64> = vec![1.0, 6.0, 2.5];
let exponents: Vec<f64> = vec![2.0, 4.0, 1.3];
let mut out = vec![0.0f64; 3];

// results can be written out to another array:
pow_array(&mut out, &bases, &exponents).unwrap();
assert_eq!(out, vec![1.0f64, 1296.0f64, 2.5f64.powf(1.3)]);

// or overwrite one of the original arrays:
pow_array_in_place(&mut bases, &exponents).unwrap();
assert_eq!(bases, vec![1.0f64, 1296.0f64, 2.5f64.powf(1.3)]);
```

Either `f64` or `f32` may be used, but the type must be consistent for all arrays used for a given function call.

The VForce functions are hand-tuned implementations of transcendental vectorized array functions built with NEON and optimized for Apple hardware. VForce is part of the Apple Accelerate framework, which ships on all MacOS versions since 10.3 (October 2003) and many more Apple devices since then.

This will not compile for any OS other than MacOS, and linking will be invalid on platforms without Accelerate. Take care to put code that uses these functions behind conditional compilation flags.

The original VForce functions are indexed by `i32`, causing them to fail when processing arrays longer than `i32::MAX` = 2,147,483,647 elements long. This implementation checks for excessive array length and will instead process arrays in `i32::MAX`-size chunks sequentially should they be input.

Almost all functions provide an out-of-place variant and in-place variant, in order to allow safe overwriting without breaking alias XOR mutability.
