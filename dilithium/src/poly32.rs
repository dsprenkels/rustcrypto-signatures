use generic_array::GenericArray;

use crate::variant;

pub(crate) struct Poly32 {
    coeffs: GenericArray<i32, variant::N>,
}