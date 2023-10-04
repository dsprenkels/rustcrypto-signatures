use generic_array::GenericArray;

use crate::variant;

#[derive(Clone, Copy, Default)]
pub(crate) struct Poly32 {
    pub(crate) coeffs: GenericArray<i32, variant::N>,
}
